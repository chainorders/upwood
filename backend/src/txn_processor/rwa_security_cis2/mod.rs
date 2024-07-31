pub mod api;
pub mod db;
pub mod processor;

#[cfg(test)]
mod test {
    use chrono::Utc;
    use concordium_cis2::{TokenAmountU64, TokenIdU64, TokenIdU8};
    use concordium_rust_sdk::{
        base::smart_contracts::ContractEvent,
        common::types::Timestamp,
        id::types::{AccountAddress, ACCOUNT_ADDRESS_SIZE},
        types::ContractAddress,
    };
    use concordium_rwa_security_nft::types::Event;
    use concordium_rwa_security_sft::types::NftTokenId;
    use concordium_rwa_utils::{
        cis2_types::NftTokenAmount,
        concordium_cis2_security::{TokenDeposited, TokenUId},
    };
    use diesel::{r2d2::ConnectionManager, PgConnection};
    use poem::web::Data;
    use poem_openapi::{param::Path, payload::Json};
    use r2d2::Pool;

    use crate::{
        db_setup::{self, MigrationsConfig},
        shared::{
            api::PagedResponse,
            test::{to_contract_event, to_token_id_vec_u64, to_token_id_vec_u8, TestDbContext},
        },
        txn_processor::rwa_security_cis2::api::Cis2Deposit,
    };

    use super::{api::Cis2Api, processor::process_events};
    const DB_SERVER_URL: &str =
        "postgres://concordium_rwa_dev_user:concordium_rwa_dev_pswd@localhost";
    const DB_NAME: &str = "concordium_rwa_dev_test";

    #[tokio::test]
    async fn process_deposits_and_withdrawls() {
        // Setup
        let now = Timestamp {
            millis: Utc::now().timestamp_millis() as u64,
        };
        let test_db_context = TestDbContext::new(DB_SERVER_URL, DB_NAME);
        db_setup::run_migrations(&MigrationsConfig {
            database_url: format!("{}/{}", test_db_context.base_url, test_db_context.db_name),
        })
        .expect("error running migrations");

        let cis2_address_1 = ContractAddress::new(1, 1);
        let cis2_address_2 = ContractAddress::new(2, 1);
        let owner_1 = AccountAddress([0; ACCOUNT_ADDRESS_SIZE]);
        let token_id_1 = to_token_id_vec_u8(TokenIdU8(1));
        let token_id_2 = to_token_id_vec_u64(TokenIdU64(2));
        let owner_2 = AccountAddress([1; ACCOUNT_ADDRESS_SIZE]);
        let events: Vec<ContractEvent> = [
            Event::Deposited(TokenDeposited {
                amount:   TokenAmountU64(1000),
                owner:    owner_1,
                token_id: TokenUId {
                    contract: ContractAddress::new(100, 0),
                    id:       token_id_1.clone(),
                },
            }),
            Event::Deposited(TokenDeposited {
                amount:   TokenAmountU64(1000),
                owner:    owner_1,
                token_id: TokenUId {
                    contract: ContractAddress::new(100, 0),
                    id:       token_id_1.clone(),
                },
            }),
            Event::Withdraw(TokenDeposited {
                amount:   TokenAmountU64(100),
                owner:    owner_1,
                token_id: TokenUId {
                    contract: ContractAddress::new(100, 0),
                    id:       token_id_1.clone(),
                },
            }),
            Event::Deposited(TokenDeposited {
                amount:   TokenAmountU64(1000),
                owner:    owner_2,
                token_id: TokenUId {
                    contract: ContractAddress::new(100, 0),
                    id:       token_id_1.clone(),
                },
            }),
            Event::Deposited(TokenDeposited {
                amount:   TokenAmountU64(1000),
                owner:    owner_2,
                token_id: TokenUId {
                    contract: ContractAddress::new(101, 0),
                    id:       token_id_2.clone(),
                },
            }),
        ]
        .iter()
        .map(to_contract_event)
        .collect();

        let database_url = format!("{}/{}", test_db_context.base_url, test_db_context.db_name);
        {
            let mut db_conn = Pool::builder()
                .max_size(1)
                .build(ConnectionManager::<PgConnection>::new(database_url.clone()))
                .expect("Error creating database pool")
                .get()
                .expect("error getting connection from db");
            process_events::<NftTokenId, NftTokenAmount>(
                &mut db_conn,
                now,
                &cis2_address_1,
                &events,
            )
            .expect("Error processing events");
            process_events::<NftTokenId, NftTokenAmount>(
                &mut db_conn,
                now,
                &cis2_address_2,
                &events,
            )
            .expect("Error processing events");
        }

        let pool = Pool::builder()
            .max_size(1)
            .build(ConnectionManager::<PgConnection>::new(database_url.clone()))
            .expect("Error creating database pool");
        let Json(PagedResponse {
            page_count,
            page,
            data: tokens,
        }) = Cis2Api
            .list_deposited(
                Data(&pool),
                Path(cis2_address_1.index),
                Path(cis2_address_1.subindex),
                Path(owner_1.to_string()),
                Path(0),
            )
            .await
            .expect("error calling api method");
        assert_eq!(page_count, 1);
        assert_eq!(page, 0);
        assert_eq!(tokens, vec![Cis2Deposit {
            deposited_amount:       1900.to_string(),
            holder_address:         owner_1.to_string(),
            token_id:               token_id_1.to_string(),
            deposited_cis2_address: ContractAddress::new(100, 0).to_string(),
        }]);

        let Json(PagedResponse {
            page_count,
            page,
            data: tokens,
        }) = Cis2Api
            .list_deposited(
                Data(&pool),
                Path(cis2_address_1.index),
                Path(cis2_address_1.subindex),
                Path(owner_2.to_string()),
                Path(0),
            )
            .await
            .expect("error calling api method");
        assert_eq!(page_count, 1);
        assert_eq!(page, 0);
        assert_eq!(tokens, vec![
            Cis2Deposit {
                deposited_amount:       1000.to_string(),
                holder_address:         owner_2.to_string(),
                token_id:               token_id_1.to_string(),
                deposited_cis2_address: ContractAddress::new(100, 0).to_string(),
            },
            Cis2Deposit {
                deposited_amount:       1000.to_string(),
                holder_address:         owner_2.to_string(),
                token_id:               token_id_2.to_string(),
                deposited_cis2_address: ContractAddress::new(101, 0).to_string(),
            }
        ]);
    }

    //todo add tests for other remaining events
}
