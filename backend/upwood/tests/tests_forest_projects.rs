//! This is meant to be a complete test of the forest project.
//! It is meant to be run with `cargo test --release -- --test tests_forest_projects`
#![feature(assert_matches)]
mod test_utils;

use chrono::{DateTime, Utc};
use concordium_cis2::{
    AdditionalData, OperatorUpdate, Receiver, TokenAmountU64, TokenIdU32, TokenIdUnit, TokenIdVec,
    Transfer, TransferParams, UpdateOperator, UpdateOperatorParams,
};
use concordium_protocols::concordium_cis2_security::{Identity, TokenUId};
use concordium_protocols::rate::Rate;
use concordium_rwa_identity_registry::identities::RegisterIdentityParams;
use concordium_rwa_identity_registry::types::IdentityAttribute;
use concordium_smart_contract_testing::{AccountAddress, Amount, ContractAddress, Energy};
use diesel::r2d2::ConnectionManager;
use events_listener::processors::cis2_utils::ContractAddressToDecimal;
use events_listener::processors::Processors;
use integration_tests::cis2_conversions::to_token_id_vec;
use integration_tests::compliance::{ComplianceTestClient, NationalitiesModuleTestClient};
use integration_tests::euroe::{EuroETestClient, RoleTypes};
use integration_tests::identity_registry::IdentityRegistryTestClient;
use integration_tests::nft_multi_rewarded_client::NftMultiRewardedTestClient;
use integration_tests::offchain_rewards_client::OffchainRewardsTestClient;
use integration_tests::security_mint_fund_client::MintFundTestClient;
use integration_tests::security_p2p_trading_client::P2PTradeTestClient;
use integration_tests::security_sft_rewards_client::SftRewardsTestClient;
use integration_tests::security_sft_single_client::SftSingleTestClient;
use passwords::PasswordGenerator;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use security_mint_fund::{ClaimInvestmentParam, FundState, TransferInvestParams};
use security_sft_rewards::rewards::ClaimRewardsParam;
use security_sft_rewards::types::TRACKED_TOKEN_ID;
use shared::db::security_mint_fund::SecurityMintFundState;
use shared::db_app::forest_project::{
    ForestProject, ForestProjectHolderRewardTotal, ForestProjectSeller, ForestProjectState,
    ForestProjectUserHolderReward, HolderReward, UserTransaction,
};
use shared::db_shared::{DbConn, DbPool};
use test_log::test;
use test_utils::test_api::ApiTestClient;
use test_utils::test_chain::{Account, Chain};
use test_utils::test_cognito::CognitoTestClient;
use test_utils::test_user::UserTestClient;
use upwood::api;
use upwood::api::investment_portfolio::InvestmentPortfolioUserAggregate;
use upwood::api::user::{
    UserRegisterReq, UserRegistrationInvitationSendReq, UserUpdateAccountAddressRequest,
};
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
    let start_time = DateTime::parse_from_rfc3339("2021-01-01T00:00:00Z")
        .expect("Failed to parse start time")
        .with_timezone(&Utc);
    let mut chain = Chain::new(start_time);
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
                    compliance:        None,
                    identity_registry: None,
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
                    compliance:        Some(compliance_contract.0),
                    identity_registry: Some(identity_registry.0),
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

    // Setting up euro contract
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
        offchain_rewards_contract_index: offchain_rewards.0.to_decimal(),
        filebase_access_key_id: "".into(),
        filebase_secret_access_key: "".into(),
        files_presigned_url_expiry_secs: 0,
        tree_nft_agent_wallet_json_str: AGENT_WALLET_JSON_STR.to_string(),
        offchain_rewards_agent_wallet_json_str: AGENT_WALLET_JSON_STR.to_string(),
        user_challenge_expiry_duration_mins: 0,
        affiliate_commission: Decimal::from_f64(0.05).unwrap(),
    };
    let offchain_agent_account_address = api_config.offchain_rewards_agent_wallet().address;
    let offchain_agent_account_keys = api_config.offchain_rewards_agent_wallet().keys;

    let mut api = ApiTestClient::new(api_config.clone()).await;
    let aws_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let mut cognito = CognitoTestClient::new(
        &aws_config,
        api_config.aws_user_pool_id,
        api_config.aws_user_pool_client_id,
    );

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
        None,
        Decimal::from_f64(0.05).unwrap(), // charging 5% for user affiliations
        1,
    )
    .await;

    let user_2 = create_user(
        &test_id,
        &mut chain,
        &mut api,
        &mut cognito,
        &admin,
        Some(user_1.account_address.clone()),
        0.into(),
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
    // This is needed so that Forest Project Contract can hold Tree SFT Rewards
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
                    chain.update_with_energy(
                        account,
                        mint_fund_1.transfer_invest_payload(&TransferInvestParams {
                            amount: TokenAmountU64(100),
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
                    chain.update_with_energy(
                        account,
                        mint_fund_1.transfer_invest_payload(&TransferInvestParams {
                            amount: TokenAmountU64(200),
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
                    api.portfolio_aggreagte(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
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
                    chain.update_with_energy(
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
                    api.portfolio_aggreagte(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
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
                    api.portfolio_aggreagte(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
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
            .call_api(|token| api.portfolio_aggreagte(token, Some(chain.block_time_naive_utc())))
            .await;
        assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
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
                chain.update_with_energy(
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
                    Energy { energy: 30_000 },
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
                chain.update_with_energy(
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
                    Energy { energy: 40_000 },
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

    // user 1 claim euro e rewards and assert
    {
        // claiming euro e rewards
        {
            user_1.transact(|sender| {
                chain.update(sender, fp_1_contract.claim_rewards_payload(&security_sft_rewards::rewards::ClaimRewardsParams {
                    owner: Receiver::Account(sender),
                    claims: vec![ClaimRewardsParam {
                        token_id: 1.into(), // received from `forest_project_rewards_claimable` call
                    }]
                }))
            }).expect("Failed to claim euro e rewards for user 1");

            processor
                .process_block(&mut db_conn, &chain.produce_block())
                .await
                .expect("Error processing block");
        }

        // Asserting total rewards for user 1
        {
            let rewards_total = user_1
                .call_api(|id_token| api.forest_project_rewards_total(id_token))
                .await;
            assert_eq!(rewards_total, vec![
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
        }

        // Asserting claimable rewards for user 1
        {
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
            assert_eq!(
                user_1_owned_fps.data[0]
                    .holder_rewards_parsed()
                    .expect("error parsing holder rewards"),
                vec![
                    ForestProjectUserHolderReward {
                        rewarded_token_contract: carbon_credits.0.to_decimal(),
                        rewarded_token_id:       0.into(),
                        total_frozen_reward:     Decimal::ZERO,
                        total_un_frozen_reward:  50000.into(),
                    },
                    ForestProjectUserHolderReward {
                        rewarded_token_contract: tree_sft.0.to_decimal(),
                        rewarded_token_id:       0.into(),
                        total_frozen_reward:     Decimal::ZERO,
                        total_un_frozen_reward:  100.into(),
                    }
                ]
            );
        }
    }

    // user 1 burns claimed carbon credits
    {
        // claiming carbon credit rewards
        {
            user_1.transact(|sender| {
                chain.update(sender, fp_1_contract.claim_rewards_payload(&security_sft_rewards::rewards::ClaimRewardsParams {
                    owner: Receiver::Account(sender),
                    claims: vec![ClaimRewardsParam {
                        token_id: 2.into(), // received from `forest_project_rewards_claimable` call
                    }]
                }))
            }).expect("Failed to claim carbon credit rewards for user 1");
        }

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
                    api.portfolio_aggreagte(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
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
                chain.update_with_energy(
                    sender,
                    fp_1_contract
                        .cis2()
                        .transfer_payload(&TransferParams(vec![Transfer {
                            amount:   TokenAmountU64(50),
                            token_id: TokenIdU32(0),
                            to:       user_2.address().into(),
                            from:     sender.into(),
                            data:     AdditionalData::empty(),
                        }])),
                    Energy { energy: 15_000 },
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
                    api.portfolio_aggreagte(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            // here only the `current_portfolio_value` has changed
            // because transfers are assumed to be done at market rate / forest project price
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
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
                .call_api(|id_token| api.forest_project_rewards_total(id_token))
                .await;
            assert_eq!(rewards_total, vec![ForestProjectHolderRewardTotal {
                holder_address:          user_1.address().to_string(),
                rewarded_token_id:       0.into(),
                rewarded_token_contract: tree_sft.0.to_decimal(),
                total_frozen_reward:     Decimal::ZERO,
                total_un_frozen_reward:  100.into(),
            },]);
        }

        // Asserting claimable rewards for user 1
        // These are unchanged because because on transferring tokens the rewards are not transferred
        {
            let rewards_claimable = user_1
                .call_api(|id_token| api.forest_project_rewards_claimable(id_token))
                .await;
            assert_eq!(rewards_claimable, vec![HolderReward {
                id: fp_1.id,
                contract_address: fp_1.contract_address,
                holder_address: user_1.account_address.clone(),
                token_id: 3.into(),
                rewarded_token_id: 0.into(),
                rewarded_token_contract: tree_sft.0.to_decimal(),
                frozen_balance: Decimal::ZERO,
                frozen_reward: Decimal::ZERO,
                un_frozen_balance: 100.into(),
                un_frozen_reward: 100.into(),
            }]);
        }
    }

    // user 1 open a sell position of p2p trade contract
    {
        // Asserting portfolio values BEFORE opening sell position
        {
            let portfolio = user_1
                .call_api(|token| {
                    api.portfolio_aggreagte(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
                locked_mint_fund_euro_e_amount: 0.into(),
                invested_value:                 100.into(),
                current_portfolio_value:        700.into(),
                yearly_return:                  1200.into(),
                monthly_return:                 100.into(),
                return_on_investment:           1300.into(),
                carbon_tons_offset:             1000.into(),
            });
        }

        // open a sell position
        {
            user_1
                .transact(|sender| {
                    chain.update(
                        sender,
                        fp_1_contract
                            .cis2()
                            .update_operator_payload(&UpdateOperatorParams(vec![UpdateOperator {
                                operator: p2p_trade_1.0.into(),
                                update:   OperatorUpdate::Add,
                            }])),
                    )
                })
                .expect("Failed to update p2p trade operator for forest project 1 for user 1");

            user_1
                .transact(|sender| {
                    chain.update(
                        sender,
                        p2p_trade_1.transfer_sell_payload(
                            &security_p2p_trading::TransferSellParams {
                                amount: TokenAmountU64(50),
                                rate:   Rate::new(1, 2).unwrap(),
                            },
                        ),
                    )
                })
                .expect("Failed to open sell position for user 1");

            processor
                .process_block(&mut db_conn, &chain.produce_block())
                .await
                .expect("Error processing block");
        }

        // Asserting portfolio values AFTER opening sell position
        {
            let portfolio = user_1
                .call_api(|token| {
                    api.portfolio_aggreagte(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
                locked_mint_fund_euro_e_amount: 0.into(),
                invested_value:                 100.into(),
                current_portfolio_value:        0.into(), /* all the shares are locked in sell position */
                yearly_return:                  1200.into(),
                monthly_return:                 100.into(),
                return_on_investment:           1300.into(),
                carbon_tons_offset:             1000.into(),
            });
        }

        // cancel sell postion
        {
            user_1
                .transact(|sender| chain.update(sender, p2p_trade_1.cancel_sell_payload()))
                .expect("Failed to cancel sell position for user 1");
            processor
                .process_block(&mut db_conn, &chain.produce_block())
                .await
                .expect("Error processing block");
        }

        // Asserting portfolio values after cancel sell
        {
            let portfolio = user_1
                .call_api(|token| {
                    api.portfolio_aggreagte(token, Some(chain.block_time_naive_utc()))
                })
                .await;
            assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
                locked_mint_fund_euro_e_amount: 0.into(),
                invested_value:                 100.into(),
                current_portfolio_value:        700.into(),
                yearly_return:                  1200.into(),
                monthly_return:                 100.into(),
                return_on_investment:           1300.into(),
                carbon_tons_offset:             1000.into(),
            });
        }
    }

    let forest_project = user_2
        .call_api(|token| api.forest_project_find(token, fp_1.id))
        .await;
    assert_eq!(forest_project.latest_price, 14.into());

    // user2 opens a sell position of p2p trade contract & user 1 buys it
    {
        user_2
            .transact(|sender| {
                chain.update(
                    sender,
                    fp_1_contract
                        .cis2()
                        .update_operator_payload(&UpdateOperatorParams(vec![UpdateOperator {
                            operator: p2p_trade_1.0.into(),
                            update:   OperatorUpdate::Add,
                        }])),
                )
            })
            .expect("Failed to update p2p trade operator for forest project 1 for user 2");

        user_2
            .transact(|sender| {
                chain.update(
                    sender,
                    p2p_trade_1.transfer_sell_payload(&security_p2p_trading::TransferSellParams {
                        amount: TokenAmountU64(50),
                        rate:   Rate::new(28, 1).unwrap(), // double the price
                    }),
                )
            })
            .expect("Failed to open sell position for user 2");
        processor
            .process_block(&mut db_conn, &chain.produce_block())
            .await
            .expect("Error processing block");
    }

    // user 1 buys / exchanges the sell position
    {
        // user 1 discovers the sell position of user 2
        let sellers = user_1
            .call_api(|token| api.forest_project_p2p_trade_sellers_list(token, fp_1.id, 0))
            .await;
        assert_eq!(sellers.data.len(), 1);
        assert_eq!(sellers.data[0], ForestProjectSeller {
            currency_token_id: 0.into(),
            currency_token_contract_address: euroe.0.to_decimal(),
            p2p_trade_contract_address: p2p_trade_1.0.to_decimal(),
            rate: 28.into(),
            token_amount: 50.into(),
            forest_project_id: fp_1.id,
            forest_project_state: ForestProjectState::Listed,
            trader_address: user_2.account_address.clone(),
        });

        let seller = &sellers.data[0];
        user_1
            .transact(|sender| {
                chain.update(
                    sender,
                    euroe
                        .cis2()
                        .update_operator_payload(&UpdateOperatorParams(vec![UpdateOperator {
                            operator: p2p_trade_1.0.into(),
                            update:   OperatorUpdate::Add,
                        }])),
                )
            })
            .expect("Failed to update euroe operator for p2p trade contract for user 1");

        user_1
            .transact(|sender| {
                chain.update_with_energy(
                    sender,
                    p2p_trade_1.transfer_exchange_payload(
                        &security_p2p_trading::TransferExchangeParams {
                            pay: TokenAmountU64(28 * 50), // 28*50 : buying 50 units at double market price = 1400
                            get: security_p2p_trading::ExchangeParams {
                                from:   seller
                                    .trader_address
                                    .parse()
                                    .expect("Failed to parse address"),
                                amount: 50.into(), // buying 50 units
                            },
                        },
                    ),
                    Energy { energy: 30_000 },
                )
            })
            .expect("Failed to exchange sell position for user 1");
        processor
            .process_block(&mut db_conn, &chain.produce_block())
            .await
            .expect("Error processing block");

        let portfolio = user_1
            .call_api(|token| api.portfolio_aggreagte(token, Some(chain.block_time_naive_utc())))
            .await;
        assert_eq!(portfolio, InvestmentPortfolioUserAggregate {
            locked_mint_fund_euro_e_amount: 0.into(),
            invested_value:                 1500.into(), /* 100 for mint fund and 1400 for buying 50 units at 28 price */
            current_portfolio_value:        1400.into(), // 100 shares at 14 price
            yearly_return:                  1200.into(),
            monthly_return:                 100.into(),
            return_on_investment:           ((Decimal::from_u64(1400).unwrap()
                - Decimal::from_u64(1500).unwrap())
                / Decimal::from_u64(1500).unwrap())
                * Decimal::from_u64(100).unwrap(), // (1400 - 1500 / 1500) * 100
            carbon_tons_offset:             1000.into(),
        });
    }

    // user 1 transactions assertions
    {
        let user_1_txns = user_1
            .call_api(|token| api.txn_history_list(token, 0))
            .await;
        assert_eq!(user_1_txns.data, vec![
            UserTransaction {
                cognito_user_id: user_1.id.clone(),
                txn_hash:        "6b0e4bfa8dc9b19ccc121b2bd0d22da7298d7a4d72b4ba1df1bbec5d56800a66"
                    .to_string(),
                txn_type:        "p2p_trading".to_string(),
                txn_subtype:     "exchange_buy".to_string(),
                project_id:      fp_1.id,
                account_address: user_1.account_address.clone(),
                currency_amount: Decimal::from(1400),
                txn_time:        DateTime::parse_from_rfc3339("2022-01-11T00:00:28Z")
                    .unwrap()
                    .naive_utc(),
            },
            UserTransaction {
                cognito_user_id: user_1.id.clone(),
                txn_hash:        "d596e806d34bd7d4304b9f00ec340f02fa90c4faa17fe39729cb15c76e0de77f"
                    .to_string(),
                txn_type:        "p2p_trading".to_string(),
                txn_subtype:     "sell_cancel".to_string(),
                project_id:      fp_1.id,
                account_address: user_1.account_address.clone(),
                currency_amount: Decimal::from(25),
                txn_time:        DateTime::parse_from_rfc3339("2022-01-11T00:00:24Z")
                    .unwrap()
                    .naive_utc(),
            },
            UserTransaction {
                cognito_user_id: user_1.id.clone(),
                txn_hash:        "ae3c8b8d99a39542f78af83dbbb42c81cd94199ec1b5f60a0801063e95842570"
                    .to_string(),
                txn_type:        "p2p_trading".to_string(),
                txn_subtype:     "sell".to_string(),
                project_id:      fp_1.id,
                account_address: user_1.account_address.clone(),
                currency_amount: Decimal::from(25),
                txn_time:        DateTime::parse_from_rfc3339("2022-01-11T00:00:22Z")
                    .unwrap()
                    .naive_utc(),
            },
            UserTransaction {
                cognito_user_id: user_1.id.clone(),
                txn_hash:        "fe5b0deca006174e2151088e1a7d60fe91819467db5d709ae5a19c01a3cd3e3f"
                    .to_string(),
                txn_type:        "mint_fund".to_string(),
                txn_subtype:     "claimed".to_string(),
                project_id:      fp_1.id,
                account_address: user_1.account_address.clone(),
                currency_amount: Decimal::from(100),
                txn_time:        DateTime::parse_from_rfc3339("2021-01-01T00:00:12Z")
                    .unwrap()
                    .naive_utc(),
            },
            UserTransaction {
                cognito_user_id: user_1.id.clone(),
                txn_hash:        "688e94a51ee508a95e761294afb7a6004b432c15d9890c80ddf23bde8caa4c26"
                    .to_string(),
                txn_type:        "mint_fund".to_string(),
                txn_subtype:     "invested".to_string(),
                project_id:      fp_1.id,
                account_address: user_1.account_address.clone(),
                currency_amount: Decimal::from(100),
                txn_time:        DateTime::parse_from_rfc3339("2021-01-01T00:00:06Z")
                    .unwrap()
                    .naive_utc(),
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
                    euroe
                        .cis2()
                        .update_operator_payload(&UpdateOperatorParams(vec![UpdateOperator {
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
                .call_api(|token| api.user_affiliate_rewards_list(token, 0))
                .await;
            assert_eq!(affiliate_rewards.data.len(), 1);
            assert_eq!(affiliate_rewards.data[0].currency_amount, 200.into()); // Amount invested by user 2 in euroe
            assert_eq!(
                affiliate_rewards.data[0].affiliate_commission,
                Decimal::from_f64(0.05).unwrap()
            );
            assert_eq!(affiliate_rewards.data[0].reward_amount, 10.into());
            assert_eq!(affiliate_rewards.data[0].remaining_reward_amount, None);
            assert_eq!(
                affiliate_rewards.data[0].affiliate_account_address,
                user_1.account_address
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
                .call_api(|token| api.user_affiliate_rewards_list(token, 0))
                .await;
            assert_eq!(affiliate_rewards.data.len(), 1);
            assert_eq!(affiliate_rewards.data[0].currency_amount, 200.into());
            assert_eq!(
                affiliate_rewards.data[0].affiliate_commission,
                Decimal::from_f64(0.05).unwrap()
            );
            assert_eq!(affiliate_rewards.data[0].reward_amount, 10.into());
            // remaining reward amount should be 0 after claiming
            assert_eq!(
                affiliate_rewards.data[0].remaining_reward_amount,
                Some(0.into())
            );
            assert_eq!(
                affiliate_rewards.data[0].affiliate_account_address,
                user_1.account_address
            );
        }
    }
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
                    compliance:        Some(compliance_contract.0),
                    identity_registry: Some(identity_registry.0),
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
    // Adding P2P trade contract as verified in identity registry
    // So that users can transfer the tokens to the contract in order to trade / sell
    admin
        .transact(|sender| {
            chain.update(
                sender,
                identity_registry.register_identity_payload(RegisterIdentityParams {
                    address:  p2p_trade.0.into(),
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
        .expect("Failed to add p2p trade as identity to identity registry");
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
    chain_admin
        .transact(|account| chain.deploy_module(account, OffchainRewardsTestClient::module()))
        .expect("Failed to deploy offchain rewards module");
}

#[allow(clippy::too_many_arguments)]
pub async fn create_user(
    test_id: &str,
    chain: &mut Chain,
    api: &mut ApiTestClient,
    cognito: &mut CognitoTestClient,
    admin: &UserTestClient,
    affiliate_account_address: Option<String>,
    user_affiliate_fees: Decimal,
    index: u8,
) -> UserTestClient {
    let user = chain.create_account(AccountAddress([100 + index; 32]), DEFAULT_ACCOUNT_BALANCE);
    let user = create_login_api_user(
        api,
        cognito,
        admin,
        format!("user_{}_{}@yopmail.com", index, test_id),
        user,
        affiliate_account_address,
        user_affiliate_fees,
    )
    .await;

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
    let (user_id, password) = create_api_user(api, cognito, &email, None).await;
    cognito.admin_add_to_admin_group(&user_id).await;
    let id_token = cognito.user_login(&email, &password).await;
    let admin = UserTestClient {
        id: user_id.clone(),
        email,
        password,
        id_token,
        account_address: "".to_string(),
    };

    let update_account_req = UserUpdateAccountAddressRequest {
        account_address:      account.address_str().clone(),
        affiliate_commission: Decimal::ZERO,
    };
    admin
        .call_api(|id_token| {
            api.admin_update_account_address(id_token, user_id.clone(), &update_account_req)
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
    user_account: Account,
    affiliate_account_address: Option<String>,
    user_affiliate_fees: Decimal,
) -> UserTestClient {
    let (user_id, password) =
        create_api_user(api, cognito, &email, affiliate_account_address).await;
    let update_account_req = UserUpdateAccountAddressRequest {
        account_address:      user_account.address_str().clone(),
        affiliate_commission: user_affiliate_fees,
    };
    // This api call is needed because current its not possible to mock concordium browser wallet
    admin
        .call_api(|id_token| {
            api.admin_update_account_address(id_token, user_id.clone(), &update_account_req)
        })
        .await
        .assert_status_is_ok();
    let id_token = cognito.user_login(&email, &password).await;

    UserTestClient {
        id: user_id,
        email,
        password,
        id_token,
        account_address: user_account.address_str(),
    }
}

async fn create_api_user(
    api: &mut ApiTestClient,
    cognito: &mut CognitoTestClient,
    email: &str,
    affiliate_account_address: Option<String>,
) -> (String, String) {
    let user_id = api
        .user_send_invitation(&UserRegistrationInvitationSendReq {
            email: email.to_string(),
            affiliate_account_address,
        })
        .await;
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
