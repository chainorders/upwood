//! This is meant to be a complete test of the forest project.
//! It is meant to be run with `cargo test --release -- --test tests_forest_projects`
#![feature(assert_matches)]
mod test_utils;

use chrono::{DateTime, Utc};
use concordium_cis2::{
    AdditionalData, OperatorUpdate, Receiver, TokenAmountU64, TokenIdU64, TokenIdUnit, TokenIdVec,
    Transfer, TransferParams, UpdateOperator, UpdateOperatorParams,
};
use concordium_protocols::concordium_cis2_security::{
    AddTokenParams, AgentWithRoles, Identity, SecurityParams, TokenAmountSecurity, TokenUId,
};
use concordium_protocols::rate::Rate;
use concordium_rwa_identity_registry::types::{IdentityAttribute, RegisterIdentityParams};
use concordium_smart_contract_testing::{AccountAddress, Amount, ContractAddress, Energy};
use diesel::r2d2::ConnectionManager;
use events_listener::processors::cis2_utils::{ContractAddressToDecimal, TokenIdToDecimal};
use events_listener::processors::Processors;
use integration_tests::cis2_security::{Cis2Payloads, Cis2SecurityPayloads};
use integration_tests::compliance::{ComplianceTestClient, NationalitiesModuleTestClient};
use integration_tests::contract_base::ContractPayloads;
use integration_tests::euroe::{EuroETestClient, RoleTypes};
use integration_tests::identity_registry::{IdentityRegistryPayloads, IdentityRegistryTestClient};
use integration_tests::nft_multi_rewarded_client::NftMultiRewardedTestClient;
use integration_tests::offchain_rewards_client::OffchainRewardsTestClient;
use integration_tests::security_mint_fund_client::MintFundTestClient;
use integration_tests::security_p2p_trading_client::{
    P2PTradeTestClient, P2PTradingClientPayloads,
};
use integration_tests::security_sft_multi_client::SftMultiTestClient;
use integration_tests::security_sft_multi_yielder_client::SftMultiYielderTestClient;
use integration_tests::security_sft_single_client::SftSingleTestClient;
use nft_multi_rewarded::types::ContractMetadataUrl;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use security_mint_fund::types::{
    AddFundParams, TransferInvestParams, UpdateFundState, UpdateFundStateParams,
};
use security_p2p_trading::{AddMarketParams, ExchangeParams, Market, TransferMarket};
use security_sft_multi_yielder::{
    UpsertYieldParams, YieldCalculation, YieldParam, YieldParams, YieldState,
};
use shared::api::PagedResponse;
use shared::db_app::forest_project::{ForestProject, ForestProjectPrice, ForestProjectState};
use shared::db_app::forest_project_crypto::prelude::*;
use shared::db_app::portfolio::UserTransaction;
use shared::db_shared::{DbConn, DbPool};
use test_log::test;
use test_utils::test_api::ApiTestClient;
use test_utils::test_chain::{Account, Chain};
use test_utils::test_user::UserTestClient;
use test_utils::{create_login_admin_user, create_login_user, PASS_GENERATOR};
use upwood::api;
use upwood::api::forest_project::ForestProjectTokenContractAggApiModel;
use upwood::api::investment_portfolio::InvestmentPortfolioUserAggregate;
use upwood::utils::aws::cognito::UserPool;
use uuid::Uuid;

pub const CHAIN_ADMIN: AccountAddress = AccountAddress([0u8; 32]);
pub const APP_ADMIN: AccountAddress = AccountAddress([1u8; 32]);
pub const DEFAULT_ACCOUNT_BALANCE: Amount = Amount::from_ccd(1_000);
const COMPLIANT_NATIONALITIES: [&str; 3] = ["IN", "US", "GB"];
const CARBON_CREDITS_METADATA_URL: &str = "https://metadata.com/carbon_credits";
const TREE_SFT_METADATA_URL: &str = "https://metadata.com/tree_sft";

