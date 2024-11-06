//! This module contains the implementation of the RWA Identity Registry.
//! The `RwaIdentityRegistry` struct provides methods to interact with the RWA
//! Identity Registry contract. It interacts with the `IRwaIdentityRegistryDb`
//! trait to fetch data from the database. The API endpoints are defined using
//! the `poem_openapi` and `poem` crates, and the responses are serialized as
//! JSON using the `Json` type.
pub mod db;
pub mod processor;

use concordium_rust_sdk::base::hashes::ModuleReference;
use concordium_rust_sdk::base::smart_contracts::{OwnedContractName, WasmModule};
pub fn module_ref() -> ModuleReference {
    WasmModule::from_slice(include_bytes!(
        "../../../../../contracts/identity-registry/contract.wasm.v1"
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
mod integration_tests {
    use chrono::{DateTime, Utc};
    use concordium_rust_sdk::base::hashes::{BlockHash, TransactionHash};
    use concordium_rust_sdk::base::smart_contracts::{
        OwnedParameter, OwnedReceiveName, WasmVersion,
    };
    use concordium_rust_sdk::common::types::Amount;
    use concordium_rust_sdk::id::types::{AccountAddress, ACCOUNT_ADDRESS_SIZE};
    use concordium_rust_sdk::types::{
        AbsoluteBlockHeight, Address, ContractAddress, ContractInitializedEvent,
        InstanceUpdatedEvent, TransactionIndex,
    };
    use concordium_rwa_identity_registry::event::{
        AgentUpdatedEvent, Event, IdentityUpdatedEvent, IssuerUpdatedEvent,
    };
    use diesel::r2d2::ConnectionManager;
    use r2d2::Pool;
    use shared::db::DbPool;
    use shared_tests::{create_new_database_container, to_contract_event};

    use crate::db_setup;
    use crate::txn_listener::db::ProcessorType;
    use crate::txn_listener::listener::{
        process_block, ContractCall, ContractCallType, ParsedBlock, ParsedTxn, Processors,
    };
    use crate::txn_processor::cis2_utils::ContractAddressToDecimal;
    use crate::txn_processor::identity_registry::{self, db};

    #[tokio::test]
    async fn add_identity() {
        let contract_address = ContractAddress::new(0, 0);
        let block_2_time = DateTime::<Utc>::from_timestamp(1, 0).unwrap();
        let mut processors = Processors::new();
        processors.insert(
            identity_registry::module_ref(),
            identity_registry::contract_name(),
            ProcessorType::IdentityRegistry,
            identity_registry::processor::process_events,
        );

        let (database_url, _container) = create_new_database_container().await;
        db_setup::run_migrations(&database_url);
        let pool: DbPool = Pool::builder()
            .max_size(1)
            .build(ConnectionManager::new(database_url))
            .expect("Error creating database pool");

        {
            let mut conn = pool.get().expect("Error getting database connection");
            let block1: ParsedBlock = ParsedBlock {
                block_hash:      BlockHash::new([0; 32]),
                block_height:    AbsoluteBlockHeight { height: 0 },
                block_slot_time: DateTime::<Utc>::from_timestamp(0, 0).unwrap(),
                transactions:    vec![ParsedTxn {
                    hash:           TransactionHash::new([0; 32]),
                    index:          TransactionIndex { index: 0 },
                    sender:         AccountAddress([0; ACCOUNT_ADDRESS_SIZE]),
                    contract_calls: vec![ContractCall {
                        call_type: ContractCallType::create_init(ContractInitializedEvent {
                            address:          contract_address,
                            amount:           Amount::from_micro_ccd(0),
                            contract_version: WasmVersion::V1,
                            init_name:        identity_registry::contract_name(),
                            origin_ref:       identity_registry::module_ref(),
                            events:           vec![],
                        }),
                        contract:  contract_address,
                    }],
                }],
            };
            process_block(
                &mut conn,
                &block1,
                &AccountAddress([0; ACCOUNT_ADDRESS_SIZE]),
                &processors,
            )
            .await
            .expect("Error processing block 1");

            let block2 = ParsedBlock {
                block_hash:      BlockHash::new([1; 32]),
                block_height:    AbsoluteBlockHeight { height: 1 },
                block_slot_time: block_2_time,
                transactions:    vec![ParsedTxn {
                    hash:           TransactionHash::new([1; 32]),
                    index:          TransactionIndex { index: 0 },
                    sender:         AccountAddress([0; ACCOUNT_ADDRESS_SIZE]),
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
                            contract:  contract_address,
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
                            contract:  contract_address,
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
                            contract:  contract_address,
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
                            contract:  contract_address,
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
                            contract:  contract_address,
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
                            contract:  contract_address,
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
                            contract:  contract_address,
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
                            contract:  contract_address,
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
                            contract:  contract_address,
                        },
                    ],
                }],
            };
            process_block(
                &mut conn,
                &block2,
                &AccountAddress([0; ACCOUNT_ADDRESS_SIZE]),
                &processors,
            )
            .await
            .expect("Error processing block 2");
        }

        let mut conn = pool.get().expect("Error getting database connection");
        let contract_address = contract_address.to_decimal();
        let (identities, page_count) = db::Identity::list(&mut conn, contract_address, 10, 0)
            .expect("Error Listing Identitites");
        assert_eq!(identities, vec![db::Identity::new(
            &Address::Account(AccountAddress([1; ACCOUNT_ADDRESS_SIZE])),
            block_2_time,
            contract_address
        )]);
        assert_eq!(page_count, 1);

        let (agents, page_count) =
            db::Agent::list(&mut conn, contract_address, 10, 0).expect("Error Listing Agents");
        assert_eq!(agents, vec![db::Agent::new(
            Address::Account(AccountAddress([10; ACCOUNT_ADDRESS_SIZE])),
            block_2_time,
            contract_address
        )]);
        assert_eq!(page_count, 1);

        let (issuers, page_count) =
            db::Issuer::list(&mut conn, contract_address, 10, 0).expect("Error Listing Issuers");
        assert_eq!(issuers, vec![db::Issuer::new(
            ContractAddress {
                index:    1001,
                subindex: 0,
            }
            .to_decimal(),
            block_2_time,
            contract_address
        )]);
        assert_eq!(page_count, 1);
    }
}
