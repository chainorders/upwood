use chrono::NaiveDateTime;
use concordium_rust_sdk::types::smart_contracts::ContractEvent;
use concordium_rust_sdk::types::ContractAddress;
use concordium_rwa_identity_registry::event::Event;
use shared::db::identity_registry::{Agent, Identity, Issuer};
use shared::db_shared::DbConn;
use tracing::{info, instrument, trace};

use crate::processors::cis2_utils::ContractAddressToDecimal;
use crate::processors::ProcessorError;

/// Processes the events of the rwa-identity-registry contract.
///
/// # Arguments
///
/// * `conn` - A mutable reference to the `DbConn` connection.
/// * `now` - The current time as a `DateTime<Utc>`.
/// * `contract` - A reference to the `ContractAddress` of the contract whose
///   events are to be processed.
/// * `events` - A slice of `ContractEvent`s to be processed.
///
/// # Returns
///
/// * A `Result` indicating the success or failure of the operation.
#[instrument(
    name="identity_registry",
    skip_all,
    fields(contract = %contract, events = events.len())
)]
pub fn process_events(
    conn: &mut DbConn,
    now: NaiveDateTime,
    contract: &ContractAddress,
    events: &[ContractEvent],
) -> Result<(), ProcessorError> {
    for event in events {
        let parsed_event = event.parse::<Event>().expect("Failed to parse event");
        trace!("Event details: {:#?}", parsed_event);

        match parsed_event {
            Event::AgentAdded(e) => {
                Agent::new(e.agent, now, contract.to_decimal()).insert(conn)?;
                info!("Agent: {} added", e.agent.to_string());
            }
            Event::AgentRemoved(e) => {
                Agent::delete(conn, contract.to_decimal(), &e.agent)?;
                info!("Agent: {} removed", e.agent.to_string());
            }
            Event::IdentityRegistered(e) => {
                Identity::new(&e.address, now, contract.to_decimal()).insert(conn)?;
                info!("Identity: {} registered", e.address.to_string());
            }
            Event::IdentityRemoved(e) => {
                Identity::delete(conn, contract.to_decimal(), &e.address)?;
                info!("Identity: {} removed", e.address.to_string());
            }
            Event::IssuerAdded(e) => {
                Issuer::new(e.issuer.to_decimal(), now, contract.to_decimal()).insert(conn)?;
                info!("Issuer: {} added", e.issuer.to_string());
            }
            Event::IssuerRemoved(e) => {
                Issuer::delete(conn, contract.to_decimal(), e.issuer.to_decimal())?;
                info!("Issuer: {} removed", e.issuer.to_string());
            }
        }
    }

    Ok(())
}

use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{OwnedContractName, WasmModule};
pub fn module_ref() -> ModuleReference {
    WasmModule::from_slice(include_bytes!(
        "../../../../contracts/identity-registry/contract.wasm.v1"
    ))
    .expect("Failed to parse identity-registry module")
    .get_module_ref()
}

pub fn contract_name() -> OwnedContractName {
    OwnedContractName::new_unchecked("init_rwa_identity_registry".to_string())
}

