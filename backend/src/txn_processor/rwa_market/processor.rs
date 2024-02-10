use async_trait::async_trait;
use bson::doc;
use concordium_rust_sdk::types::{
    smart_contracts::{ContractEvent, ModuleReference, OwnedContractName},
    ContractAddress,
};
use concordium_rwa_market::event::{Event, PaymentAmount, PaymentTokenUId};

use super::db::{DbDepositedToken, IContractDb};
use crate::{
    txn_listener::EventsProcessor,
    txn_processor::db::{
        DbAccountAddress, DbContractAddress, DbTokenAmount, DbTokenId, ICollection,
    },
};

pub struct Processor<TDb: IContractDb> {
    /// Client to interact with the MongoDB database.
    pub db:         TDb,
    /// Module reference of the contract.
    pub module_ref: ModuleReference,
}

#[async_trait]
impl<TDb: Sync + Send + IContractDb> EventsProcessor for Processor<TDb> {
    /// Returns the name of the contract this processor is responsible for.
    ///
    /// # Returns
    ///
    /// * A reference to the `OwnedContractName` of the contract.
    fn contract_name(&self) -> &OwnedContractName { self.db.contract_name() }

    /// Returns the module reference of the contract this processor is
    /// responsible for.
    ///
    /// # Returns
    ///
    /// * A reference to the `ModuleReference` of the contract.
    fn module_ref(&self) -> &ModuleReference { &self.module_ref }

    /// Processes the events of the rwa-identity-registry contract.
    ///
    /// # Arguments
    ///
    /// * `contract` - A reference to the `ContractAddress` of the contract
    ///   whose events are to be processed.
    /// * `events` - A slice of `ContractEvent`s to be processed.
    ///
    /// # Returns
    ///
    /// * A Result indicating the success or failure of the operation.
    async fn process_events(
        &self,
        contract: &ContractAddress,
        events: &[ContractEvent],
    ) -> anyhow::Result<()> {
        for event in events {
            let parsed_event = event.parse::<Event>()?;
            log::info!("Event: {:?}", parsed_event);

            match parsed_event {
                Event::Deposited(e) => {
                    let token_contract = DbContractAddress(e.token_id.contract);
                    let owner = DbAccountAddress(e.owner);
                    let token_id = DbTokenId(e.token_id.id.to_string().parse()?);
                    let token_amount = DbTokenAmount(e.amount.0.into());

                    self.db
                        .deposited_tokens(contract)
                        .upsert_one(DbDepositedToken::key(&token_contract, &token_id, &owner)?, |t| {
                            let mut t = match t {
                                Some(t) => t,
                                None => DbDepositedToken::default(
                                    token_contract.clone(),
                                    token_id.clone(),
                                    owner.clone(),
                                ),
                            };
                            t.deposited_amount.add_assign(token_amount.clone());
                            t.unlisted_amount.add_assign(token_amount.clone());
                            t
                        })
                        .await?
                }
                Event::Withdraw(e) => {
                    let token_contract = DbContractAddress(e.token_id.contract);
                    let owner = DbAccountAddress(e.owner);
                    let token_id = DbTokenId(e.token_id.id.to_string().parse()?);
                    let token_amount = DbTokenAmount(e.amount.0.into());

                    self.db
                        .deposited_tokens(contract)
                        .upsert_one(DbDepositedToken::key(&token_contract, &token_id, &owner)?, |t| {
                            let mut t = match t {
                                Some(t) => t,
                                None => DbDepositedToken::default(
                                    token_contract.clone(),
                                    token_id.clone(),
                                    owner.clone(),
                                ),
                            };
                            t.deposited_amount.sub_assign(token_amount.clone());
                            t.unlisted_amount.sub_assign(token_amount.clone());
                            t
                        })
                        .await?
                }
                Event::Listed(e) => {
                    let token_contract = DbContractAddress(e.token_id.contract);
                    let owner = DbAccountAddress(e.owner);
                    let token_id = DbTokenId(e.token_id.id.to_string().parse()?);
                    let token_amount = DbTokenAmount(e.supply.0.into());
                    self.db
                        .deposited_tokens(contract)
                        .upsert_one(DbDepositedToken::key(&token_contract, &token_id, &owner)?, |t| {
                            let mut t = match t {
                                Some(t) => t,
                                None => DbDepositedToken::default(
                                    token_contract.clone(),
                                    token_id.clone(),
                                    owner.clone(),
                                ),
                            };
                            t.listed_amount.add_assign(token_amount.clone());
                            t.unlisted_amount.sub_assign(token_amount.clone());
                            t
                        })
                        .await?;
                }
                Event::DeListed(e) => {
                    let token_contract = DbContractAddress(e.token_id.contract);
                    let owner = DbAccountAddress(e.owner);
                    let token_id = DbTokenId(e.token_id.id.to_string().parse()?);

                    self.db
                        .deposited_tokens(contract)
                        .upsert_one(DbDepositedToken::key(&token_contract, &token_id, &owner)?, |t| {
                            let mut t = match t {
                                Some(t) => t,
                                None => DbDepositedToken::default(
                                    token_contract.clone(),
                                    token_id.clone(),
                                    owner.clone(),
                                ),
                            };
                            t.unlisted_amount.add_assign(t.listed_amount.clone());
                            t.listed_amount = DbTokenAmount::zero();
                            t
                        })
                        .await?;
                }
                Event::Exchanged(e) => {
                    let buy_token_contract = DbContractAddress(e.buy_token_id.contract);
                    let buy_token_id = DbTokenId(e.buy_token_id.id.to_string().parse()?);
                    let bought_from = DbAccountAddress(e.buy_token_owner);
                    let buy_token_amount = DbTokenAmount(e.buy_amount.0.into());

                    self.db
                        .deposited_tokens(contract)
                        .upsert_one(
                            DbDepositedToken::key(&buy_token_contract, &buy_token_id, &bought_from)?,
                            |t| {
                                let mut t = match t {
                                    Some(t) => t,
                                    None => DbDepositedToken::default(
                                        buy_token_contract.clone(),
                                        buy_token_id.clone(),
                                        bought_from.clone(),
                                    ),
                                };
                                t.listed_amount.sub_assign(buy_token_amount.clone());
                                t
                            },
                        )
                        .await?;

                    if let (PaymentTokenUId::Cis2(token_id), PaymentAmount::Cis2(amount)) =
                        (e.pay_token_id, e.pay_amount)
                    {
                        let token_contract = DbContractAddress(token_id.contract);
                        let token_id = DbTokenId(token_id.id.to_string().parse()?);
                        let owner = DbAccountAddress(e.pay_token_owner);
                        let token_amount = DbTokenAmount(amount.0.into());
                        self.db
                            .deposited_tokens(contract)
                            .upsert_one(DbDepositedToken::key(&token_contract, &token_id, &owner)?, |t| {
                                let mut t = match t {
                                    Some(t) => t,
                                    None => DbDepositedToken::default(
                                        token_contract.clone(),
                                        token_id.clone(),
                                        owner.clone(),
                                    ),
                                };
                                t.deposited_amount.sub_assign(token_amount.clone());
                                t.unlisted_amount.sub_assign(token_amount.clone());
                                t
                            })
                            .await?;
                    }
                }
            }
        }

        Ok(())
    }
}
