//! This is meant to be a complete test of the forest project.
//! It is meant to be run with `cargo test --release -- --test tests_forest_projects`
#![feature(assert_matches)]
mod test_utils;

use chrono::Months;
use concordium_cis2::{
    OperatorUpdate, Receiver, TokenAmountU64, TokenIdUnit, TokenIdVec, UpdateOperator,
    UpdateOperatorParams,
};
use concordium_protocols::concordium_cis2_security::{Identity, TokenUId};
use concordium_protocols::rate::Rate;
use concordium_rwa_identity_registry::identities::RegisterIdentityParams;
use concordium_rwa_identity_registry::types::IdentityAttribute;
use concordium_smart_contract_testing::{AccountAddress, Amount};
use diesel::r2d2::ConnectionManager;
use events_listener::processors::cis2_utils::ContractAddressToDecimal;
use events_listener::processors::Processors;
use integration_tests::cis2_conversions::to_token_id_vec;
use integration_tests::compliance::{ComplianceTestClient, NationalitiesModuleTestClient};
use integration_tests::euroe::{EuroETestClient, RoleTypes};
use integration_tests::identity_registry::IdentityRegistryTestClient;
use integration_tests::nft_multi_rewarded_client::NftMultiRewardedTestClient;
use integration_tests::security_mint_fund_client::MintFundTestClient;
use integration_tests::security_p2p_trading_client::P2PTradeTestClient;
use integration_tests::security_sft_rewards_client::SftRewardsTestClient;
use integration_tests::security_sft_single_client::SftSingleTestClient;
use passwords::PasswordGenerator;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use security_mint_fund::{ClaimInvestmentParam, FundState, TransferInvestParams};
use security_sft_rewards::rewards::ClaimRewardsParam;
use security_sft_rewards::types::TRACKED_TOKEN_ID;
use shared::db::security_mint_fund::SecurityMintFundState;
use shared::db_app::forest_project::{
    ForestProject, ForestProjectHolderRewardTotal, ForestProjectState, ForestProjectUser,
    HolderReward,
};
use shared::db_shared::{DbConn, DbPool};
use test_log::test;
use test_utils::test_api::ApiTestClient;
use test_utils::test_chain::{Account, Chain};
use test_utils::test_cognito::CognitoTestClient;
use test_utils::test_user::UserTestClient;
use upwood::api;
use upwood::api::investment_portfolio::InvestmentPortfolioUserAggregate;
use upwood::api::user::UserRegisterReq;
use uuid::Uuid;

const PASS_GENERATOR: PasswordGenerator = PasswordGenerator::new()
    .length(10)
    .numbers(true)
    .lowercase_letters(true)
    .uppercase_letters(true)
    .symbols(true)
    .spaces(false)
    .strict(true);
pub const CHAIN_ADMIN: AccountAddress = AccountAddress([0u8; 32]);
pub const APP_ADMIN: AccountAddress = AccountAddress([1u8; 32]);
pub const DEFAULT_ACCOUNT_BALANCE: Amount = Amount::from_ccd(1_000);
const COMPLIANT_NATIONALITIES: [&str; 3] = ["IN", "US", "GB"];
const CARBON_CREDITS_METADATA_URL: &str = "https://metadata.com/carbon_credits";
const TREE_SFT_METADATA_URL: &str = "https://metadata.com/tree_sft";
const EUROE_REWARD_METADATA_URL: &str = "https://metadata.com/reward/euroe";
const CARBON_CREDITS_REWARD_METADATA_URL: &str = "https://metadata.com/reward/carbon_credits";
const TREE_SFT_REWARD_METADATA_URL: &str = "https://metadata.com/reward/tree_sft";
const COGNITO_POOL_ID: &str = "eu-west-2_5JdhiSJOg";
const COGNITO_CLIENT_ID: &str = "b3c0pbkt8hnut1ggl9evluhs2";
const AGENT_WALLET_JSON_STR: &str = "{\"type\":\"concordium-browser-wallet-account\",\"v\":0,\"environment\":\"testnet\",\"value\":{\"accountKeys\":{\"keys\":{\"0\":{\"keys\":{\"0\":{\"signKey\":\"ab3f1fa2303811f2d68581a212c12c9388fd9d530b9daa892ae1415c5154ea21\",\"verifyKey\":\"2d501e8dd06a4a4e37f4676a6e12f947c53f7ae5f35f372eb2bf023dd3fca2e7\"}},\"threshold\":1}},\"threshold\":1},\"credentials\":{\"0\":\"902429b5b471ebe0cbbb60a901ff2c885b8398abf738d52137133e890b2e84a95448fd136fe5ee3e7ee3c5a104169cd3\"},\"address\":\"4fWTMJSAymJoFeTbohJzwejT6Wzh1dAa2BtnbDicgjQrc94TgW\"}}";