// todo add api module exposing open api
// todo update integration tests using the api
#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use concordium_rust_sdk::base::smart_contracts::{
        OwnedParameter, OwnedReceiveName, WasmVersion,
    };
    use concordium_rust_sdk::common::types::Amount;
    use concordium_rust_sdk::id::types::{AccountAddress, ACCOUNT_ADDRESS_SIZE};
    use concordium_rust_sdk::types::{
        Address, ContractAddress, ContractInitializedEvent, InstanceUpdatedEvent,
    };
    use concordium_rwa_identity_registry::event::{
        AgentUpdatedEvent, Event, IdentityUpdatedEvent, IssuerUpdatedEvent,
    };
    use diesel::r2d2::ConnectionManager;
    use r2d2::Pool;
    use shared::db::identity_registry::{Agent, Identity, Issuer};
    use shared::db::txn_listener::ListenerBlock;
    use shared::db_setup;
    use shared::db_shared::DbPool;
    use shared_tests::{create_new_database_container, to_contract_event};

    use crate::listener::{ContractCall, ContractCallType, ParsedBlock, ParsedTxn};
    use crate::processors::cis2_utils::ContractAddressToDecimal;
    use crate::processors::Processors;

    #[tokio::test]
    async fn add_identity() {
        let contract_address = ContractAddress::new(0, 0);
        let block_2_time = DateTime::<Utc>::from_timestamp(1, 0).unwrap();
        let chain_admin = AccountAddress([0; ACCOUNT_ADDRESS_SIZE]);
        let mut processors = Processors::new(vec![chain_admin.to_string()]);

        let (db_config, _container) = create_new_database_container().await;
        db_setup::run_migrations(&db_config.db_url());
        let pool: DbPool = Pool::builder()
            .max_size(1)
            .build(ConnectionManager::new(db_config.db_url()))
            .expect("Error creating database pool");

        {
            let mut conn = pool.get().expect("Error getting database connection");
            let block1 = ParsedBlock {
                block:        ListenerBlock {
                    block_hash:      [0; 32].to_vec(),
                    block_height:    0.into(),
                    block_slot_time: DateTime::from_timestamp(0, 0).unwrap().naive_utc(),
                },
                transactions: vec![ParsedTxn {
                    index:          0,
                    hash:           [0; 32].to_vec(),
                    sender:         AccountAddress([0; ACCOUNT_ADDRESS_SIZE]).to_string(),
                    contract_calls: vec![ContractCall {
                        call_type: ContractCallType::create_init(ContractInitializedEvent {
                            address:          contract_address,
                            amount:           Amount::from_micro_ccd(0),
                            contract_version: WasmVersion::V1,
                            init_name:        super::contract_name(),
                            origin_ref:       super::module_ref(),
                            events:           vec![],
                        }),
                        contract:  contract_address.to_decimal(),
                    }],
                }],
            };
            processors
                .process_block(&mut conn, &block1)
                .await
                .expect("Error processing block 1");

            let block2 = ParsedBlock {
                block:        ListenerBlock {
                    block_hash:      [1; 32].to_vec(),
                    block_height:    1.into(),
                    block_slot_time: block_2_time.naive_utc(),
                },
                transactions: vec![ParsedTxn {
                    index:          0,
                    hash:           [1; 32].to_vec(),
                    sender:         AccountAddress([0; ACCOUNT_ADDRESS_SIZE]).to_string(),
                    contract_calls: vec![
                        ContractCall {
                            call_type: ContractCallType::create_update(
                                InstanceUpdatedEvent {
                                    address:          contract_address,
                                    amount:           Amount::from_micro_ccd(0),
                                    contract_version: WasmVersion::V1,
                                    instigator:       AccountAddress([0; ACCOUNT_ADDRESS_SIZE])
                                        .into(),
                                    message:          OwnedParameter::new_unchecked(vec![]),
                                    receive_name:     OwnedReceiveName::new_unchecked(
                                        "add_identity".to_string(),
                                    ),
                                    events:           vec![to_contract_event(
                                        &Event::IdentityRegistered(IdentityUpdatedEvent {
                                            address: AccountAddress([0; ACCOUNT_ADDRESS_SIZE])
                                                .into(),
                                        }),
                                    )],
                                },
                                None,
                            ),
                            contract:  contract_address.to_decimal(),
                        },
                        ContractCall {
                            call_type: ContractCallType::create_update(
                                InstanceUpdatedEvent {
                                    address:          contract_address,
                                    amount:           Amount::from_micro_ccd(0),
                                    contract_version: WasmVersion::V1,
                                    instigator:       AccountAddress([0; ACCOUNT_ADDRESS_SIZE])
                                        .into(),
                                    message:          OwnedParameter::new_unchecked(vec![]),
                                    receive_name:     OwnedReceiveName::new_unchecked(
                                        "add_identity".to_string(),
                                    ),
                                    events:           vec![to_contract_event(
                                        &Event::IdentityRegistered(IdentityUpdatedEvent {
                                            address: AccountAddress([1; ACCOUNT_ADDRESS_SIZE])
                                                .into(),
                                        }),
                                    )],
                                },
                                None,
                            ),
                            contract:  contract_address.to_decimal(),
                        },
                        ContractCall {
                            call_type: ContractCallType::create_update(
                                InstanceUpdatedEvent {
                                    address:          contract_address,
                                    amount:           Amount::from_micro_ccd(0),
                                    contract_version: WasmVersion::V1,
                                    instigator:       AccountAddress([0; ACCOUNT_ADDRESS_SIZE])
                                        .into(),
                                    message:          OwnedParameter::new_unchecked(vec![]),
                                    receive_name:     OwnedReceiveName::new_unchecked(
                                        "remove_identity".to_string(),
                                    ),
                                    events:           vec![to_contract_event(
                                        &Event::IdentityRemoved(IdentityUpdatedEvent {
                                            address: AccountAddress([0; ACCOUNT_ADDRESS_SIZE])
                                                .into(),
                                        }),
                                    )],
                                },
                                None,
                            ),
                            contract:  contract_address.to_decimal(),
                        },
                        ContractCall {
                            call_type: ContractCallType::create_update(
                                InstanceUpdatedEvent {
                                    address:          contract_address,
                                    amount:           Amount::from_micro_ccd(0),
                                    contract_version: WasmVersion::V1,
                                    instigator:       AccountAddress([0; ACCOUNT_ADDRESS_SIZE])
                                        .into(),
                                    message:          OwnedParameter::new_unchecked(vec![]),
                                    receive_name:     OwnedReceiveName::new_unchecked(
                                        "add_agent".to_string(),
                                    ),
                                    events:           vec![to_contract_event(&Event::AgentAdded(
                                        AgentUpdatedEvent {
                                            agent: AccountAddress([10; ACCOUNT_ADDRESS_SIZE])
                                                .into(),
                                        },
                                    ))],
                                },
                                None,
                            ),
                            contract:  contract_address.to_decimal(),
                        },
                        ContractCall {
                            call_type: ContractCallType::create_update(
                                InstanceUpdatedEvent {
                                    address:          contract_address,
                                    amount:           Amount::from_micro_ccd(0),
                                    contract_version: WasmVersion::V1,
                                    instigator:       AccountAddress([0; ACCOUNT_ADDRESS_SIZE])
                                        .into(),
                                    message:          OwnedParameter::new_unchecked(vec![]),
                                    receive_name:     OwnedReceiveName::new_unchecked(
                                        "add_agent".to_string(),
                                    ),
                                    events:           vec![to_contract_event(&Event::AgentAdded(
                                        AgentUpdatedEvent {
                                            agent: AccountAddress([11; ACCOUNT_ADDRESS_SIZE])
                                                .into(),
                                        },
                                    ))],
                                },
                                None,
                            ),
                            contract:  contract_address.to_decimal(),
                        },
                        ContractCall {
                            call_type: ContractCallType::create_update(
                                InstanceUpdatedEvent {
                                    address:          contract_address,
                                    amount:           Amount::from_micro_ccd(0),
                                    contract_version: WasmVersion::V1,
                                    instigator:       AccountAddress([0; ACCOUNT_ADDRESS_SIZE])
                                        .into(),
                                    message:          OwnedParameter::new_unchecked(vec![]),
                                    receive_name:     OwnedReceiveName::new_unchecked(
                                        "remove_agent".to_string(),
                                    ),
                                    events:           vec![to_contract_event(
                                        &Event::AgentRemoved(AgentUpdatedEvent {
                                            agent: AccountAddress([11; ACCOUNT_ADDRESS_SIZE])
                                                .into(),
                                        }),
                                    )],
                                },
                                None,
                            ),
                            contract:  contract_address.to_decimal(),
                        },
                        ContractCall {
                            call_type: ContractCallType::create_update(
                                InstanceUpdatedEvent {
                                    address:          contract_address,
                                    amount:           Amount::from_micro_ccd(0),
                                    contract_version: WasmVersion::V1,
                                    instigator:       AccountAddress([0; ACCOUNT_ADDRESS_SIZE])
                                        .into(),
                                    message:          OwnedParameter::new_unchecked(vec![]),
                                    receive_name:     OwnedReceiveName::new_unchecked(
                                        "add_issuer".to_string(),
                                    ),
                                    events:           vec![to_contract_event(&Event::IssuerAdded(
                                        IssuerUpdatedEvent {
                                            issuer: ContractAddress {
                                                index:    1000,
                                                subindex: 0,
                                            },
                                        },
                                    ))],
                                },
                                None,
                            ),
                            contract:  contract_address.to_decimal(),
                        },
                        ContractCall {
                            call_type: ContractCallType::create_update(
                                InstanceUpdatedEvent {
                                    address:          contract_address,
                                    amount:           Amount::from_micro_ccd(0),
                                    contract_version: WasmVersion::V1,
                                    instigator:       AccountAddress([0; ACCOUNT_ADDRESS_SIZE])
                                        .into(),
                                    message:          OwnedParameter::new_unchecked(vec![]),
                                    receive_name:     OwnedReceiveName::new_unchecked(
                                        "add_issuer".to_string(),
                                    ),
                                    events:           vec![to_contract_event(&Event::IssuerAdded(
                                        IssuerUpdatedEvent {
                                            issuer: ContractAddress {
                                                index:    1001,
                                                subindex: 0,
                                            },
                                        },
                                    ))],
                                },
                                None,
                            ),
                            contract:  contract_address.to_decimal(),
                        },
                        ContractCall {
                            call_type: ContractCallType::create_update(
                                InstanceUpdatedEvent {
                                    address:          contract_address,
                                    amount:           Amount::from_micro_ccd(0),
                                    contract_version: WasmVersion::V1,
                                    instigator:       AccountAddress([0; ACCOUNT_ADDRESS_SIZE])
                                        .into(),
                                    message:          OwnedParameter::new_unchecked(vec![]),
                                    receive_name:     OwnedReceiveName::new_unchecked(
                                        "remove_issuer".to_string(),
                                    ),
                                    events:           vec![to_contract_event(
                                        &Event::IssuerRemoved(IssuerUpdatedEvent {
                                            issuer: ContractAddress {
                                                index:    1000,
                                                subindex: 0,
                                            },
                                        }),
                                    )],
                                },
                                None,
                            ),
                            contract:  contract_address.to_decimal(),
                        },
                    ],
                }],
            };
            processors
                .process_block(&mut conn, &block2)
                .await
                .expect("Error processing block 2");
        }

        let mut conn = pool.get().expect("Error getting database connection");
        let contract_address = contract_address.to_decimal();
        let (identities, page_count) =
            Identity::list(&mut conn, contract_address, 10, 0).expect("Error Listing Identitites");
        assert_eq!(identities, vec![Identity::new(
            &Address::Account(AccountAddress([1; ACCOUNT_ADDRESS_SIZE])),
            block_2_time.naive_utc(),
            contract_address
        )]);
        assert_eq!(page_count, 1);

        let (agents, page_count) =
            Agent::list(&mut conn, contract_address, 10, 0).expect("Error Listing Agents");
        assert_eq!(agents, vec![Agent::new(
            Address::Account(AccountAddress([10; ACCOUNT_ADDRESS_SIZE])),
            block_2_time.naive_utc(),
            contract_address
        )]);
        assert_eq!(page_count, 1);

        let (issuers, page_count) =
            Issuer::list(&mut conn, contract_address, 10, 0).expect("Error Listing Issuers");
        assert_eq!(issuers, vec![Issuer::new(
            ContractAddress {
                index:    1001,
                subindex: 0,
            }
            .to_decimal(),
            block_2_time.naive_utc(),
            contract_address
        )]);
        assert_eq!(page_count, 1);
    }
}
