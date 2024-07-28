//! This module contains the implementation of the RWA Identity Registry.
//! The `RwaIdentityRegistry` struct provides methods to interact with the RWA
//! Identity Registry contract. It interacts with the `IRwaIdentityRegistryDb`
//! trait to fetch data from the database. The API endpoints are defined using
//! the `poem_openapi` and `poem` crates, and the responses are serialized as
//! JSON using the `Json` type.

pub mod db;
pub mod processor;

#[cfg(test)]
mod integration_tests {
    use anyhow::Ok;
    use chrono::{DateTime, Utc};
    use concordium_rust_sdk::{
        base::{contracts_common::Serial, smart_contracts::ContractEvent},
        id::types::{AccountAddress, ACCOUNT_ADDRESS_SIZE},
        types::{Address, ContractAddress},
    };
    use diesel::{r2d2::ConnectionManager, Connection, PgConnection};
    use itertools::Itertools;
    use r2d2::Pool;

    use crate::{
        db_setup,
        txn_processor::rwa_identity_registry::{db, processor},
    };
    use concordium_rwa_identity_registry::event::{
        AgentUpdatedEvent, Event, IdentityUpdatedEvent, IssuerUpdatedEvent,
    };

    const DATABASE_URL: &str =
        "postgres://concordium_rwa_dev_user:concordium_rwa_dev_pswd@localhost/concordium_rwa_dev";

    #[test]
    fn add_identity() {
        let now = DateTime::<Utc>::from_timestamp_millis(10).unwrap();
        let contract_address = ContractAddress::new(0, 0);
        let events = [
            Event::IdentityRegistered(IdentityUpdatedEvent {
                address: Address::Account(AccountAddress([0; ACCOUNT_ADDRESS_SIZE])),
            }),
            Event::IdentityRegistered(IdentityUpdatedEvent {
                address: Address::Account(AccountAddress([1; ACCOUNT_ADDRESS_SIZE])),
            }),
            Event::IdentityRemoved(IdentityUpdatedEvent {
                address: Address::Account(AccountAddress([0; ACCOUNT_ADDRESS_SIZE])),
            }),
            Event::AgentAdded(AgentUpdatedEvent {
                agent: Address::Account(AccountAddress([10; ACCOUNT_ADDRESS_SIZE])),
            }),
            Event::AgentAdded(AgentUpdatedEvent {
                agent: Address::Account(AccountAddress([11; ACCOUNT_ADDRESS_SIZE])),
            }),
            Event::AgentRemoved(AgentUpdatedEvent {
                agent: Address::Account(AccountAddress([11; ACCOUNT_ADDRESS_SIZE])),
            }),
            Event::IssuerAdded(IssuerUpdatedEvent {
                issuer: ContractAddress {
                    index:    1000,
                    subindex: 0,
                },
            }),
            Event::IssuerAdded(IssuerUpdatedEvent {
                issuer: ContractAddress {
                    index:    1001,
                    subindex: 0,
                },
            }),
            Event::IssuerRemoved(IssuerUpdatedEvent {
                issuer: ContractAddress {
                    index:    1000,
                    subindex: 0,
                },
            }),
        ];
        let events: Vec<ContractEvent> = events.iter().map(to_contract_event).collect_vec();
        let manager = ConnectionManager::<PgConnection>::new(DATABASE_URL);
        let pool =
            Pool::builder().max_size(1).build(manager).expect("Error creating database pool");
        let mut conn = pool.get().expect("Error getting database connection");

        conn.test_transaction(|conn| {
            //setup
            db_setup::run_migrations_on_conn(conn)?;

            //execution
            processor::process_events(conn, now, &contract_address, &events)
                .expect("Error inserting events for first contract");
            processor::process_events(conn, now, &ContractAddress::new(1, 0), &events)
                .expect("Error inserting events for second contract");

            //assertion
            let (identities, page_count) = db::list_identities(conn, &contract_address, 10, 0)
                .expect("Error Listing Identitites");
            assert_eq!(identities, vec![db::Identity::new(
                &Address::Account(AccountAddress([1; ACCOUNT_ADDRESS_SIZE])),
                now,
                &contract_address
            )]);
            assert_eq!(page_count, 1);

            let (agents, page_count) =
                db::list_agents(conn, &contract_address, 10, 0).expect("Error Listing Agents");
            assert_eq!(agents, vec![db::Agent::new(
                Address::Account(AccountAddress([10; ACCOUNT_ADDRESS_SIZE])),
                now,
                &contract_address
            )]);
            assert_eq!(page_count, 1);

            let (issuers, page_count) =
                db::list_issuers(conn, &contract_address, 10, 0).expect("Error Listing Agents");
            assert_eq!(issuers, vec![db::Issuer::new(
                &ContractAddress {
                    index:    1001,
                    subindex: 0,
                },
                now,
                &contract_address
            )]);
            assert_eq!(page_count, 1);
            Ok(())
        });
    }

    fn to_contract_event(e: &Event) -> ContractEvent {
        let mut out: Vec<u8> = Vec::new();
        e.serial(&mut out).expect("Error serializing event");
        out.into()
    }
}