#[test(tokio::test)]
pub async fn test_forest_projects() {
    let test_id = format!("fpsu_{}", uuid::Uuid::new_v4());
    let mut chain = Chain::new(
        chrono::Utc::now()
            .checked_sub_months(Months::new(12 * 10))
            .unwrap(),
    );
    let admin = chain.create_account(APP_ADMIN, DEFAULT_ACCOUNT_BALANCE);
    let chain_deployer = chain.create_account(CHAIN_ADMIN, DEFAULT_ACCOUNT_BALANCE);
    deploy_modules(&mut chain, &chain_deployer);

    let euroe = admin
        .transact(|account| chain.init(account, EuroETestClient::init_payload()))
        .map(EuroETestClient)
        .expect("Failed to init euroe contract");
    let identity_registry = admin
        .transact(|account| chain.init(account, IdentityRegistryTestClient::init_payload()))
        .map(IdentityRegistryTestClient)
        .expect("Failed to init identity registry contract");
    let compliance_mod_contract = admin
        .transact(|account| chain.init(account, NationalitiesModuleTestClient::init_payload(&concordium_rwa_compliance::compliance_modules::allowed_nationalities::init::InitParams {
            identity_registry: identity_registry.0,
            nationalities: COMPLIANT_NATIONALITIES.iter().map(|s| s.to_string()).collect(),
        })))
        .expect("Failed to init compliance module contract");
    let compliance_contract = admin
        .transact(|account| {
            chain.init(
                account,
                ComplianceTestClient::init_payload(
                    &concordium_rwa_compliance::compliance::init::InitParams {
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
                    compliance:        compliance_contract.0,
                    identity_registry: identity_registry.0,
                    sponsors:          None,
                    metadata_url:
                        concordium_protocols::concordium_cis2_ext::ContractMetadataUrl {
                            url:  CARBON_CREDITS_METADATA_URL.to_string(),
                            hash: None,
                        },
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
                    compliance:        compliance_contract.0,
                    identity_registry: identity_registry.0,
                    sponsors:          None,
                    metadata_url:
                        concordium_protocols::concordium_cis2_ext::ContractMetadataUrl {
                            url:  TREE_SFT_METADATA_URL.to_string(),
                            hash: None,
                        },
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
                        id:       TokenIdVec(vec![]),
                    },
                }),
            )
        })
        .map(NftMultiRewardedTestClient)
        .expect("Failed to init tree nft contract");

    let (db_config, _container) = shared_tests::create_new_database_container().await;
    shared::db_setup::run_migrations(&db_config.db_url());
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
        processor
            .process_block(&mut db_conn, &chain.produce_block())
            .await
            .expect("Error processing block");
    }

    let api_config = api::Config {
        api_socket_address: "localhost".to_string(),
        api_socket_port: 8080,
        db_pool_max_size: 10,
        aws_user_pool_id: COGNITO_POOL_ID.to_string(),
        aws_user_pool_client_id: COGNITO_CLIENT_ID.to_string(),
        aws_user_pool_region: "eu-west-2".to_string(),
        concordium_network: "testnet".to_string(),
        concordium_node_uri: "http://node.testnet.concordium.com:20000".to_string(),
        filebase_bucket_name: "".to_string(),
        filebase_s3_endpoint_url: "".to_string(),
        files_bucket_name: "upwood-dev-files-bucket".to_string(),
        postgres_db: db_config.postgres_db,
        postgres_host: db_config.postgres_host,
        postgres_user: db_config.postgres_user,
        postgres_password: db_config.postgres_password.into(),
        postgres_port: db_config.postgres_port.to_u32().unwrap(),
        carbon_credit_contract_index: carbon_credits.0.to_decimal(),
        compliance_contract_index: compliance_contract.0.to_decimal(),
        euro_e_contract_index: euroe.0.to_decimal(),
        identity_registry_contract_index: identity_registry.0.to_decimal(),
        tree_ft_contract_index: tree_sft.0.to_decimal(),
        tree_nft_contract_index: tree_nft.0.to_decimal(),
        filebase_access_key_id: "".into(),
        filebase_secret_access_key: "".into(),
        files_presigned_url_expiry_secs: 0,
        tree_nft_agent_wallet_json_str: AGENT_WALLET_JSON_STR.to_string(),
        user_challenge_expiry_duration_mins: 0,
    };
    let mut api = ApiTestClient::new(api_config.clone()).await;
    let aws_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let mut cognito = CognitoTestClient::new(
        &aws_config,
        api_config.aws_user_pool_id,
        api_config.aws_user_pool_client_id,
    );
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
    let admin = create_login_api_admin(
        &mut api,
        &mut cognito,
        format!("admin_{}@yopmail.com", test_id),
        admin,
    )
    .await;

    let user_1 = create_user(
        &test_id,
        &mut chain,
        &mut api,
        &mut cognito,
        &admin,
        &euroe,
        &identity_registry,
        1,
    )
    .await;

    let user_2 = create_user(
        &test_id,
        &mut chain,
        &mut api,
        &mut cognito,
        &admin,
        &euroe,
        &identity_registry,
        2,
    )
    .await;

    let (fp_1_contract, mint_fund_1, p2p_trade_1, fp_1) = create_forest_project(
        &mut chain,
        &mut api,
        &mut db_conn,
        &mut processor,
        &admin,
        &euroe,
        &carbon_credits,
        &tree_sft,
        &identity_registry,
        &compliance_contract,
        1,
        2.into(),
    )
    .await;

    // Adding forest project as a registered holder in Identity Registry
    // This is needed so that Forest Project COntract can hold Carbon Credits
    admin
        .transact(|sender| {
            chain.update(
                sender,
                identity_registry.register_identity_payload(RegisterIdentityParams {
                    address:  fp_1_contract.0.into(),
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
        .expect("Failed to register identity for forest project 1");
    admin
        .transact(|sender| {
            chain.update(
                sender,
                identity_registry.register_identity_payload(RegisterIdentityParams {
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
            .call_api(|token| api.forest_project_list_active(token, 0))
            .await;
        assert_eq!(projects.data.len(), 0);
    }

    let fp_1 = admin
        .call_api(|token| {
            api.admin_update_forest_project(token, ForestProject {
                state: ForestProjectState::Listed,
                ..fp_1
            })
        })
        .await;
    assert_eq!(fp_1.state, ForestProjectState::Listed);

    // asserting user forest project active list
    {
        let projects = user_1
            .call_api(|token| api.forest_project_list_active(token, 0))
            .await;
        assert_eq!(projects.data.len(), 1);
        assert_eq!(projects.data[0].id, fp_1.id);
        assert_eq!(projects.data[0].mint_fund_rate, 1.into());
        assert_eq!(
            projects.data[0].mint_fund_state,
            SecurityMintFundState::Open
        );
        assert_eq!(projects.data[0].mint_fund_token_un_frozen_balance, None);
        assert_eq!(projects.data[0].p2p_trading_token_amount, None);
    }

    // mint fund 1 investments
    {
        // user 1 investments in mint fund 1
        {
            user_1
                .transact(|account| {
                    chain.update(
                        account,
                        euroe
                            .cis2()
                            .update_operator_payload(&UpdateOperatorParams(vec![UpdateOperator {
                                operator: mint_fund_1.0.into(),
                                update:   OperatorUpdate::Add,
                            }])),
                    )
                })
                .expect("Failed to update euroe operator for mint fund 1 for user_1");
            user_1
                .transact(|account| {
                    chain.update(
                        account,
                        mint_fund_1.transfer_invest_payload(&TransferInvestParams {
                            amount: TokenAmountU64(100),
                        }),
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
                        euroe
                            .cis2()
                            .update_operator_payload(&UpdateOperatorParams(vec![UpdateOperator {
                                operator: mint_fund_1.0.into(),
                                update:   OperatorUpdate::Add,
                            }])),
                    )
                })
                .expect("Failed to update euroe operator for mint fund 1 for user_2");
            user_2
                .transact(|account| {
                    chain.update(
                        account,
                        mint_fund_1.transfer_invest_payload(&TransferInvestParams {
                            amount: TokenAmountU64(200),
                        }),
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
            let projects = user_1
                .call_api(|token| api.forest_project_list_active(token, 0))
                .await;
            assert_eq!(projects.data.len(), 1);
            assert_eq!(projects.data[0].id, fp_1.id);
            assert_eq!(
                projects.data[0].mint_fund_state,
                SecurityMintFundState::Open
            );
            assert_eq!(
                projects.data[0].mint_fund_token_frozen_balance,
                Some(100.into())
            );
            assert_eq!(projects.data[0].p2p_trading_token_amount, None);

            let portfolio = user_1
                .call_api(|token| {
                    api.portfolio_aggreagte(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
                locked_euro_e_amount:    100.into(),
                invested_value:          100.into(),
                current_portfolio_value: Decimal::ZERO,
                carbon_tons_offset:      Decimal::ZERO,
                monthly_return:          Decimal::ZERO,
                return_on_investment:    Decimal::from(-100),
                yearly_return:           Decimal::ZERO,
            });

            let portfolio = user_2
                .call_api(|token| {
                    api.portfolio_aggreagte(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
                locked_euro_e_amount:    200.into(),
                invested_value:          200.into(),
                current_portfolio_value: Decimal::ZERO,
                carbon_tons_offset:      Decimal::ZERO,
                monthly_return:          Decimal::ZERO,
                return_on_investment:    Decimal::from(-100),
                yearly_return:           Decimal::ZERO,
            });
        }

        // successfull completion of mint fund 1
        {
            admin
                .transact(|account| {
                    chain.update(
                        account,
                        mint_fund_1
                            .update_fund_state_payload(&FundState::Success(admin.address().into())),
                    )
                })
                .expect("Failed to update mint fund 1 state to success");
            processor
                .process_block(&mut db_conn, &chain.produce_block())
                .await
                .expect("Error processing block");

            let investors = admin
                .call_api(|token| api.admin_forest_project_investor_list(token, fp_1.id, 0))
                .await
                .data;
            assert_eq!(investors.len(), 2);
            println!("Investors: {:?}", investors);
            admin
                .transact(|account| {
                    chain.update(
                        account,
                        mint_fund_1.claim_investment_payload(
                            &security_mint_fund::ClaimInvestParams {
                                investments: investors
                                    .iter()
                                    .map(|i| ClaimInvestmentParam {
                                        investor: i
                                            .investor
                                            .parse()
                                            .expect("Failed to parse investor"),
                                    })
                                    .collect(),
                            },
                        ),
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
                    api.portfolio_aggreagte(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
                locked_euro_e_amount:    0.into(),
                invested_value:          100.into(),
                current_portfolio_value: 200.into(), // 100 shares at 2 price
                carbon_tons_offset:      Decimal::ZERO,
                monthly_return:          200.into(),
                return_on_investment:    100.into(),
                yearly_return:           200.into(),
            });

            let portfolio = user_2
                .call_api(|token| {
                    api.portfolio_aggreagte(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
                locked_euro_e_amount:    0.into(),
                invested_value:          200.into(),
                current_portfolio_value: 400.into(),
                carbon_tons_offset:      Decimal::ZERO,
                monthly_return:          400.into(),
                return_on_investment:    100.into(),
                yearly_return:           400.into(),
            });
        }

        // asserting owned forest projects for user 1
        {
            let user_1_owned_fps = user_1
                .call_api(|id_token| api.forest_project_list_owned(id_token))
                .await;
            assert_eq!(user_1_owned_fps.data.len(), 1);
            assert_eq!(user_1_owned_fps.data[0].id, fp_1.id);
            assert_eq!(
                user_1_owned_fps.data[0].project_token_holder_address,
                Some(user_1.account_address.clone())
            );
            assert_eq!(
                user_1_owned_fps.data[0].project_token_un_frozen_balance,
                Some(100.into())
            );
            assert_eq!(
                user_1_owned_fps.data[0].project_token_frozen_balance,
                Some(0.into())
            );
        }
    }

    let mint_fund_completion_time = chain.block_time_naive_utc();
    // asserting portfolio values over time
    {
        let forest_project = admin
            .call_api(|token| api.admin_find_forest_project(token, fp_1.id))
            .await;
        for month in 1..=12 {
            // increase the current time by 30 days
            let forest_project = forest_project.clone();
            chain.tick_block_time(concordium_smart_contract_testing::Duration::from_days(30));
            admin
                .call_api(|token| {
                    api.admin_update_forest_project(token, ForestProject {
                        id: forest_project.id,
                        latest_price: forest_project.latest_price + Decimal::from(month), // increase price by 1 each month
                        updated_at: chain.block_time_naive_utc(),
                        ..forest_project
                    })
                })
                .await;
        }

        // asserting user portfolio after 12 months at mint_fund_completion_time
        let portfolio = user_1
            .call_api(|token| api.portfolio_aggreagte(token, Some(mint_fund_completion_time)))
            .await;
        assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
            locked_euro_e_amount:    0.into(),
            invested_value:          100.into(),
            current_portfolio_value: 200.into(), // 100 shares at 2 price
            carbon_tons_offset:      Decimal::ZERO,
            monthly_return:          200.into(),
            return_on_investment:    100.into(),
            yearly_return:           200.into(),
        });

        chain.tick_block_time(concordium_smart_contract_testing::Duration::from_days(15));
        // asserting user portfolio after 12 months at current time
        let portfolio = user_1
            .call_api(|token| api.portfolio_aggreagte(token, Some(chain.block_time_naive_utc())))
            .await;
        assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
            locked_euro_e_amount:    0.into(),
            invested_value:          100.into(),
            // (initial price of 2 + 12 months increase of 1 each month) * 100 shares
            current_portfolio_value: 1400.into(),
            yearly_return:           1200.into(),
            monthly_return:          100.into(),
            return_on_investment:    1300.into(),
            carbon_tons_offset:      Decimal::ZERO,
        });
    }

    // adding rewards to the forest project 1
    {
        // adding EuroE Reward
        admin
            .transact(|sender| {
                chain.update(
                    sender,
                    fp_1_contract.transfer_add_reward_payload(
                        &security_sft_rewards::rewards::TransferAddRewardParams {
                            reward_token_id:       TokenIdVec(vec![]),
                            reward_token_contract: euroe.0,
                            data:
                                security_sft_rewards::rewards::AddRewardContractParam {
                                    metadata_url: nft_multi_rewarded::types::ContractMetadataUrl {
                                        url:  EUROE_REWARD_METADATA_URL.to_string(),
                                        hash: None,
                                    },
                                    rate:         Rate::new(10, 1).unwrap(),
                                },
                        },
                    ),
                )
            })
            .expect("Failed to add euroe reward to forest project 1");
        // minting carbon credits
        admin
            .transact(|sender| {
                chain.update(
                    sender,
                    carbon_credits.mint_payload(&security_sft_single::types::MintParams {
                        token_id: TokenIdUnit(),
                        owners:   vec![security_sft_single::types::MintParam {
                            address: admin.address(),
                            amount:  1000000.into(),
                        }],
                    }),
                )
            })
            .expect("Failed to mint carbon credits");
        // adding Carbon Credits Reward
        admin
            .transact(|sender| {
                chain.update(
                    sender,
                    fp_1_contract.transfer_add_reward_payload(
                        &security_sft_rewards::rewards::TransferAddRewardParams {
                            reward_token_id:       TokenIdVec(vec![]),
                            reward_token_contract: carbon_credits.0,
                            data:
                                security_sft_rewards::rewards::AddRewardContractParam {
                                    metadata_url: nft_multi_rewarded::types::ContractMetadataUrl {
                                        url:  CARBON_CREDITS_REWARD_METADATA_URL.to_string(),
                                        hash: None,
                                    },
                                    rate:         Rate::new(500, 1).unwrap(),
                                },
                        },
                    ),
                )
            })
            .expect("Failed to add carbon credits reward to forest project 1");
        // minting tree sft
        admin
            .transact(|sender| {
                chain.update(
                    sender,
                    tree_sft.mint_payload(&security_sft_single::types::MintParams {
                        token_id: TokenIdUnit(),
                        owners:   vec![security_sft_single::types::MintParam {
                            address: admin.address(),
                            amount:  1000000.into(),
                        }],
                    }),
                )
            })
            .expect("Failed to mint tree sft");
        // adding Tree SFT Reward
        admin
            .transact(|sender| {
                chain.update(
                    sender,
                    fp_1_contract.transfer_add_reward_payload(
                        &security_sft_rewards::rewards::TransferAddRewardParams {
                            reward_token_id:       TokenIdVec(vec![]),
                            reward_token_contract: tree_sft.0,
                            data:
                                security_sft_rewards::rewards::AddRewardContractParam {
                                    metadata_url: nft_multi_rewarded::types::ContractMetadataUrl {
                                        url:  TREE_SFT_REWARD_METADATA_URL.to_string(),
                                        hash: None,
                                    },
                                    rate:         Rate::new(1, 1).unwrap(),
                                },
                        },
                    ),
                )
            })
            .expect("Failed to add tree sft reward to forest project 1");

        processor
            .process_block(&mut db_conn, &chain.produce_block())
            .await
            .expect("Error processing block");
    }

    // asserting rewards for user 1
    {
        // Asserting total rewards for user 1
        let rewards_total = user_1
            .call_api(|id_token| api.forest_project_rewards_total(id_token))
            .await;
        assert_eq!(rewards_total, vec![
            ForestProjectHolderRewardTotal {
                holder_address:          user_1.address().to_string(),
                rewarded_token_id:       0.into(),
                rewarded_token_contract: euroe.0.to_decimal(),
                total_frozen_reward:     Decimal::ZERO,
                total_un_frozen_reward:  1000.into(),
            },
            ForestProjectHolderRewardTotal {
                holder_address:          user_1.address().to_string(),
                rewarded_token_id:       0.into(),
                rewarded_token_contract: carbon_credits.0.to_decimal(),
                total_frozen_reward:     Decimal::ZERO,
                total_un_frozen_reward:  50000.into(),
            },
            ForestProjectHolderRewardTotal {
                holder_address:          user_1.address().to_string(),
                rewarded_token_id:       0.into(),
                rewarded_token_contract: tree_sft.0.to_decimal(),
                total_frozen_reward:     Decimal::ZERO,
                total_un_frozen_reward:  100.into(),
            },
        ]);

        // Asserting claimable rewards for user 1
        let rewards_claimable = user_1
            .call_api(|id_token| api.forest_project_rewards_claimable(id_token))
            .await;
        assert_eq!(rewards_claimable, vec![HolderReward {
            id: fp_1.id,
            contract_address: fp_1.contract_address,
            holder_address: user_1.account_address.clone(),
            token_id: 1.into(),
            rewarded_token_id: 0.into(),
            rewarded_token_contract: euroe.0.to_decimal(),
            frozen_balance: Decimal::ZERO,
            un_frozen_balance: 100.into(),
            frozen_reward: Decimal::ZERO,
            un_frozen_reward: 1000.into(),
        }]);
    }

    // user 1 claim rewards and assert
    {
        user_1.transact(|sender| {
            chain.update(sender, fp_1_contract.claim_rewards_payload(&security_sft_rewards::rewards::ClaimRewardsParams {
                owner: Receiver::Account(sender),
                claims: vec![ClaimRewardsParam {
                    token_id: 1.into(), // received from `forest_project_rewards_claimable` call
                }]
            }))
        }).expect("Failed to claim rewards for user 1");

        processor
            .process_block(&mut db_conn, &chain.produce_block())
            .await
            .expect("Error processing block");

        let rewards_claimable = user_1
            .call_api(|id_token| api.forest_project_rewards_claimable(id_token))
            .await;
        assert_eq!(rewards_claimable, vec![HolderReward {
            id: fp_1.id,
            contract_address: fp_1.contract_address,
            holder_address: user_1.account_address.clone(),
            token_id: 2.into(),
            rewarded_token_id: 0.into(),
            rewarded_token_contract: carbon_credits.0.to_decimal(),
            frozen_balance: Decimal::ZERO,
            frozen_reward: Decimal::ZERO,
            un_frozen_balance: 100.into(),
            un_frozen_reward: 50000.into(),
        }]);
    }

    // // user 1 open a sell position of p2p trade contract
    // {
    //     user_1
    //         .transact(|sender| {
    //             chain.update(
    //                 sender,
    //                 p2p_trade_1.transfer_sell_payload(&security_p2p_trading::TransferSellParams {
    //                     amount: TokenAmountU64(100),
    //                     rate:   Rate::new(2, 1).unwrap(),
    //                 }),
    //             )
    //         })
    //         .expect("Failed to open sell position for user 1");
    // }
}

#[allow(clippy::too_many_arguments)]
async fn create_forest_project(
    chain: &mut Chain,
    api: &mut ApiTestClient,
    db_conn: &mut DbConn,
    processors: &mut Processors,
    admin: &UserTestClient,
    euroe: &EuroETestClient,
    carbon_credits: &SftSingleTestClient,
    tree_sft: &SftSingleTestClient,
    identity_registry: &IdentityRegistryTestClient,
    compliance_contract: &ComplianceTestClient,
    index: u16,
    latest_price: Decimal,
) -> (
    SftRewardsTestClient,
    MintFundTestClient,
    P2PTradeTestClient,
    ForestProject,
) {
    let project_contract = admin
        .transact(|account| {
            chain.init(
                account,
                SftRewardsTestClient::init_payload(&security_sft_rewards::types::InitParam {
                    identity_registry:         identity_registry.0,
                    compliance:                compliance_contract.0,
                    sponsors:                  None,
                    metadata_url:
                        concordium_protocols::concordium_cis2_ext::ContractMetadataUrl {
                            url:  format!("https://metadata.com/forest_project/{}", index),
                            hash: None,
                        },
                    blank_reward_metadata_url:
                        concordium_protocols::concordium_cis2_ext::ContractMetadataUrl {
                            url:  format!("https://metadata.com/blank_reward/{}", index),
                            hash: None,
                        },
                }),
            )
        })
        .map(SftRewardsTestClient)
        .expect("Failed to init sft rewards contract");

    // update operators for admin to that rewards can later be transferred to itself by the Forest Project Contract
    {
        admin
            .transact(|sender| {
                chain.update(
                    sender,
                    euroe
                        .cis2()
                        .update_operator_payload(&UpdateOperatorParams(vec![UpdateOperator {
                            operator: project_contract.0.into(),
                            update:   OperatorUpdate::Add,
                        }])),
                )
            })
            .expect("Failed to update euroe operator for project contract");
        admin
            .transact(|sender| {
                chain.update(
                    sender,
                    carbon_credits
                        .cis2()
                        .update_operator_payload(&UpdateOperatorParams(vec![UpdateOperator {
                            operator: project_contract.0.into(),
                            update:   OperatorUpdate::Add,
                        }])),
                )
            })
            .expect("Failed to update carbon credits operator for project contract");
        admin
            .transact(|sender| {
                chain.update(
                    sender,
                    tree_sft
                        .cis2()
                        .update_operator_payload(&UpdateOperatorParams(vec![UpdateOperator {
                            operator: project_contract.0.into(),
                            update:   OperatorUpdate::Add,
                        }])),
                )
            })
            .expect("Failed to update tree sft operator for project contract");
    }

    let fund_tracked_token = admin
        .transact(|account| {
            chain.init(
                account,
                SftSingleTestClient::init_payload(&security_sft_single::types::InitParam {
                    compliance:        compliance_contract.0,
                    identity_registry: identity_registry.0,
                    sponsors:          None,
                    metadata_url:
                        concordium_protocols::concordium_cis2_ext::ContractMetadataUrl {
                            url:  format!("https://metadata.com/fund_tracked_token/{}", index),
                            hash: None,
                        },
                }),
            )
        })
        .map(SftSingleTestClient)
        .expect("Failed to init tree sft contract");
    let mint_fund = admin
        .transact(|account| {
            chain.init(
                account,
                MintFundTestClient::init_payload(&security_mint_fund::InitParam {
                    currency_token:   TokenUId {
                        contract: euroe.0,
                        id:       TokenIdVec(vec![]),
                    },
                    investment_token: TokenUId {
                        contract: project_contract.0,
                        id:       to_token_id_vec(TRACKED_TOKEN_ID),
                    },
                    token:            TokenUId {
                        contract: fund_tracked_token.0,
                        id:       TokenIdVec(vec![]),
                    },
                    fund_state:       FundState::Open,
                    rate:             Rate::new(1, 1).unwrap(),
                }),
            )
        })
        .map(MintFundTestClient)
        .expect("Failed to init mint fund contract");
    admin
        .transact(|account| {
            chain.update(
                account,
                fund_tracked_token.add_agent_payload(&security_sft_single::types::Agent {
                    address: mint_fund.0.into(),
                    roles:   vec![
                        security_sft_single::types::AgentRole::Mint,
                        security_sft_single::types::AgentRole::Freeze,
                        security_sft_single::types::AgentRole::UnFreeze,
                        security_sft_single::types::AgentRole::ForcedBurn,
                        security_sft_single::types::AgentRole::ForcedTransfer,
                    ],
                }),
            )
        })
        .expect("Failed to add mint fund as agent to fund tracked token");
    admin.transact(|sender| {
        chain
            .update(
                sender,
                project_contract.add_agent_payload(&security_sft_rewards::types::Agent {
                    address: mint_fund.0.into(),
                    roles:   vec![security_sft_rewards::types::AgentRole::Mint],
                }),
            )
            .expect("Failed to add mint fund as agent to project contract")
    });
    let p2p_trade = admin
        .transact(|account| {
            chain.init(
                account,
                P2PTradeTestClient::init_payload(&security_p2p_trading::InitParam {
                    currency: TokenUId {
                        contract: euroe.0,
                        id:       TokenIdVec(vec![]),
                    },
                    token:    TokenUId {
                        contract: project_contract.0,
                        id:       to_token_id_vec(TRACKED_TOKEN_ID),
                    },
                }),
            )
        })
        .map(P2PTradeTestClient)
        .expect("Failed to init p2p trade contract");
    processors
        .process_block(db_conn, &chain.produce_block())
        .await
        .expect("Error processing block");
    let project = admin
        .call_api(|token| {
            api.admin_create_forest_project(token, ForestProject {
                id: Uuid::new_v4(),
                area: "100 HA".to_string(),
                desc_long: format!("Forest Project {} Long description", index),
                name: format!("Forest Project {}", index),
                desc_short: format!("Forest Project {} Short description", index),
                property_media_header: "Property Media Header".to_string(),
                property_media_footer: "Property Media Footer".to_string(),
                image_small_url: "https://image.com/small".to_string(),
                image_large_url: "https://image.com/large".to_string(),
                label: "GROW".to_string(),
                carbon_credits: 200,
                shares_available: 100,
                contract_address: project_contract.0.to_decimal(),
                latest_price,
                geo_spatial_url: Some("https://geo.com/spatial".to_string()),
                offering_doc_link: Some("https://offering.com/doc".to_string()),
                mint_fund_contract_address: Some(mint_fund.0.to_decimal()),
                p2p_trade_contract_address: Some(p2p_trade.0.to_decimal()),
                roi_percent: 12.5,
                state: ForestProjectState::Draft,
                created_at: chain.block_time_naive_utc(),
                updated_at: chain.block_time_naive_utc(),
            })
        })
        .await;
    (project_contract, mint_fund, p2p_trade, project)
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
        .transact(|account| chain.deploy_module(account, SftRewardsTestClient::module()))
        .expect("Failed to deploy security p2p trade module");
}

#[allow(clippy::too_many_arguments)]
pub async fn create_user(
    test_id: &str,
    chain: &mut Chain,
    api: &mut ApiTestClient,
    cognito: &mut CognitoTestClient,
    admin: &UserTestClient,
    euroe: &EuroETestClient,
    identity_registry: &IdentityRegistryTestClient,
    index: u8,
) -> UserTestClient {
    let user = chain.create_account(AccountAddress([100 + index; 32]), DEFAULT_ACCOUNT_BALANCE);
    let user = create_login_api_user(
        api,
        cognito,
        admin,
        format!("user_{}_{}@yopmail.com", index, test_id),
        user,
    )
    .await;
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
    admin
        .transact(|account| {
            chain.update(
                account,
                identity_registry.register_identity_payload(RegisterIdentityParams {
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

pub async fn create_login_api_admin(
    api: &mut ApiTestClient,
    cognito: &mut CognitoTestClient,
    email: String,
    account: Account,
) -> UserTestClient {
    let (user_id, password) = create_api_user(api, cognito, &email).await;
    cognito.admin_add_to_admin_group(&user_id).await;
    let id_token = cognito.user_login(&email, &password).await;
    let admin = UserTestClient {
        id: user_id.clone(),
        email,
        password,
        id_token,
        account_address: "".to_string(),
    };
    admin
        .call_api(|id_token| {
            api.admin_user_update_account_address(id_token, user_id.clone(), account.address_str())
        })
        .await;

    UserTestClient {
        account_address: account.address_str(),
        ..admin
    }
}

pub async fn create_login_api_user(
    api: &mut ApiTestClient,
    cognito: &mut CognitoTestClient,
    admin: &UserTestClient,
    email: String,
    account: Account,
) -> UserTestClient {
    let (user_id, password) = create_api_user(api, cognito, &email).await;
    admin
        .call_api(|id_token| {
            api.admin_user_update_account_address(id_token, user_id.clone(), account.address_str())
        })
        .await
        .assert_status_is_ok();
    let id_token = cognito.user_login(&email, &password).await;

    UserTestClient {
        id: user_id,
        email,
        password,
        id_token,
        account_address: account.address_str(),
    }
}

async fn create_api_user(
    api: &mut ApiTestClient,
    cognito: &mut CognitoTestClient,
    email: &str,
) -> (String, String) {
    let user_id = api.user_send_invitation(email).await;
    let temp_password = PASS_GENERATOR.generate_one().unwrap();
    // This is needed just to ensure that temp passwords match
    // API call sets random passwords for Cognito users (It it set by Cognito)
    cognito
        .admin_set_user_password(&user_id, &temp_password)
        .await;
    let password = PASS_GENERATOR.generate_one().unwrap();
    let id_token = cognito
        .user_change_password(email, &temp_password, &password)
        .await;
    api.user_register(id_token, &UserRegisterReq {
        desired_investment_amount: 100,
    })
    .await;
    (user_id, password)
}
