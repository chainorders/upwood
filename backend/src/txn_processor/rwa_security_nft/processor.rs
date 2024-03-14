use super::db::{
    ContractConfig, DbToken, IRwaSecurityNftDb, TokenHolder, TokenHolderOperator,
    TokenHolderRecoveryRecord,
};
use crate::{
    shared::db::{DbAddress, DbContractAddress, DbTokenAmount, DbTokenId, ICollection},
    txn_listener::EventsProcessor,
};
use async_trait::async_trait;
use bson::{doc, to_document};
use concordium_cis2::{Cis2Event, OperatorUpdate};
use concordium_rust_sdk::{
    cis2::TokenAmount,
    types::{
        smart_contracts::{ContractEvent, ModuleReference, OwnedContractName},
        ContractAddress,
    },
};
use concordium_rwa_security_nft::event::Event;
use tokio::try_join;

pub struct RwaSecurityNftProcessor<TDb> {
    /// Client to interact with the MongoDB database.
    pub db:         TDb,
    /// Module reference of the contract.
    pub module_ref: ModuleReference,
}

#[async_trait]
impl<TDb: Send + Sync + IRwaSecurityNftDb> EventsProcessor for RwaSecurityNftProcessor<TDb> {
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

    /// Processes the events of the contract.
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
                Event::AgentAdded(e) => {
                    self.db.agents(contract).insert_one(DbAddress(e.agent)).await?;
                }
                Event::AgentRemoved(e) => {
                    self.db.agents(contract).delete_one(to_document(&DbAddress(e.agent))?).await?;
                }
                Event::ComplianceAdded(e) => {
                    self.db
                        .config(contract)
                        .upsert_one(doc! {}, |c| {
                            let contract = DbContractAddress(e.0);
                            match c {
                                None => ContractConfig {
                                    compliance:        Some(contract),
                                    identity_registry: None,
                                },
                                Some(mut c) => {
                                    c.compliance = Some(contract);
                                    c
                                }
                            }
                        })
                        .await?;
                }
                Event::IdentityRegistryAdded(e) => {
                    self.db
                        .config(contract)
                        .upsert_one(doc! {}, |c| {
                            let contract = DbContractAddress(e.0);
                            match c {
                                None => ContractConfig {
                                    compliance:        None,
                                    identity_registry: Some(contract),
                                },
                                Some(mut c) => {
                                    c.identity_registry = Some(contract);
                                    c
                                }
                            }
                        })
                        .await?;
                }
                Event::Paused(e) => {
                    let token_id = DbTokenId(e.token_id.to_string().parse()?);
                    self.db
                        .tokens(contract)
                        .upsert_one(DbToken::key(&token_id), |t| {
                            let mut token = match t {
                                None => DbToken::default(token_id.clone()),
                                Some(t) => t,
                            };
                            token.is_paused = true;
                            token
                        })
                        .await?;
                }
                Event::UnPaused(e) => {
                    let token_id = DbTokenId(e.token_id.to_string().parse()?);
                    self.db
                        .tokens(contract)
                        .upsert_one(DbToken::key(&token_id), |t| {
                            let mut token = match t {
                                None => DbToken::default(token_id.clone()),
                                Some(t) => t,
                            };
                            token.is_paused = false;
                            token
                        })
                        .await?;
                }
                Event::Recovered(e) => {
                    let replacement_update = self.db.replace_holder(
                        contract,
                        DbAddress(e.lost_account),
                        DbAddress(e.new_account),
                    );
                    let token_holder_recovery_records = self.db.recovery_records(contract);
                    try_join!(
                        replacement_update,
                        token_holder_recovery_records.insert_one(TokenHolderRecoveryRecord {
                            new_account:  DbAddress(e.new_account),
                            lost_account: DbAddress(e.lost_account),
                        })
                    )?;
                }
                Event::TokenFrozen(e) => {
                    let token_id = DbTokenId(e.token_id.to_string().parse()?);
                    let token_amount: TokenAmount = e.amount.0.into();
                    self.db
                        .holders(contract)
                        .upsert_one(TokenHolder::key(&token_id, &DbAddress(e.address)), |t| {
                            let mut token_holder = match t {
                                None => {
                                    TokenHolder::default(token_id.clone(), DbAddress(e.address))
                                }
                                Some(t) => t,
                            };
                            token_holder
                                .frozen_balance
                                .add_assign(DbTokenAmount(token_amount.clone()));
                            token_holder
                        })
                        .await?;
                }
                Event::TokenUnFrozen(e) => {
                    let token_id = DbTokenId(e.token_id.to_string().parse()?);
                    let token_amount: TokenAmount = e.amount.0.into();
                    self.db
                        .holders(contract)
                        .upsert_one(TokenHolder::key(&token_id, &DbAddress(e.address)), |t| {
                            let mut token_holder = match t {
                                None => {
                                    TokenHolder::default(token_id.clone(), DbAddress(e.address))
                                }
                                Some(t) => t,
                            };
                            token_holder
                                .frozen_balance
                                .sub_assign(DbTokenAmount(token_amount.clone()));
                            token_holder
                        })
                        .await?;
                }
                Event::Cis2(e) => match e {
                    Cis2Event::Mint(e) => {
                        let token_id = DbTokenId(e.token_id.to_string().parse()?);
                        let token_amount = DbTokenAmount(e.amount.0.into());
                        let token_holders = self.db.holders(contract);
                        let tokens = self.db.tokens(contract);
                        try_join!(
                            tokens.insert_one(DbToken {
                                supply: token_amount.clone(),
                                ..DbToken::default(token_id.clone())
                            }),
                            token_holders.insert_one(TokenHolder {
                                balance: token_amount.clone(),
                                ..TokenHolder::default(token_id, DbAddress(e.owner))
                            },)
                        )?;
                    }
                    Cis2Event::TokenMetadata(e) => {
                        let token_id = DbTokenId(e.token_id.to_string().parse()?);
                        self.db
                            .tokens(contract)
                            .upsert_one(DbToken::key(&token_id), |t| {
                                let mut token = match t {
                                    None => DbToken::default(token_id.clone()),
                                    Some(t) => t,
                                };
                                token.metadata_url = Some(e.metadata_url.url.to_owned());
                                token.metadata_url_hash = e.metadata_url.hash.map(hex::encode);
                                token
                            })
                            .await?;
                    }
                    Cis2Event::Transfer(e) => {
                        let token_id = DbTokenId(e.token_id.to_string().parse()?);
                        let token_amount = DbTokenAmount(e.amount.0.into());
                        let token_holders = self.db.holders(contract);
                        try_join!(
                            token_holders.upsert_one(
                                TokenHolder::key(&token_id, &DbAddress(e.from)),
                                |h| {
                                    let mut token_holder = match h {
                                        None => TokenHolder::default(
                                            token_id.clone(),
                                            DbAddress(e.from),
                                        ),
                                        Some(h) => h,
                                    };
                                    token_holder.balance.sub_assign(token_amount.clone());
                                    token_holder
                                }
                            ),
                            token_holders.upsert_one(
                                TokenHolder::key(&token_id, &DbAddress(e.to)),
                                |h| {
                                    let mut token_holder = match h {
                                        None => {
                                            TokenHolder::default(token_id.clone(), DbAddress(e.to))
                                        }
                                        Some(h) => h,
                                    };
                                    token_holder.balance.add_assign(token_amount.clone());
                                    token_holder
                                }
                            )
                        )?;
                    }
                    Cis2Event::Burn(e) => {
                        let token_id = DbTokenId(e.token_id.to_string().parse()?);
                        let token_amount = DbTokenAmount(e.amount.0.into());
                        let token_holders = self.db.holders(contract);
                        let tokens = self.db.tokens(contract);

                        try_join!(
                            tokens.upsert_one(DbToken::key(&token_id), |t| {
                                let mut token = match t {
                                    None => DbToken::default(token_id.clone()),
                                    Some(t) => t,
                                };
                                token.supply.sub_assign(token_amount.clone());
                                token
                            }),
                            token_holders.upsert_one(
                                TokenHolder::key(&token_id, &DbAddress(e.owner)),
                                |h| {
                                    let mut token_holder = match h {
                                        None => TokenHolder::default(
                                            token_id.clone(),
                                            DbAddress(e.owner),
                                        ),
                                        Some(h) => h,
                                    };
                                    token_holder.balance.sub_assign(token_amount.clone());
                                    token_holder
                                }
                            )
                        )?;
                    }
                    Cis2Event::UpdateOperator(e) => {
                        let owner = DbAddress(e.owner);
                        let operator = DbAddress(e.operator);

                        match e.update {
                            OperatorUpdate::Add => {
                                self.db
                                    .operators(contract)
                                    .upsert_one(TokenHolderOperator::key(&owner, &operator), |o| {
                                        match o {
                                            None => TokenHolderOperator::default(
                                                DbAddress(e.owner),
                                                DbAddress(e.operator),
                                            ),
                                            Some(o) => o,
                                        }
                                    })
                                    .await?
                            }
                            OperatorUpdate::Remove => {
                                self.db
                                    .operators(contract)
                                    .delete_one(TokenHolderOperator::key(&owner, &operator))
                                    .await?
                            }
                        };
                    }
                },
            }
        }

        Ok(())
    }
}
