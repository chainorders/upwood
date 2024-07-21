use super::db::{DbDepositedToken, RwaMarketDb};
use crate::{
    shared::db::{DbAccountAddress, DbContractAddress, DbTokenAmount, DbTokenId, ICollection},
    txn_listener::EventsProcessor,
};
use async_trait::async_trait;
use concordium_rust_sdk::types::{
    smart_contracts::{ContractEvent, ModuleReference, OwnedContractName},
    ContractAddress,
};
use concordium_rwa_market::event::{Event, PaymentAmount, PaymentTokenUId};

pub struct RwaMarketProcessor {
    /// Client to interact with the MongoDB database.
    pub client:        mongodb::Client,
    /// Module reference of the contract.
    pub module_ref:    ModuleReference,
    /// Name of the contract.
    pub contract_name: OwnedContractName,
}

impl RwaMarketProcessor {
    pub fn database(&self, contract: &ContractAddress) -> RwaMarketDb {
        let db = self
            .client
            .database(&format!("{}-{}-{}", self.contract_name, contract.index, contract.subindex));

        RwaMarketDb::init(db)
    }
}

#[async_trait]
impl EventsProcessor for RwaMarketProcessor {
    /// Returns the name of the contract this processor is responsible for.
    ///
    /// # Returns
    ///
    /// * A reference to the `OwnedContractName` of the contract.
    fn contract_name(&self) -> &OwnedContractName { &self.contract_name }

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
        &mut self,
        contract: &ContractAddress,
        events: &[ContractEvent],
    ) -> anyhow::Result<u64> {
        let mut process_events_count = 0u64;
        let mut db = self.database(contract);
        for event in events {
            let parsed_event = event.parse::<Event>()?;
            log::debug!("Event: {}/{} {:?}", contract.index, contract.subindex, parsed_event);

            match parsed_event {
                Event::Deposited(e) => {
                    let token_contract = DbContractAddress(e.token_id.contract);
                    let owner = DbAccountAddress(e.owner);
                    let token_id = DbTokenId(e.token_id.id.to_string().parse()?);
                    let token_amount = DbTokenAmount(e.amount.0.into());

                    db.deposited_tokens
                        .upsert_one(
                            DbDepositedToken::key(&token_contract, &token_id, &owner)?,
                            |t| {
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
                            },
                        )
                        .await?;
                    process_events_count += 1;
                }
                Event::Withdraw(e) => {
                    let token_contract = DbContractAddress(e.token_id.contract);
                    let owner = DbAccountAddress(e.owner);
                    let token_id = DbTokenId(e.token_id.id.to_string().parse()?);
                    let token_amount = DbTokenAmount(e.amount.0.into());

                    db.deposited_tokens
                        .upsert_one(
                            DbDepositedToken::key(&token_contract, &token_id, &owner)?,
                            |t| {
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
                            },
                        )
                        .await?;
                    process_events_count += 1;
                }
                Event::Listed(e) => {
                    let token_contract = DbContractAddress(e.token_id.contract);
                    let owner = DbAccountAddress(e.owner);
                    let token_id = DbTokenId(e.token_id.id.to_string().parse()?);
                    let token_amount = DbTokenAmount(e.supply.0.into());
                    db.deposited_tokens
                        .upsert_one(
                            DbDepositedToken::key(&token_contract, &token_id, &owner)?,
                            |t| {
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
                            },
                        )
                        .await?;
                    process_events_count += 1;
                }
                Event::DeListed(e) => {
                    let token_contract = DbContractAddress(e.token_id.contract);
                    let owner = DbAccountAddress(e.owner);
                    let token_id = DbTokenId(e.token_id.id.to_string().parse()?);

                    db.deposited_tokens
                        .upsert_one(
                            DbDepositedToken::key(&token_contract, &token_id, &owner)?,
                            |t| {
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
                            },
                        )
                        .await?;
                    process_events_count += 1;
                }
                Event::Exchanged(e) => {
                    let buy_token_contract = DbContractAddress(e.buy_token_id.contract);
                    let buy_token_id = DbTokenId(e.buy_token_id.id.to_string().parse()?);
                    let bought_from = DbAccountAddress(e.buy_token_owner);
                    let buy_token_amount = DbTokenAmount(e.buy_amount.0.into());

                    db.deposited_tokens
                        .upsert_one(
                            DbDepositedToken::key(
                                &buy_token_contract,
                                &buy_token_id,
                                &bought_from,
                            )?,
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
                        db.deposited_tokens
                            .upsert_one(
                                DbDepositedToken::key(&token_contract, &token_id, &owner)?,
                                |t| {
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
                                },
                            )
                            .await?;
                    }
                    process_events_count += 1;
                }
            }
        }

        Ok(process_events_count)
    }
}