#[test(tokio::test)]
pub async fn test_forest_projects() {
    dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".env")).ok();
    dotenvy::from_filename(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".secure.env"))
        .ok();
    let test_id = format!("fpsu_{}", uuid::Uuid::new_v4());
    let start_time = DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z")
        .expect("Failed to parse start time")
        .with_timezone(&Utc);
    let mut chain = Chain::new(start_time);
    let admin = chain.create_account(APP_ADMIN, DEFAULT_ACCOUNT_BALANCE);
    let chain_deployer = chain.create_account(CHAIN_ADMIN, DEFAULT_ACCOUNT_BALANCE);
    deploy_modules(&mut chain, &chain_deployer);
    let euroe = admin
        .transact(|account| chain.init(account, EuroETestClient::init_payload(&())))
        .map(EuroETestClient)
        .expect("Failed to init euroe contract");

    // euro e contract setup
    {
        admin
            .transact(|account| {
                chain.update(
                    account,
                    euroe.grant_role_payload(&RoleTypes {
                        adminrole: admin.0.address.into(),
                        mintrole:  admin.0.address.into(),
                        burnrole:  admin.0.address.into(),
                        blockrole: admin.0.address.into(),
                        pauserole: admin.0.address.into(),
                    }),
                )
            })
            .expect("Failed to grant euroe roles");
        admin
            .transact(|account| {
                chain.update(
                    account,
                    euroe.mint_payload(&integration_tests::euroe::MintParams {
                        amount: TokenAmountU64(1_000_000_000),
                        owner:  admin.address().into(),
                    }),
                )
            })
            .expect("Failed to mint euroe for admin");
    }

    let (
        carbon_credits,
        tree_sft,
        tree_nft,
        mint_fund_contract,
        trading_contract,
        yielder_contract,
        identity_registry,
        compliance_contract,
        offchain_rewards,
    ) = deploy_upwood_contracts(&mut chain, &admin, euroe);

    let (db_config, _container) = shared_tests::create_new_database_container().await;
    shared::db_setup::run_migrations(&db_config.db_url());
    // Uncomment the following lines to run the tests on the local database container
    // let db_config = shared_tests::PostgresTestConfig {
    //     postgres_db:       "concordium_rwa_dev".to_string(),
    //     postgres_host:     "localhost".to_string(),
    //     postgres_password: "concordium_rwa_dev_pswd".to_string(),
    //     postgres_port:     5432,
    //     postgres_user:     "concordium_rwa_dev_user".to_string(),
    // };

    let db_url = db_config.db_url();
    let pool: DbPool = r2d2::Pool::builder()
        .build(ConnectionManager::new(&db_url))
        .expect("Error creating database pool");
    let mut db_conn = pool.get().expect("Error getting connection from pool");
    let mut processor = Processors::new(vec![APP_ADMIN.to_string()]);
    processor
        .process_block(&mut db_conn, &chain.produce_block())
        .await
        .expect("Error processing block");

    let api_config: api::Config = config::Config::builder()
        .add_source(config::Environment::default())
        .build()
        .expect("Failed to build config")
        .try_deserialize()
        .expect("Failed to deserialize config");
    let api_config = api::Config {
        postgres_db: db_config.postgres_db,
        postgres_host: db_config.postgres_host,
        postgres_password: db_config.postgres_password.into(),
        postgres_port: db_config.postgres_port,
        postgres_user: db_config.postgres_user,
        carbon_credit_contract_index: carbon_credits.0.to_decimal(),
        compliance_contract_index: compliance_contract.0.to_decimal(),
        euro_e_contract_index: euroe.0.to_decimal(),
        identity_registry_contract_index: identity_registry.0.to_decimal(),
        tree_ft_contract_index: tree_sft.0.to_decimal(),
        tree_nft_contract_index: tree_nft.0.to_decimal(),
        offchain_rewards_contract_index: offchain_rewards.0.to_decimal(),
        mint_funds_contract_index: mint_fund_contract.0.to_decimal(),
        trading_contract_index: trading_contract.0.to_decimal(),
        yielder_contract_index: yielder_contract.0.to_decimal(),
        ..api_config
    };
    let mut api = ApiTestClient::new(api_config.clone()).await;
    let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let mut user_pool = UserPool::new(
        &sdk_config,
        &api_config.aws_user_pool_id,
        &api_config.aws_user_pool_client_id,
        &api_config.aws_user_pool_region,
    )
    .await
    .expect("Failed to create user pool");

    let offchain_agent_account_address = api_config.offchain_rewards_agent_wallet().address;
    let offchain_agent_account_keys = api_config.offchain_rewards_agent_wallet().keys;
    let admin_password = PASS_GENERATOR.generate_one().unwrap();
    let email_admin = format!("admin_{}@yopmail.com", test_id);
    let (id_token_admin, admin_cognito) = create_login_admin_user(
        &mut user_pool,
        email_admin.as_str(),
        admin_password.as_str(),
        admin.address_str().as_str(),
    )
    .await;
    let cognito_user_id = admin_cognito
        .attributes()
        .iter()
        .find_map(|a| {
            if a.name == "sub" {
                Some(a.value.clone())
            } else {
                None
            }
        })
        .flatten()
        .expect("Failed to get cognito user id");
    let admin = UserTestClient {
        account_address: admin.address().to_string(),
        email:           email_admin.to_string(),
        id_token:        id_token_admin,
        id:              cognito_user_id,
        password:        admin_password,
    };

    {
        let metadata = TokenMetadata {
            contract_address: carbon_credits.0.to_decimal(),
            token_id:         TokenIdUnit().to_decimal(),
            symbol:           Some("CC".to_string()),
            decimals:         Some(0),
        };
        admin
            .call_api(|token| api.admin_create_token_metadata(token, &metadata))
            .await;
    }

    {
        let metadata = TokenMetadata {
            contract_address: tree_sft.0.to_decimal(),
            token_id:         TokenIdUnit().to_decimal(),
            symbol:           Some("TREES".to_string()),
            decimals:         Some(0),
        };
        admin
            .call_api(|token| api.admin_create_token_metadata(token, &metadata))
            .await;
    }
    {
        let metadata = TokenMetadata {
            contract_address: tree_nft.0.to_decimal(),
            token_id:         TokenIdUnit().to_decimal(),
            symbol:           Some("TREE".to_string()),
            decimals:         Some(0),
        };
        admin
            .call_api(|token| api.admin_create_token_metadata(token, &metadata))
            .await;
    }
    {
        let metadata = TokenMetadata {
            contract_address: euroe.0.to_decimal(),
            token_id:         TokenIdUnit().to_decimal(),
            symbol:           Some("EUROe".to_string()),
            decimals:         Some(6),
        };
        admin
            .call_api(|token| api.admin_create_token_metadata(token, &metadata))
            .await;
    }

    let user_1 = create_user(
        &test_id,
        &mut chain,
        &mut user_pool,
        &mut api,
        &admin,
        None,
        Decimal::from_f64(0.05).unwrap(), // charging 5% for user affiliations
        1,
    )
    .await;

    let user_2 = create_user(
        &test_id,
        &mut chain,
        &mut user_pool,
        &mut api,
        &admin,
        Some(user_1.account_address.clone()),
        0.into(),
        2,
    )
    .await;

    let (fp_1_contract, fp_1_pre_sale, fp_1) = create_forest_project(
        &mut chain,
        &mut api,
        &mut db_conn,
        &mut processor,
        &euroe,
        &admin,
        &identity_registry,
        &compliance_contract,
        &mint_fund_contract,
        &yielder_contract,
        &trading_contract,
        1,
        2.into(),
        28,
    )
    .await;
    let (.., fp_2) = create_forest_project(
        &mut chain,
        &mut api,
        &mut db_conn,
        &mut processor,
        &euroe,
        &admin,
        &identity_registry,
        &compliance_contract,
        &mint_fund_contract,
        &yielder_contract,
        &trading_contract,
        2,
        2.into(),
        18,
    )
    .await;
    let (.., fp_3) = create_forest_project(
        &mut chain,
        &mut api,
        &mut db_conn,
        &mut processor,
        &euroe,
        &admin,
        &identity_registry,
        &compliance_contract,
        &mint_fund_contract,
        &yielder_contract,
        &trading_contract,
        3,
        2.into(),
        11,
    )
    .await;
    let (.., fp_4) = create_forest_project(
        &mut chain,
        &mut api,
        &mut db_conn,
        &mut processor,
        &euroe,
        &admin,
        &identity_registry,
        &compliance_contract,
        &mint_fund_contract,
        &yielder_contract,
        &trading_contract,
        4,
        2.into(),
        10,
    )
    .await;

    let fp_1_token_1 = TokenIdU64(1);
    admin
        .transact(|sender| {
            chain.update(
                sender,
                fp_1_contract.add_token_payload(&AddTokenParams {
                    token_id:       fp_1_token_1,
                    token_metadata: ContractMetadataUrl {
                        url:  "https://metadata.com/fp_1".to_string(),
                        hash: None,
                    },
                }),
            )
        })
        .expect("Failed to add token to forest project");

    admin
        .transact(|sender| {
            chain.update(
                sender,
                identity_registry.register_identity_payload(&RegisterIdentityParams {
                    address:  admin.address().into(),
                    identity: Identity {
                        attributes:  vec![IdentityAttribute {
                            tag:   5,
                            value: COMPLIANT_NATIONALITIES[1].to_string(),
                        }],
                        credentials: vec![],
                    },
                }),
            )
        })
        .expect("Failed to register identity for admin");

    // asserting user forest project active list before listing
    {
        let projects = user_1
            .call_api(|token| {
                api.forest_project_list_by_state(token, ForestProjectState::Active, 0)
            })
            .await;
        assert_eq!(projects.data.len(), 0);
    }

    {
        let fp_1 = ForestProject {
            state: ForestProjectState::Active,
            ..fp_1
        };
        let fp_1 = admin
            .call_api(|token| api.admin_update_forest_project(token, &fp_1))
            .await;
        assert_eq!(fp_1.state, ForestProjectState::Active);

        let fp_2 = ForestProject {
            state: ForestProjectState::Active,
            ..fp_2
        };
        admin
            .call_api(|token| api.admin_update_forest_project(token, &fp_2))
            .await;

        let fp_3 = ForestProject {
            state: ForestProjectState::Active,
            ..fp_3
        };
        admin
            .call_api(|token| api.admin_update_forest_project(token, &fp_3))
            .await;

        let fp_4 = ForestProject {
            state: ForestProjectState::Active,
            ..fp_4
        };
        admin
            .call_api(|token| api.admin_update_forest_project(token, &fp_4))
            .await;
    }

    {
        admin
            .transact(|sender| {
                chain.update(
                    sender,
                    fp_1_pre_sale.add_token_payload(&AddTokenParams {
                        token_id:       fp_1_token_1,
                        token_metadata: ContractMetadataUrl {
                            url:  "https://metadata.com/fp_1".to_string(),
                            hash: None,
                        },
                    }),
                )
            })
            .expect("Failed to add token to forest project pre sale");
        admin
            .transact(|sender| {
                chain.update(
                    sender,
                    mint_fund_contract.add_fund_payload(&AddFundParams {
                        token:          TokenUId {
                            id:       fp_1_token_1,
                            contract: fp_1_pre_sale.0,
                        },
                        rate:           Rate::new(1, 1).unwrap(),
                        security_token: TokenUId {
                            contract: fp_1_contract.0,
                            id:       fp_1_token_1,
                        },
                    }),
                )
            })
            .expect("Failed to add fund to mint fund contract");
        processor
            .process_block(&mut db_conn, &chain.produce_block())
            .await
            .expect("Error processing block");

        let mut property_token_contract = admin
            .call_api(|token| {
                api.admin_forest_project_token_contract_find_by_type(
                    token,
                    fp_1.id,
                    SecurityTokenContractType::Property,
                )
            })
            .await;
        property_token_contract.fund_token_id = Some(fp_1_token_1.to_decimal());
        admin
            .call_api(|id_token| {
                api.forest_project_token_contract_update(id_token, &property_token_contract)
            })
            .await;
    }

    // asserting user forest project active list
    {
        let PagedResponse { data: projects, .. } = user_1
            .call_api(|token| {
                api.forest_project_list_by_state(token, ForestProjectState::Active, 0)
            })
            .await;
        assert_eq!(projects.len(), 4);
    }

    // mint fund 1 investments
    {
        // user 1 investments in mint fund 1
        {
            user_1
                .transact(|account| {
                    chain.update(
                        account,
                        euroe.update_operator_payload(&UpdateOperatorParams(vec![
                            UpdateOperator {
                                operator: mint_fund_contract.0.into(),
                                update:   OperatorUpdate::Add,
                            },
                        ])),
                    )
                })
                .expect("Failed to update euroe operator for mint fund 1 for user_1");
            user_1
                .transact(|account| {
                    chain.update_with_energy(
                        account,
                        mint_fund_contract.transfer_invest_payload(&TransferInvestParams {
                            amount:         TokenAmountU64(100),
                            security_token: TokenUId {
                                contract: fp_1_contract.0,
                                id:       fp_1_token_1,
                            },
                        }),
                        Energy { energy: 30_000 },
                    )
                })
                .expect("Failed to transfer invest for user_1");
            processor
                .process_block(&mut db_conn, &chain.produce_block())
                .await
                .expect("Error processing block");
        }

        // user 2 investments in mint fund 1
        {
            user_2
                .transact(|account| {
                    chain.update(
                        account,
                        euroe.update_operator_payload(&UpdateOperatorParams(vec![
                            UpdateOperator {
                                operator: mint_fund_contract.0.into(),
                                update:   OperatorUpdate::Add,
                            },
                        ])),
                    )
                })
                .expect("Failed to update euroe operator for mint fund 1 for user_2");
            user_2
                .transact(|account| {
                    chain.update_with_energy(
                        account,
                        mint_fund_contract.transfer_invest_payload(&TransferInvestParams {
                            amount:         TokenAmountU64(200),
                            security_token: TokenUId {
                                contract: fp_1_contract.0,
                                id:       fp_1_token_1,
                            },
                        }),
                        Energy { energy: 30_000 },
                    )
                })
                .expect("Failed to transfer invest for user_2");
            processor
                .process_block(&mut db_conn, &chain.produce_block())
                .await
                .expect("Error processing block");
        }

        // asserting user portfolio after investment in mint fund 1
        {
            let project = user_1
                .call_api(|token| api.forest_project_get(token, fp_1.id))
                .await;
            assert_eq!(project.user_balance, 100.into()); // user 1 has 100 shares

            let portfolio = user_1
                .call_api(|token| {
                    api.portfolio_aggregate(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
                euro_e_token_metadata:          Some(TokenMetadata {
                    contract_address: euroe.0.to_decimal(),
                    token_id:         TokenIdUnit().to_decimal(),
                    symbol:           Some("EUROe".to_string()),
                    decimals:         Some(6),
                }),
                locked_mint_fund_euro_e_amount: 100.into(),
                invested_value:                 100.into(),
                current_portfolio_value:        Decimal::ZERO,
                carbon_tons_offset:             Decimal::ZERO,
                monthly_return:                 Decimal::from(-100),
                return_on_investment:           Decimal::from(-100),
                yearly_return:                  Decimal::from(-100),
            });

            let portfolio = user_2
                .call_api(|token| {
                    api.portfolio_aggregate(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
                euro_e_token_metadata:          Some(TokenMetadata {
                    contract_address: euroe.0.to_decimal(),
                    token_id:         TokenIdUnit().to_decimal(),
                    symbol:           Some("EUROe".to_string()),
                    decimals:         Some(6),
                }),
                locked_mint_fund_euro_e_amount: 200.into(),
                invested_value:                 200.into(),
                current_portfolio_value:        Decimal::ZERO,
                carbon_tons_offset:             Decimal::ZERO,
                monthly_return:                 Decimal::from(-200),
                return_on_investment:           Decimal::from(-100),
                yearly_return:                  Decimal::from(-200),
            });
        }

        // successfull completion of mint fund 1
        {
            admin
                .transact(|account| {
                    chain.update(
                        account,
                        mint_fund_contract.update_fund_state_payload(&UpdateFundStateParams {
                            state:          UpdateFundState::Success(admin.address().into()),
                            security_token: TokenUId {
                                contract: fp_1_contract.0,
                                id:       fp_1_token_1,
                            },
                        }),
                    )
                })
                .expect("Failed to update mint fund 1 state to success");
            processor
                .process_block(&mut db_conn, &chain.produce_block())
                .await
                .expect("Error processing block");

            let investors = admin
                .call_api(|token| {
                    api.admin_forest_project_investor_list(
                        token,
                        Some(fp_1.id),
                        None,
                        None,
                        0,
                        None,
                    )
                })
                .await
                .data;
            assert_eq!(investors.len(), 2);
            println!("Investors: {:?}", investors);
            admin
                .transact(|sender| {
                    chain.update_with_energy(
                        sender,
                        mint_fund_contract.claim_investment_payload(
                            &security_mint_fund::types::ClaimInvestmentParams {
                                investments: investors
                                    .iter()
                                    .map(|i| security_mint_fund::types::ClaimInvestmentParam {
                                        investor:       i
                                            .investor
                                            .investor
                                            .parse()
                                            .expect("Failed to parse investor"),
                                        security_token: TokenUId {
                                            contract: fp_1_contract.0,
                                            id:       fp_1_token_1,
                                        },
                                    })
                                    .collect(),
                            },
                        ),
                        Energy { energy: 60_000 },
                    )
                })
                .expect("Failed to claim mint fund 1 for investors");

            processor
                .process_block(&mut db_conn, &chain.produce_block())
                .await
                .expect("Error processing block");
        }

        // assertion of portfolio after mint fund completion
        {
            let portfolio = user_1
                .call_api(|token| {
                    api.portfolio_aggregate(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
                euro_e_token_metadata:          Some(TokenMetadata {
                    contract_address: euroe.0.to_decimal(),
                    token_id:         TokenIdUnit().to_decimal(),
                    symbol:           Some("EUROe".to_string()),
                    decimals:         Some(6),
                }),
                locked_mint_fund_euro_e_amount: 0.into(),
                invested_value:                 100.into(),
                current_portfolio_value:        200.into(), // 100 shares at 2 price
                yearly_return:                  100.into(),
                monthly_return:                 100.into(),
                return_on_investment:           100.into(),
                carbon_tons_offset:             Decimal::ZERO,
            });

            let portfolio = user_2
                .call_api(|token| {
                    api.portfolio_aggregate(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
                euro_e_token_metadata:          Some(TokenMetadata {
                    contract_address: euroe.0.to_decimal(),
                    token_id:         TokenIdUnit().to_decimal(),
                    symbol:           Some("EUROe".to_string()),
                    decimals:         Some(6),
                }),
                locked_mint_fund_euro_e_amount: 0.into(),
                invested_value:                 200.into(),
                current_portfolio_value:        400.into(),
                yearly_return:                  200.into(),
                monthly_return:                 200.into(),
                return_on_investment:           100.into(),
                carbon_tons_offset:             Decimal::ZERO,
            });
        }

        // asserting owned forest projects for user 1
        {
            let user_1_owned_fps = user_1
                .call_api(|id_token| api.forest_project_list_owned(id_token))
                .await;
            assert_eq!(user_1_owned_fps.data.len(), 1);
        }
    }

    let mint_fund_completion_time = chain.block_time_naive_utc();
    // asserting portfolio values over time
    {
        let forest_project = admin
            .call_api(|token| api.admin_find_forest_project(token, fp_1.id))
            .await;
        for month in 1..=12 {
            chain.tick_block_time(concordium_smart_contract_testing::Duration::from_days(30));
            let price = ForestProjectPrice {
                price: Decimal::from(2 + month),
                price_at: chain.block_time_naive_utc(),
                currency_token_id: TokenIdUnit().to_decimal(),
                currency_token_contract_address: euroe.0.to_decimal(),
                project_id: forest_project.id,
            };
            admin
                .call_api(|token| {
                    api.admin_forest_project_create_price(token, forest_project.id, &price)
                })
                .await;
        }

        // asserting user portfolio after 12 months at mint_fund_completion_time
        let portfolio = user_1
            .call_api(|token| api.portfolio_aggregate(token, Some(mint_fund_completion_time)))
            .await;
        assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
            euro_e_token_metadata:          Some(TokenMetadata {
                contract_address: euroe.0.to_decimal(),
                token_id:         TokenIdUnit().to_decimal(),
                symbol:           Some("EUROe".to_string()),
                decimals:         Some(6),
            }),
            locked_mint_fund_euro_e_amount: 0.into(),
            invested_value:                 100.into(),
            current_portfolio_value:        200.into(), // 100 shares at 2 price
            yearly_return:                  100.into(),
            monthly_return:                 100.into(),
            return_on_investment:           100.into(),
            carbon_tons_offset:             Decimal::ZERO,
        });

        chain.tick_block_time(concordium_smart_contract_testing::Duration::from_days(15));
        // asserting user portfolio after 12 months at current time
        let portfolio = user_1
            .call_api(|token| api.portfolio_aggregate(token, Some(chain.block_time_naive_utc())))
            .await;
        assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
            euro_e_token_metadata:          Some(TokenMetadata {
                contract_address: euroe.0.to_decimal(),
                token_id:         TokenIdUnit().to_decimal(),
                symbol:           Some("EUROe".to_string()),
                decimals:         Some(6),
            }),
            locked_mint_fund_euro_e_amount: 0.into(),
            invested_value:                 100.into(),
            // (initial price of 2 + 12 months increase of 1 each month) * 100 shares
            current_portfolio_value:        1400.into(),
            yearly_return:                  1200.into(),
            monthly_return:                 100.into(),
            return_on_investment:           1300.into(),
            carbon_tons_offset:             Decimal::ZERO,
        });
    }

    // adding yeilds to the forest project 1
    let fp_1_token_2 = TokenIdU64(2);
    {
        // minting carbon credits
        admin
            .transact(|sender| {
                chain.update(
                    sender,
                    carbon_credits.mint_payload(&security_sft_single::types::MintParams {
                        token_id: TokenIdUnit(),
                        owners:   vec![security_sft_single::types::MintParam {
                            address: Receiver::Account(admin.address()),
                            amount:  TokenAmountSecurity::new_un_frozen(1000000.into()),
                        }],
                    }),
                )
            })
            .expect("Failed to mint carbon credits");
        // minting tree sft
        admin
            .transact(|sender| {
                chain.update(
                    sender,
                    tree_sft.mint_payload(&security_sft_single::types::MintParams {
                        token_id: TokenIdUnit(),
                        owners:   vec![security_sft_single::types::MintParam {
                            address: Receiver::Account(admin.address()),
                            amount:  TokenAmountSecurity::new_un_frozen(1000000.into()),
                        }],
                    }),
                )
            })
            .expect("Failed to mint tree sft");
        // adding yields
        admin
            .transact(|sender| {
                chain.update(
                    sender,
                    fp_1_contract.add_token_payload(&AddTokenParams {
                        token_id:       fp_1_token_2,
                        token_metadata: ContractMetadataUrl {
                            url:  "https://metadata.com/fp_2".to_string(),
                            hash: None,
                        },
                    }),
                )
            })
            .expect("Failed to add token to forest project");
        admin
            .transact(|sender| {
                chain.update(
                    sender,
                    yielder_contract.upsert_yield_payload(&UpsertYieldParams {
                        token_contract: fp_1_contract.0,
                        token_id:       fp_1_token_2,
                        yields:         vec![
                            YieldState {
                                calculation: YieldCalculation::Quantity(Rate::new(10, 1).unwrap()),
                                token_id:    TokenIdVec(vec![]),
                                contract:    euroe.0,
                            },
                            YieldState {
                                calculation: YieldCalculation::Quantity(Rate::new(500, 1).unwrap()),
                                token_id:    TokenIdVec(vec![]),
                                contract:    carbon_credits.0,
                            },
                            YieldState {
                                calculation: YieldCalculation::Quantity(Rate::new(1, 1).unwrap()),
                                token_id:    TokenIdVec(vec![]),
                                contract:    tree_sft.0,
                            },
                        ],
                    }),
                )
            })
            .expect("Failed to add euroe reward to forest project 1");

        processor
            .process_block(&mut db_conn, &chain.produce_block())
            .await
            .expect("Error processing block");
    }

    // asserting rewards for user 1
    {
        // Asserting total rewards for user 1
        let rewards_total = user_1
            .call_api(|id_token| api.forest_project_yields_total(id_token))
            .await;
        assert_eq!(rewards_total, vec![
            UserYieldsAggregate {
                cognito_user_id:          user_1.id.clone(),
                yielder_contract_address: yielder_contract.0.to_decimal(),
                yield_token_id:           0.into(),
                yield_contract_address:   euroe.0.to_decimal(),
                yield_amount:             1000.into(),
                yield_token_decimals:     6,
                yield_token_symbol:       "EUROe".to_string(),
            },
            UserYieldsAggregate {
                cognito_user_id:          user_1.id.clone(),
                yielder_contract_address: yielder_contract.0.to_decimal(),
                yield_token_id:           0.into(),
                yield_contract_address:   carbon_credits.0.to_decimal(),
                yield_amount:             50000.into(),
                yield_token_decimals:     0,
                yield_token_symbol:       "CC".to_string(),
            },
            UserYieldsAggregate {
                cognito_user_id:          user_1.id.clone(),
                yielder_contract_address: yielder_contract.0.to_decimal(),
                yield_token_id:           0.into(),
                yield_contract_address:   tree_sft.0.to_decimal(),
                yield_amount:             100.into(),
                yield_token_decimals:     0,
                yield_token_symbol:       "TREES".to_string(),
            },
        ]);

        // Asserting claimable rewards for user 1
        let rewards_claimable = user_1
            .call_api(|id_token| api.forest_project_yields_claimable(id_token))
            .await;
        assert_eq!(rewards_claimable, vec![ForestProjectTokenUserYieldClaim {
            forest_project_id:        fp_1.id,
            token_id:                 fp_1_token_1.to_decimal(),
            max_token_id:             fp_1_token_2.to_decimal(),
            token_contract_address:   fp_1_contract.0.to_decimal(),
            holder_address:           user_1.account_address.clone(),
            token_balance:            100.into(),
            cognito_user_id:          user_1.id.clone(),
            yielder_contract_address: yielder_contract.0.to_decimal(),
        }]);
    }

    // user 1 owned forest projects assertions
    {
        let user_owned_token_contracts = user_1
            .call_api(|id_token| {
                api.forest_project_token_contracts_list_owned(id_token, None, None)
            })
            .await;
        assert_eq!(user_owned_token_contracts.data.len(), 1);
        assert_eq!(
            user_owned_token_contracts.data[0],
            ForestProjectTokenContractAggApiModel {
                forest_project_id:               fp_1.id,
                token_contract_address:          fp_1_contract.0.to_decimal(),
                carbon_credit_yield_balance:     50000.into(),
                carbon_credit_token_decimal:     0,
                currency_token_decimal:          6,
                euro_e_token_decimal:            6,
                currency_token_contract_address: euroe.0.to_decimal(),
                currency_token_id:               0.into(),
                euro_e_yields_balance:           1000.into(),
                user_balance:                    100.into(),
                user_balance_price:              (100 * 14).into(),
                currency_token_symbol:           "EUROe".to_string(),
                forest_project_name:             "Forest Project 1".to_string(),
                token_contract_type:             SecurityTokenContractType::Property,
            }
        );
    }

    // user 1 claims yields
    {
        // claiming yields
        user_1
            .transact(|sender| {
                chain.update_with_energy(
                    sender,
                    yielder_contract.yield_for_payload(&YieldParams {
                        owner:  sender,
                        yields: vec![YieldParam {
                            amount:         100.into(),
                            token_contract: fp_1_contract.0,
                            token_ver_from: fp_1_token_1,
                            token_ver_to:   fp_1_token_2,
                        }],
                    }),
                    Energy { energy: 60_000 },
                )
            })
            .expect("Failed to claim euro e rewards for user 1");
        processor
            .process_block(&mut db_conn, &chain.produce_block())
            .await
            .expect("Error processing block");

        // Asserting total rewards for user 1
        {
            let rewards_total = user_1
                .call_api(|id_token| api.forest_project_yields_total(id_token))
                .await;
            assert_eq!(rewards_total, vec![]);
        }

        // Asserting claimable rewards for user 1
        {
            let rewards_claimable = user_1
                .call_api(|id_token| api.forest_project_yields_claimable(id_token))
                .await;
            assert_eq!(rewards_claimable, vec![]);
        }

        // asserting owned forest projects for user 1
        {
            let user_1_owned_fps = user_1
                .call_api(|id_token| api.forest_project_list_owned(id_token))
                .await;
            assert_eq!(user_1_owned_fps.data.len(), 1);
        }
    }

    // user 1 burns claimed carbon credits
    {
        user_1
            .transact(|sender| {
                chain.update(
                    sender,
                    carbon_credits.burn_payload(
                        &concordium_protocols::concordium_cis2_security::BurnParams(vec![
                            security_sft_single::types::Burn {
                                amount:   1000.into(),
                                owner:    sender.into(),
                                token_id: TokenIdUnit(),
                            },
                        ]),
                    ),
                )
            })
            .expect("Failed to burn carbon credits for user 1");
        processor
            .process_block(&mut db_conn, &chain.produce_block())
            .await
            .expect("Error processing block");
    }

    // user 1 transfers the forest project units & assertions
    {
        // asserting portfolio values before manual transfer
        {
            let portfolio = user_1
                .call_api(|token| {
                    api.portfolio_aggregate(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
                euro_e_token_metadata:          Some(TokenMetadata {
                    contract_address: euroe.0.to_decimal(),
                    token_id:         TokenIdUnit().to_decimal(),
                    symbol:           Some("EUROe".to_string()),
                    decimals:         Some(6),
                }),
                locked_mint_fund_euro_e_amount: 0.into(),
                invested_value:                 100.into(),
                current_portfolio_value:        1400.into(),
                yearly_return:                  1200.into(),
                monthly_return:                 100.into(),
                return_on_investment:           1300.into(),
                carbon_tons_offset:             1000.into(),
            });
        }

        user_1
            .transact(|sender| {
                chain.update(
                    sender,
                    fp_1_contract.transfer_payload(&TransferParams(vec![Transfer {
                        amount:   TokenAmountU64(50),
                        token_id: fp_1_token_2,
                        to:       user_2.address().into(),
                        from:     sender.into(),
                        data:     AdditionalData::empty(),
                    }])),
                )
            })
            .expect("Failed to transfer forest project 1 units from user 1 to user 2");
        processor
            .process_block(&mut db_conn, &chain.produce_block())
            .await
            .expect("Error processing block");

        // Asserting portfolio values after transfer
        {
            let portfolio = user_1
                .call_api(|token| {
                    api.portfolio_aggregate(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            // here only the `current_portfolio_value` has changed
            // because transfers are assumed to be done at market rate / forest project price
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
                euro_e_token_metadata:          Some(TokenMetadata {
                    contract_address: euroe.0.to_decimal(),
                    token_id:         TokenIdUnit().to_decimal(),
                    symbol:           Some("EUROe".to_string()),
                    decimals:         Some(6),
                }),
                locked_mint_fund_euro_e_amount: 0.into(),
                invested_value:                 100.into(),
                current_portfolio_value:        700.into(),
                yearly_return:                  1200.into(),
                monthly_return:                 100.into(),
                return_on_investment:           1300.into(),
                carbon_tons_offset:             1000.into(),
            });
        }

        // Asserting total rewards for user 1
        // These are unchanged because because on transferring tokens the rewards are not transferred
        {
            let rewards_total = user_1
                .call_api(|id_token| api.forest_project_yields_total(id_token))
                .await;
            assert_eq!(rewards_total, vec![]);
        }

        // Asserting claimable rewards for user 1
        {
            let rewards_claimable = user_1
                .call_api(|id_token| api.forest_project_yields_claimable(id_token))
                .await;
            assert_eq!(rewards_claimable, vec![]);
        }
    }

    // user 1 open a sell position of p2p trade contract
    {
        // Asserting portfolio values BEFORE opening sell position
        {
            let portfolio = user_1
                .call_api(|token| {
                    api.portfolio_aggregate(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
                euro_e_token_metadata:          Some(TokenMetadata {
                    contract_address: euroe.0.to_decimal(),
                    token_id:         TokenIdUnit().to_decimal(),
                    symbol:           Some("EUROe".to_string()),
                    decimals:         Some(6),
                }),
                locked_mint_fund_euro_e_amount: 0.into(),
                invested_value:                 100.into(),
                current_portfolio_value:        700.into(),
                yearly_return:                  1200.into(),
                monthly_return:                 100.into(),
                return_on_investment:           1300.into(),
                carbon_tons_offset:             1000.into(),
            });
        }

        // admin creates a market in the trading contract
        {
            admin
                .transact(|sender| {
                    chain.update(
                        sender,
                        trading_contract.add_market_payload(&AddMarketParams {
                            token_contract: fp_1_contract.0,
                            market:         Market::Transfer(TransferMarket {
                                token_id:           fp_1_token_2,
                                liquidity_provider: sender,
                                buy_rate:           Rate::new(1, 2).unwrap(),
                                sell_rate:          Rate::new(1, 2).unwrap(),
                            }),
                        }),
                    )
                })
                .expect("Failed to create market for forest project 1");
            processor
                .process_block(&mut db_conn, &chain.produce_block())
                .await
                .expect("Error processing block");
        }

        // user 1 sells
        {
            user_1
                .transact(|sender| {
                    chain.update(
                        sender,
                        trading_contract.sell_payload(&ExchangeParams {
                            contract: fp_1_contract.0,
                            amount:   TokenAmountU64(50),
                            rate:     Rate::new(1, 2).unwrap(),
                        }),
                    )
                })
                .expect("Failed to sell for user 1");

            processor
                .process_block(&mut db_conn, &chain.produce_block())
                .await
                .expect("Error processing block");
        }

        // Asserting portfolio values AFTER selling
        {
            let portfolio = user_1
                .call_api(|token| {
                    api.portfolio_aggregate(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
                euro_e_token_metadata:          Some(TokenMetadata {
                    contract_address: euroe.0.to_decimal(),
                    token_id:         TokenIdUnit().to_decimal(),
                    symbol:           Some("EUROe".to_string()),
                    decimals:         Some(6),
                }),
                locked_mint_fund_euro_e_amount: 0.into(),
                invested_value:                 75.into(), // 100 initially invested - 25 taken out
                current_portfolio_value:        0.into(),
                yearly_return:                  525.into(),
                monthly_return:                 (-575).into(),
                return_on_investment:           Decimal::from_f64(866.666667).unwrap(),
                carbon_tons_offset:             1000.into(),
            });
        }
    }

    // user 2 sells
    {
        user_2
            .transact(|sender| {
                chain.update(
                    sender,
                    fp_1_contract.update_operator_payload(&UpdateOperatorParams(vec![
                        UpdateOperator {
                            operator: trading_contract.0.into(),
                            update:   OperatorUpdate::Add,
                        },
                    ])),
                )
            })
            .expect("Failed to update p2p trade operator for forest project 1 for user 2");

        user_2
            .transact(|sender| {
                chain.update(
                    sender,
                    trading_contract.sell_payload(&ExchangeParams {
                        contract: fp_1_contract.0,
                        amount:   TokenAmountU64(50),
                        rate:     Rate::new(1, 2).unwrap(),
                    }),
                )
            })
            .expect("Failed to open sell position for user 2");
        processor
            .process_block(&mut db_conn, &chain.produce_block())
            .await
            .expect("Error processing block");
    }

    // user 1 buys
    {
        user_1
            .transact(|sender| {
                chain.update(
                    sender,
                    euroe.update_operator_payload(&UpdateOperatorParams(vec![UpdateOperator {
                        operator: trading_contract.0.into(),
                        update:   OperatorUpdate::Add,
                    }])),
                )
            })
            .expect("Failed to update euroe operator for p2p trade contract for user 1");

        user_1
            .transact(|sender| {
                chain.update(
                    sender,
                    trading_contract.buy_payload(&ExchangeParams {
                        contract: fp_1_contract.0,
                        amount:   TokenAmountU64(50),
                        rate:     Rate::new(1, 2).unwrap(),
                    }),
                )
            })
            .expect("Failed to buy for user 1");
        processor
            .process_block(&mut db_conn, &chain.produce_block())
            .await
            .expect("Error processing block");

        let portfolio = user_1
            .call_api(|token| api.portfolio_aggregate(token, Some(chain.block_time_naive_utc())))
            .await;
        assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
            euro_e_token_metadata:          Some(TokenMetadata {
                contract_address: euroe.0.to_decimal(),
                token_id:         TokenIdUnit().to_decimal(),
                symbol:           Some("EUROe".to_string()),
                decimals:         Some(6),
            }),
            locked_mint_fund_euro_e_amount: 0.into(),
            invested_value:                 100.into(),
            current_portfolio_value:        700.into(), // 50 shares at 14 price
            yearly_return:                  1175.into(),
            monthly_return:                 75.into(),
            return_on_investment:           1300.into(),
            carbon_tons_offset:             1000.into(),
        });
    }

    // user 1 transactions assertions
    {
        let user_1_txns = user_1
            .call_api(|token| api.user_transactions_list(token, Some(0)))
            .await;
        assert_eq!(user_1_txns.data, vec![
            UserTransaction {
                cognito_user_id:                 user_1.id.clone(),
                transaction_hash:
                    "1fc244af2b96d0169a177e2559af29e0484744e4b8501d1044d76c9f7b3cf307".to_string(),
                transaction_type:                "buy".to_string(),
                forest_project_id:               fp_1.id,
                currency_amount:                 Decimal::from(25),
                account_address:                 user_1.account_address.clone(),
                currency_token_contract_address: euroe.0.to_decimal(),
                currency_token_id:               0.into(),
                currency_token_symbol:           "EUROe".to_string(),
                currency_token_decimals:         6,
                block_height:                    Decimal::from(17),
            },
            UserTransaction {
                cognito_user_id:                 user_1.id.clone(),
                transaction_hash:
                    "daceee899cdb68711b2b16b1708d684b193062163a2ef8ca80e5a3f002212822".to_string(),
                transaction_type:                "sell".to_string(),
                forest_project_id:               fp_1.id,
                currency_amount:                 Decimal::from(25),
                account_address:                 user_1.account_address.clone(),
                currency_token_contract_address: euroe.0.to_decimal(),
                currency_token_id:               0.into(),
                currency_token_symbol:           "EUROe".to_string(),
                currency_token_decimals:         6,
                block_height:                    Decimal::from(15),
            },
            UserTransaction {
                cognito_user_id:                 user_1.id.clone(),
                transaction_hash:
                    "51be0458d20b1ce2ff5316e7e473109c26f5d8b457037edf8c4529f6ff82bddb".to_string(),
                forest_project_id:               fp_1.id,
                transaction_type:                "claimed".to_string(),
                currency_amount:                 Decimal::from(100),
                account_address:                 user_1.account_address.clone(),
                currency_token_contract_address: euroe.0.to_decimal(),
                currency_token_id:               0.into(),
                currency_token_symbol:           "EUROe".to_string(),
                currency_token_decimals:         6,
                block_height:                    Decimal::from(9),
            },
            UserTransaction {
                cognito_user_id:                 user_1.id.clone(),
                transaction_hash:
                    "83440636eff7b2ec0f78ef7b8e480a033e8aeb67e6ecb657ce9bcdfdb21aa744".to_string(),
                transaction_type:                "invested".to_string(),
                forest_project_id:               fp_1.id,
                currency_amount:                 Decimal::from(100),
                account_address:                 user_1.account_address.clone(),
                currency_token_contract_address: euroe.0.to_decimal(),
                currency_token_id:               0.into(),
                currency_token_symbol:           "EUROe".to_string(),
                currency_token_decimals:         6,
                block_height:                    Decimal::from(6),
            },
        ]);
    }

    // admin adds agent to offchain rewards
    {
        chain.create_account_with_keys(
            offchain_agent_account_address,
            offchain_agent_account_keys,
            DEFAULT_ACCOUNT_BALANCE,
        );

        admin
            .transact(|sender| {
                chain.update(
                    sender,
                    offchain_rewards.add_agent_payload(&offchain_rewards::types::Agent {
                        address: offchain_agent_account_address.into(),
                    }),
                )
            })
            .expect("Failed to add agent to offchain rewards");
        admin
            .transact(|sender| {
                chain.update(
                    sender,
                    euroe.update_operator_payload(&UpdateOperatorParams(vec![UpdateOperator {
                        operator: offchain_rewards.0.into(),
                        update:   OperatorUpdate::Add,
                    }])),
                )
            })
            .expect("Failed to update euroe operator for offchain rewards");
    }

    // user 1 affiliate rewards
    {
        // Asserting affiliate rewards for user 1
        let reward = {
            let affiliate_rewards = user_1
                .call_api(|token| api.user_affiliate_rewards_list(token, Some(0)))
                .await;
            assert_eq!(affiliate_rewards.data.len(), 1);
            assert_eq!(affiliate_rewards.data[0].reward_amount, 0.into());
            assert_eq!(affiliate_rewards.data[0].remaining_reward_amount, 10.into());
            assert_eq!(
                affiliate_rewards.data[0].affiliate_cognito_user_id,
                user_1.id.clone()
            );

            affiliate_rewards.data[0].clone()
        };

        // Claiming affiliate rewards for user 1
        {
            let claim_req = user_1
                .call_api(|token| {
                    api.user_affiliate_rewards_claim(token, reward.investment_record_id)
                })
                .await;

            user_1
                .transact(|sender| {
                    chain.update(
                        sender,
                        offchain_rewards.claim_reward_payload(
                            &offchain_rewards::types::ClaimRequest {
                                claim:     offchain_rewards::types::ClaimInfo {
                                    account:               claim_req.claim.account.parse().unwrap(),
                                    account_nonce:         claim_req.claim.account_nonce,
                                    contract_address:      ContractAddress::new(
                                        claim_req.claim.contract_address.to_u64().unwrap(),
                                        0,
                                    ),
                                    reward_id:             claim_req.claim.reward_id,
                                    reward_amount:         TokenAmountU64(
                                        claim_req.claim.reward_amount.to_u64().unwrap(),
                                    ),
                                    reward_token_id:       TokenIdUnit(),
                                    reward_token_contract: ContractAddress::new(
                                        claim_req.claim.reward_token_contract.to_u64().unwrap(),
                                        0,
                                    ),
                                },
                                signer:    claim_req.signer.parse().unwrap(),
                                signature: serde_json::from_value(claim_req.signature)
                                    .expect("signature deserialization"),
                            },
                        ),
                    )
                })
                .expect("Failed to claim affiliate rewards for user 1");

            processor
                .process_block(&mut db_conn, &chain.produce_block())
                .await
                .expect("Error processing block");
        }

        // Asserting affiliate rewards for user 1 after claiming
        {
            let affiliate_rewards = user_1
                .call_api(|token| api.user_affiliate_rewards_list(token, Some(0)))
                .await;
            assert_eq!(affiliate_rewards.data.len(), 1);
            assert_eq!(
                affiliate_rewards.data[0].affiliate_commission,
                Decimal::from_f64(0.05).unwrap()
            );
            assert_eq!(affiliate_rewards.data[0].reward_amount, 10.into());
            // remaining reward amount should be 0 after claiming
            assert_eq!(affiliate_rewards.data[0].remaining_reward_amount, 0.into());
            assert_eq!(
                affiliate_rewards.data[0].affiliate_cognito_user_id,
                user_1.id
            );
        }
    }

    // user 1 owned forest projects assertions
    {
        let user_owned_token_contracts = user_1
            .call_api(|id_token| {
                api.forest_project_token_contracts_list_owned(id_token, None, None)
            })
            .await;
        assert_eq!(user_owned_token_contracts.data.len(), 1);
        assert_eq!(
            user_owned_token_contracts.data[0],
            ForestProjectTokenContractAggApiModel {
                forest_project_id:               fp_1.id,
                token_contract_address:          fp_1_contract.0.to_decimal(),
                carbon_credit_yield_balance:     0.into(),
                carbon_credit_token_decimal:     0,
                currency_token_decimal:          6,
                euro_e_token_decimal:            0,
                currency_token_contract_address: euroe.0.to_decimal(),
                currency_token_id:               0.into(),
                euro_e_yields_balance:           0.into(),
                user_balance:                    50.into(),
                user_balance_price:              (50 * 14).into(),
                currency_token_symbol:           "EUROe".to_string(),
                forest_project_name:             "Forest Project 1".to_string(),
                token_contract_type:             SecurityTokenContractType::Property,
            }
        );
    }
}

pub fn deploy_upwood_contracts(
    chain: &mut Chain,
    admin: &Account,
    euro_e_contract: EuroETestClient,
) -> (
    SftSingleTestClient,        // Carbon Credits Contract
    SftSingleTestClient,        // Tree SFT Contract
    NftMultiRewardedTestClient, // Tree NFT Contract
    MintFundTestClient,         // Mint Fund Contract
    P2PTradeTestClient,         // Trading Contract
    SftMultiYielderTestClient,  // Yielder Contract
    IdentityRegistryTestClient, // Identity Registry Contract
    ComplianceTestClient,       // Compliance Contract
    OffchainRewardsTestClient,  // Offchain Rewards Contract
) {
    let currency_token = TokenUId {
        contract: euro_e_contract.0,
        id:       TokenIdUnit(),
    };
    let mint_fund_contract = admin
        .transact(|account| {
            chain.init(
                account,
                MintFundTestClient::init_payload(&security_mint_fund::types::InitParam {
                    agents: vec![],
                    currency_token,
                }),
            )
        })
        .map(MintFundTestClient)
        .expect("Failed to init mint fund contract");
    let trading_contract = admin
        .transact(|account| {
            chain.init(
                account,
                P2PTradeTestClient::init_payload(&security_p2p_trading::InitParam {
                    currency: currency_token,
                    agents:   vec![],
                }),
            )
        })
        .map(P2PTradeTestClient)
        .expect("Failed to init trading contract");
    admin
        .transact(|sender| {
            chain.update(
                sender,
                euro_e_contract.update_operator_single_payload(UpdateOperator {
                    update:   OperatorUpdate::Add,
                    operator: trading_contract.0.into(),
                }),
            )
        })
        .expect("Failed to update mint fund operator for trading contract");
    let yielder_contract = admin
        .transact(|account| {
            chain.init(
                account,
                SftMultiYielderTestClient::init_payload(&security_sft_multi_yielder::InitParam {
                    treasury: admin.address().into(),
                    agents:   vec![],
                }),
            )
        })
        .map(SftMultiYielderTestClient)
        .expect("Failed to init yeilder contract");
    admin
        .transact(|sender| {
            chain.update(
                sender,
                euro_e_contract.update_operator_single_payload(UpdateOperator {
                    update:   OperatorUpdate::Add,
                    operator: yielder_contract.0.into(),
                }),
            )
        })
        .expect("Failed to update euroe operator for yielder contract");
    let identity_registry = admin
        .transact(|account| chain.init(account, IdentityRegistryTestClient::init_payload(&())))
        .map(IdentityRegistryTestClient)
        .expect("Failed to init identity registry contract");
    let compliance_mod_contract = admin
    .transact(|account| chain.init(account, NationalitiesModuleTestClient::init_payload(&concordium_rwa_compliance::compliance_modules::allowed_nationalities::types::InitParams {
        identity_registry: identity_registry.0,
        nationalities: COMPLIANT_NATIONALITIES.iter().map(|s| s.to_string()).collect(),
    })))
    .expect("Failed to init compliance module contract");
    let compliance_contract = admin
        .transact(|account| {
            chain.init(
                account,
                ComplianceTestClient::init_payload(
                    &concordium_rwa_compliance::compliance::types::InitParams {
                        modules: vec![compliance_mod_contract],
                    },
                ),
            )
        })
        .map(ComplianceTestClient)
        .expect("Failed to init compliance contract");
    let carbon_credits = admin
        .transact(|account| {
            chain.init(
                account,
                SftSingleTestClient::init_payload(&security_sft_single::types::InitParam {
                    security:     None,
                    metadata_url: concordium_protocols::concordium_cis2_ext::ContractMetadataUrl {
                        url:  CARBON_CREDITS_METADATA_URL.to_string(),
                        hash: None,
                    },
                    agents:       vec![AgentWithRoles {
                        address: yielder_contract.0.into(),
                        roles:   vec![security_sft_single::types::AgentRole::Operator],
                    }],
                }),
            )
        })
        .map(SftSingleTestClient)
        .expect("Failed to init carbon credits contract");
    let tree_sft = admin
        .transact(|account| {
            chain.init(
                account,
                SftSingleTestClient::init_payload(&security_sft_single::types::InitParam {
                    security:     None,
                    metadata_url: concordium_protocols::concordium_cis2_ext::ContractMetadataUrl {
                        url:  TREE_SFT_METADATA_URL.to_string(),
                        hash: None,
                    },
                    agents:       vec![AgentWithRoles {
                        address: yielder_contract.0.into(),
                        roles:   vec![security_sft_single::types::AgentRole::Operator],
                    }],
                }),
            )
        })
        .map(SftSingleTestClient)
        .expect("Failed to init tree sft contract");
    let tree_nft = admin
        .transact(|account| {
            chain.init(
                account,
                NftMultiRewardedTestClient::init_payload(&nft_multi_rewarded::types::InitParam {
                    reward_token: TokenUId {
                        contract: tree_sft.0,
                        id:       TokenIdUnit(),
                    },
                }),
            )
        })
        .map(NftMultiRewardedTestClient)
        .expect("Failed to init tree nft contract");
    admin
        .transact(|account| {
            chain.update(
                account,
                tree_sft.add_agent_payload(&AgentWithRoles {
                    address: tree_nft.contract_address().into(),
                    roles:   vec![security_sft_single::types::AgentRole::Operator],
                }),
            )
        })
        .expect("Failed to add agent to tree sft contract");
    let offchain_rewards = admin
        .transact(|account| {
            chain.init(
                account,
                OffchainRewardsTestClient::init_payload(&offchain_rewards::types::InitParam {
                    treasury: admin.address().into(),
                }),
            )
        })
        .map(OffchainRewardsTestClient)
        .expect("Failed to init offchain rewards contract");

    (
        carbon_credits,
        tree_sft,
        tree_nft,
        mint_fund_contract,
        trading_contract,
        yielder_contract,
        identity_registry,
        compliance_contract,
        offchain_rewards,
    )
}

#[allow(clippy::too_many_arguments)]
async fn create_forest_project(
    chain: &mut Chain,
    api: &mut ApiTestClient,
    db_conn: &mut DbConn,
    processors: &mut Processors,
    euro_e_contract: &EuroETestClient,
    admin: &UserTestClient,
    identity_registry: &IdentityRegistryTestClient,
    compliance_contract: &ComplianceTestClient,
    mint_fund_contract: &MintFundTestClient,
    yielder_contract: &SftMultiYielderTestClient,
    trading_contract: &P2PTradeTestClient,
    index: u16,
    latest_price: Decimal,
    test_image_id: u32,
) -> (
    SftMultiTestClient, // project property contract
    SftMultiTestClient, // project fund pre sale contract
    ForestProject,      // project
) {
    let project = ForestProject {
        id: Uuid::new_v4(),
        area: "100 HA".to_string(),
        desc_long: format!("Forest Project {} Long description", index),
        name: format!("Forest Project {}", index),
        desc_short: format!("Forest Project {} Short description", index),
        property_media_header: "Property Media Header".to_string(),
        property_media_footer: "Property Media Footer".to_string(),
        image_small_url: format!("https://picsum.photos/id/{}/550/250", test_image_id),
        image_large_url: format!("https://picsum.photos/id/{}/1088/494", test_image_id),
        label: "GROW".to_string(),
        carbon_credits: 200,
        shares_available: 100,
        roi_percent: 12.5,
        state: ForestProjectState::Draft,
        created_at: chain.block_time_naive_utc(),
        updated_at: chain.block_time_naive_utc(),
        offering_doc_title: None,
        offering_doc_header: None,
        offering_doc_img_url: None,
        offering_doc_footer: None,
        financial_projection_title: None,
        financial_projection_header: None,
        financial_projection_img_url: None,
        financial_projection_footer: None,
        geo_title: None,
        geo_header: None,
        geo_img_url: None,
        geo_footer: None,
    };
    let project = admin
        .call_api(|token| api.admin_create_forest_project(token, &project))
        .await;
    let price = ForestProjectPrice {
        project_id: project.id,
        price_at: chain.block_time_naive_utc(),
        currency_token_id: TokenIdUnit().to_decimal(),
        currency_token_contract_address: euro_e_contract.0.to_decimal(),
        price: latest_price,
    };
    admin
        .call_api(|token| api.admin_forest_project_create_price(token, project.id, &price))
        .await;

    let project_property_contract = admin
        .transact(|account| {
            chain.init(
                account,
                SftMultiTestClient::init_payload(&security_sft_multi::types::InitParam {
                    security: Some(SecurityParams {
                        compliance:        compliance_contract.0,
                        identity_registry: identity_registry.0,
                    }),
                    agents:   vec![
                        AgentWithRoles {
                            address: mint_fund_contract.0.into(),
                            roles:   vec![
                                security_sft_multi::types::AgentRole::Mint,
                                security_sft_multi::types::AgentRole::Operator,
                            ],
                        },
                        AgentWithRoles {
                            address: yielder_contract.0.into(),
                            roles:   vec![
                                security_sft_multi::types::AgentRole::Mint,
                                security_sft_multi::types::AgentRole::Operator,
                            ],
                        },
                        AgentWithRoles {
                            address: trading_contract.0.into(),
                            roles:   vec![security_sft_multi::types::AgentRole::Operator],
                        },
                    ],
                }),
            )
        })
        .map(SftMultiTestClient)
        .expect("Failed to init project property contract");
    let project_fund_pre_sale_contract = admin
        .transact(|account| {
            chain.init(
                account,
                SftMultiTestClient::init_payload(&security_sft_multi::types::InitParam {
                    security: Some(SecurityParams {
                        compliance:        compliance_contract.0,
                        identity_registry: identity_registry.0,
                    }),
                    agents:   vec![AgentWithRoles {
                        address: mint_fund_contract.0.into(),
                        roles:   vec![
                            security_sft_multi::types::AgentRole::Mint,
                            security_sft_multi::types::AgentRole::ForcedBurn,
                        ],
                    }],
                }),
            )
        })
        .map(SftMultiTestClient)
        .expect("Failed to init project fund pre sale contract");
    processors
        .process_block(db_conn, &chain.produce_block())
        .await
        .expect("Error processing block");

    {
        let property = ForestProjectTokenContract {
            forest_project_id: project.id,
            contract_address:  project_property_contract.0.to_decimal(),
            fund_token_id:     None,
            market_token_id:   None,
            contract_type:     SecurityTokenContractType::Property,
            symbol:            "FP1".to_string(),
            decimals:          0,
            metadata_url:      "https://metadata.com/fp1".to_string(),
            metadata_hash:     None,
            created_at:        chrono::Utc::now().naive_utc(),
            updated_at:        chrono::Utc::now().naive_utc(),
        };
        admin
            .call_api(|id_token| api.forest_project_token_contract_create(id_token, &property))
            .await;
    }

    {
        let property_pre_sale = ForestProjectTokenContract {
            forest_project_id: project.id,
            contract_address:  project_fund_pre_sale_contract.0.to_decimal(),
            fund_token_id:     None,
            market_token_id:   None,
            contract_type:     SecurityTokenContractType::PropertyPreSale,
            symbol:            "FP-PRE".to_string(),
            decimals:          0,
            metadata_url:      "https://metadata.com/fp2".to_string(),
            metadata_hash:     None,
            created_at:        chrono::Utc::now().naive_utc(),
            updated_at:        chrono::Utc::now().naive_utc(),
        };
        admin
            .call_api(|id_token| {
                api.forest_project_token_contract_create(id_token, &property_pre_sale)
            })
            .await;
    }

    (
        project_property_contract,
        project_fund_pre_sale_contract,
        project,
    )
}

pub fn deploy_modules(chain: &mut Chain, chain_admin: &Account) {
    chain_admin
        .transact(|account| chain.deploy_module(account, EuroETestClient::module()))
        .expect("Failed to deploy euroe module");
    chain_admin
        .transact(|account| chain.deploy_module(account, IdentityRegistryTestClient::module()))
        .expect("Failed to deploy identity registry module");
    chain_admin
        .transact(|account| chain.deploy_module(account, ComplianceTestClient::module()))
        .expect("Failed to deploy compliance module");
    chain_admin
        .transact(|account| chain.deploy_module(account, SftSingleTestClient::module()))
        .expect("Failed to deploy security sft single module");
    chain_admin
        .transact(|account| chain.deploy_module(account, NftMultiRewardedTestClient::module()))
        .expect("Failed to deploy security nft multi rewarded module");
    chain_admin
        .transact(|account| chain.deploy_module(account, MintFundTestClient::module()))
        .expect("Failed to deploy security mint fund module");
    chain_admin
        .transact(|account| chain.deploy_module(account, P2PTradeTestClient::module()))
        .expect("Failed to deploy security p2p trade module");
    chain_admin
        .transact(|account| chain.deploy_module(account, SftMultiTestClient::module()))
        .expect("Failed to deploy security p2p trade module");
    chain_admin
        .transact(|account| chain.deploy_module(account, OffchainRewardsTestClient::module()))
        .expect("Failed to deploy offchain rewards module");
    chain_admin
        .transact(|account| chain.deploy_module(account, SftMultiYielderTestClient::module()))
        .expect("Failed to deploy sft multi yielder module");
}

#[allow(clippy::too_many_arguments)]
pub async fn create_user(
    test_id: &str,
    chain: &mut Chain,
    user_pool: &mut UserPool,
    api: &mut ApiTestClient,
    admin: &UserTestClient,
    affiliate_account_address: Option<String>,
    user_affiliate_fees: Decimal,
    index: u8,
) -> UserTestClient {
    let user_account =
        chain.create_account(AccountAddress([100 + index; 32]), DEFAULT_ACCOUNT_BALANCE);
    let pass = PASS_GENERATOR.generate_one().unwrap();
    let (token, user) = create_login_user(
        user_pool,
        api,
        &admin.id_token,
        format!("user_{}_{}@yopmail.com", index, test_id).as_str(),
        pass.as_str(),
        user_account.address_str().as_str(),
        affiliate_account_address,
        Some(user_affiliate_fees),
    )
    .await;
    let user = UserTestClient {
        account_address: user.account_address.parse().unwrap(),
        email:           user.email,
        id:              user.cognito_user_id,
        id_token:        token,
        password:        pass.to_string(),
    };

    let contracts_config = admin.call_api(|_| api.system_config()).await;
    let euroe = EuroETestClient(contracts_config.euro_e());
    admin
        .transact(|account| {
            chain.update(
                account,
                euroe.mint_payload(&integration_tests::euroe::MintParams {
                    owner:  user.address().into(),
                    amount: TokenAmountU64(1000 * 1_000_000),
                }),
            )
        })
        .expect("Failed to mint euroe for user_1");

    let identity_registry = IdentityRegistryTestClient(contracts_config.identity_registry());
    admin
        .transact(|account| {
            chain.update(
                account,
                identity_registry.register_identity_payload(&RegisterIdentityParams {
                    address:  user.address().into(),
                    identity: Identity {
                        attributes:  vec![IdentityAttribute {
                            tag:   5,
                            value: COMPLIANT_NATIONALITIES[0].to_string(),
                        }],
                        credentials: vec![],
                    },
                }),
            )
        })
        .expect("Failed to add user_1 identity to identity registry");
    user
}
