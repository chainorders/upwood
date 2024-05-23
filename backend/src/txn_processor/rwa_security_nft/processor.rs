use super::db::{
    ContractConfig, DbToken, RwaSecurityNftDb, TokenHolder, TokenHolderOperator,
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

pub struct RwaSecurityNftProcessor {
    /// Client to interact with the MongoDB database.
    pub client:        mongodb::Client,
    /// Module reference of the contract.
    pub module_ref:    ModuleReference,
    /// Name of the contract.
    pub contract_name: OwnedContractName,
}

impl RwaSecurityNftProcessor {
    pub fn database(&self, contract: &ContractAddress) -> RwaSecurityNftDb {
        let db = self
            .client
            .database(&format!("{}-{}-{}", self.contract_name, contract.index, contract.subindex));

        RwaSecurityNftDb::init(db)
    }
}

#[async_trait]
impl EventsProcessor for RwaSecurityNftProcessor {
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
    ) -> anyhow::Result<u64> {
        let mut process_events_count = 0u64;
        let mut db = self.database(contract);

        let parsed_events =
            events.iter().map(|e| e.parse::<Event>()).collect::<Result<Vec<_>, _>>()?;
        for parsed_event in parsed_events {
            log::debug!("Event: {}/{} {:?}", contract.index, contract.subindex, parsed_event);

            match parsed_event {
                Event::AgentAdded(e) => {
                    db.agents.insert_one(DbAddress(e.agent)).await?;
                    process_events_count += 1;
                }
                Event::AgentRemoved(e) => {
                    db.agents.delete_one(to_document(&DbAddress(e.agent))?).await?;
                    process_events_count += 1;
                }
                Event::ComplianceAdded(e) => {
                    db.config
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
                    process_events_count += 1;
                }
                Event::IdentityRegistryAdded(e) => {
                    db.config
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
                    process_events_count += 1;
                }
                Event::Paused(e) => {
                    let token_id = DbTokenId(e.token_id.to_string().parse()?);
                    db.tokens
                        .upsert_one(DbToken::key(&token_id), |t| {
                            let mut token = match t {
                                None => DbToken::default(token_id.clone()),
                                Some(t) => t,
                            };
                            token.is_paused = true;
                            token
                        })
                        .await?;
                    process_events_count += 1;
                }
                Event::UnPaused(e) => {
                    let token_id = DbTokenId(e.token_id.to_string().parse()?);
                    db.tokens
                        .upsert_one(DbToken::key(&token_id), |t| {
                            let mut token = match t {
                                None => DbToken::default(token_id.clone()),
                                Some(t) => t,
                            };
                            token.is_paused = false;
                            token
                        })
                        .await?;
                    process_events_count += 1;
                }
                Event::Recovered(e) => {
                    db.replace_holder(DbAddress(e.lost_account), DbAddress(e.new_account)).await?;
                    db.recovery_records
                        .insert_one(TokenHolderRecoveryRecord {
                            new_account:  DbAddress(e.new_account),
                            lost_account: DbAddress(e.lost_account),
                        })
                        .await?;
                    process_events_count += 1;
                }
                Event::TokenFrozen(e) => {
                    let token_id = DbTokenId(e.token_id.to_string().parse()?);
                    let token_amount: TokenAmount = e.amount.0.into();
                    db.holders
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
                    process_events_count += 1;
                }
                Event::TokenUnFrozen(e) => {
                    let token_id = DbTokenId(e.token_id.to_string().parse()?);
                    let token_amount: TokenAmount = e.amount.0.into();
                    db.holders
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
                    process_events_count += 1;
                }
                Event::Cis2(e) => match e {
                    Cis2Event::Mint(e) => {
                        let token_id = DbTokenId(e.token_id.to_string().parse()?);
                        let token_amount = DbTokenAmount(e.amount.0.into());
                        try_join!(
                            db.tokens.upsert_one(DbToken::key(&token_id), |t| {
                                let mut token = match t {
                                    None => DbToken::default(token_id.clone()),
                                    Some(t) => t,
                                };
                                token.supply.add_assign(token_amount.clone());
                                token
                            }),
                            db.holders.upsert_one(
                                TokenHolder::key(&token_id, &DbAddress(e.owner)),
                                |h| {
                                    let mut token_holder = match h {
                                        None => TokenHolder::default(
                                            token_id.clone(),
                                            DbAddress(e.owner),
                                        ),
                                        Some(h) => h,
                                    };
                                    token_holder.balance.add_assign(token_amount.clone());
                                    token_holder
                                }
                            )
                        )?;
                        process_events_count += 1;
                    }
                    Cis2Event::TokenMetadata(e) => {
                        let token_id = DbTokenId(e.token_id.to_string().parse()?);
                        db.tokens
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
                        process_events_count += 1;
                    }
                    Cis2Event::Transfer(e) => {
                        let token_id = DbTokenId(e.token_id.to_string().parse()?);
                        let token_amount = DbTokenAmount(e.amount.0.into());
                        db.holders
                            .upsert_one(TokenHolder::key(&token_id, &DbAddress(e.from)), |h| {
                                let mut token_holder = match h {
                                    None => {
                                        TokenHolder::default(token_id.clone(), DbAddress(e.from))
                                    }
                                    Some(h) => h,
                                };
                                token_holder.balance.sub_assign(token_amount.clone());
                                token_holder
                            })
                            .await?;
                        db.holders
                            .upsert_one(TokenHolder::key(&token_id, &DbAddress(e.to)), |h| {
                                let mut token_holder = match h {
                                    None => TokenHolder::default(token_id.clone(), DbAddress(e.to)),
                                    Some(h) => h,
                                };
                                token_holder.balance.add_assign(token_amount.clone());
                                token_holder
                            })
                            .await?;
                        process_events_count += 1;
                    }
                    Cis2Event::Burn(e) => {
                        let token_id = DbTokenId(e.token_id.to_string().parse()?);
                        let token_amount = DbTokenAmount(e.amount.0.into());

                        try_join!(
                            db.tokens.upsert_one(DbToken::key(&token_id), |t| {
                                let mut token = match t {
                                    None => DbToken {
                                        supply: token_amount.clone(),
                                        ..DbToken::default(token_id.clone())
                                    },
                                    Some(t) => t,
                                };
                                token.supply.sub_assign(token_amount.clone());
                                token
                            }),
                            db.holders.upsert_one(
                                TokenHolder::key(&token_id, &DbAddress(e.owner)),
                                |h| {
                                    let mut token_holder = match h {
                                        None => TokenHolder {
                                            balance: token_amount.clone(),
                                            ..TokenHolder::default(
                                                token_id.clone(),
                                                DbAddress(e.owner),
                                            )
                                        },
                                        Some(h) => h,
                                    };
                                    token_holder.balance.sub_assign(token_amount.clone());
                                    token_holder
                                }
                            )
                        )?;
                        process_events_count += 1;
                    }
                    Cis2Event::UpdateOperator(e) => {
                        let owner = DbAddress(e.owner);
                        let operator = DbAddress(e.operator);

                        match e.update {
                            OperatorUpdate::Add => {
                                db.operators
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
                                db.operators
                                    .delete_one(TokenHolderOperator::key(&owner, &operator))
                                    .await?
                            }
                        };
                        process_events_count += 1;
                    }
                },
            }
        }

        Ok(process_events_count)
    }
}
