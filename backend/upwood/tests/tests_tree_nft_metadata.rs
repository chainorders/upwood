mod test_utils;

use std::sync::Arc;

use chrono::{DateTime, NaiveDateTime};
use cis2_conversions::to_token_id_vec;
use compliance::init_nationalities;
use concordium_cis2::{TokenIdUnit, UpdateOperator};
use concordium_protocols::concordium_cis2_security::TokenUId;
use concordium_rust_sdk::types::WalletAccount;
use concordium_smart_contract_testing::*;
use diesel::r2d2::ConnectionManager;
use events_listener::listener::{ContractCall, ParsedBlock, ParsedTxn};
use events_listener::processors::cis2_utils::{ContractAddressToDecimal, TokenIdToDecimal};
use events_listener::processors::Processors;
use integration_tests::*;
use nft_multi_rewarded::types::{Agent, ContractMetadataUrl};
use nft_multi_rewarded::{MintData, SignedMetadata};
use poem::web::Data;
use rust_decimal::Decimal;
use shared::db::txn_listener::ListenerBlock;
use shared::db_setup;
use shared::db_shared::DbPool;
use test_utils::conversions::{to_contract_call_init, to_contract_call_update};
use tracing_test::traced_test;
use upwood::api::tree_nft_metadata::{self, AddMetadataRequest};
use upwood::api::{self, BearerAuthorization, SystemContractsConfig};
use upwood::utils::aws::cognito::Claims;

const ADMIN: AccountAddress = AccountAddress([0; 32]);
const HOLDER: AccountAddress = AccountAddress([1; 32]);
const NFT_AGENT: AccountAddress = AccountAddress([2; 32]);
const DEFAULT_BALANCE_AMOUNT: Amount = Amount::from_micro_ccd(1_000_000_000);
const COMPLIANT_NATIONALITIES: [&str; 3] = ["IN", "US", "GB"];
const REWARD_SFT_METADATA: &str = "https://metadata.com/rewarded_sft_token";
const NFT_METADATA: &str = "https://metadata.com/nft_token";

#[traced_test]
#[tokio::test]
async fn signature_tests() {
    // Setup Database
    let (db_config, _container) = shared_tests::create_new_database_container().await;
    db_setup::run_migrations(&db_config.db_url());
    let pool: DbPool = r2d2::Pool::builder()
        .max_size(10)
        .build(ConnectionManager::new(db_config.db_url()))
        .expect("Error creating database pool");
    let block_duration = Duration::from_seconds(2);
    let mut chain = Chain::new_with_time(Timestamp { millis: 0 });
    let admin = Account::new(ADMIN, DEFAULT_BALANCE_AMOUNT);
    chain.create_account(admin.clone());
    let mut processors = Processors::new(vec![admin.address.to_string()]);
    let mut processor_db_conn = pool.get().expect("db connection");

    // api
    let metadata_admin_api = api::tree_nft_metadata::Api;
    metadata_admin_api
        .metadata_insert(
            BearerAuthorization(Claims {
                sub:            "USER_ID".to_string(),
                email:          "admin@example.com".to_string(),
                cognito_groups: Some(vec!["admin".to_string()]),
                email_verified: Some(true),
                account:        None,
            }),
            Data(&pool),
            poem_openapi::payload::Json(AddMetadataRequest {
                metadata_url:          api::tree_nft_metadata::MetadataUrl {
                    url:  NFT_METADATA.to_string(),
                    hash: None,
                },
                probablity_percentage: 100,
            }),
        )
        .await
        .expect("metadata insert");

    // Setup Chain

    let holder = Account::new(HOLDER, DEFAULT_BALANCE_AMOUNT);
    chain.create_account(holder.clone());

    let agent_account_keys = AccountKeys::singleton(&mut rand::thread_rng());
    let agent = Account::new_with_keys(
        NFT_AGENT,
        AccountBalance::new(DEFAULT_BALANCE_AMOUNT, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::from(&agent_account_keys),
    );
    chain.create_account(agent.clone());

    let contracts = {
        let (contracts, contract_calls) = setup_chain(&mut chain, &admin, &COMPLIANT_NATIONALITIES);
        let block = ParsedBlock {
            block:        ListenerBlock {
                block_slot_time: to_utc(&chain.block_time()),
                block_height:    1.into(),
                block_hash:      [1, 32].to_vec(),
            },
            transactions: vec![ParsedTxn {
                index: 1,
                hash: [1, 32].to_vec(),
                sender: admin.address.to_string(),
                contract_calls,
            }],
        };
        processors
            .process_block(&mut processor_db_conn, &block)
            .await
            .expect("process block");
        contracts
    };

    {
        chain.tick_block_time(block_duration).unwrap();
        let contract_calls = identity_registry::register_nationalities(
            &mut chain,
            &admin,
            &contracts.identity_registry(),
            vec![
                (holder.address.into(), COMPLIANT_NATIONALITIES[0]),
                (contracts.tree_nft().into(), COMPLIANT_NATIONALITIES[0]),
            ],
        )
        .into_iter()
        .map(|res| res.expect("register_nationalities"))
        .flat_map(|res| to_contract_call_update(&res))
        .collect::<Vec<_>>();
        let block = ParsedBlock {
            block:        ListenerBlock {
                block_slot_time: to_utc(&chain.block_time()),
                block_height:    2.into(),
                block_hash:      [2, 32].to_vec(),
            },
            transactions: vec![ParsedTxn {
                index: 1,
                hash: [2, 32].to_vec(),
                sender: admin.address.to_string(),
                contract_calls,
            }],
        };
        processors
            .process_block(&mut processor_db_conn, &block)
            .await
            .expect("process block");
    }

    {
        chain.tick_block_time(block_duration).unwrap();
        let add_agent_res = nft_multi_rewarded_client::add_agent(
            &mut chain,
            &admin,
            contracts.tree_nft(),
            &Agent {
                address: NFT_AGENT.into(),
            },
        )
        .expect("add agent");

        let mint_res = security_sft_single_client::mint(
            &mut chain,
            &admin,
            &contracts.tree_ft(),
            &security_sft_single::types::MintParams {
                token_id: TokenIdUnit(),
                owners:   vec![security_sft_single::types::MintParam {
                    amount:  1.into(),
                    address: holder.address,
                }],
            },
        )
        .expect("sft mint");

        let update_op_res = security_sft_single_client::update_operator_single(
            &mut chain,
            &holder,
            contracts.tree_ft(),
            UpdateOperator {
                update:   concordium_cis2::OperatorUpdate::Add,
                operator: contracts.tree_nft().into(),
            },
        )
        .expect("update operator");

        let block = ParsedBlock {
            block:        ListenerBlock {
                block_slot_time: to_utc(&chain.block_time()),
                block_height:    3.into(),
                block_hash:      [3, 32].to_vec(),
            },
            transactions: vec![
                ParsedTxn {
                    index:          1,
                    hash:           [3, 32].to_vec(),
                    sender:         admin.address.to_string(),
                    contract_calls: to_contract_call_update(&add_agent_res),
                },
                ParsedTxn {
                    index:          2,
                    hash:           [4, 32].to_vec(),
                    sender:         admin.address.to_string(),
                    contract_calls: to_contract_call_update(&mint_res),
                },
                ParsedTxn {
                    index:          3,
                    hash:           [5, 32].to_vec(),
                    sender:         holder.address.to_string(),
                    contract_calls: to_contract_call_update(&update_op_res),
                },
            ],
        };
        processors
            .process_block(&mut processor_db_conn, &block)
            .await
            .expect("process block");
    }

    // Api Interactions
    let tree_nft_config = tree_nft_metadata::TreeNftConfig {
        agent: Arc::new(tree_nft_metadata::TreeNftAgent(WalletAccount {
            address: agent.address,
            keys:    agent_account_keys,
        })),
    };
    let api_user_claims = BearerAuthorization(Claims {
        sub:            "NORMAL_USER_ID".to_string(),
        cognito_groups: None,
        email_verified: Some(true),
        email:          "normal_user@example.com".to_string(),
        account:        Some(holder.address.to_string()),
    });

    let poem_openapi::payload::Json(random_metadata) = tree_nft_metadata::Api
        .metadata_get_random(
            api_user_claims.clone(),
            Data(&pool),
            Data(&tree_nft_config),
            Data(&contracts),
        )
        .await
        .expect("metadata get random");

    {
        let transfer_mint_res = nft_multi_rewarded_client::transfer_mint(
            &mut chain,
            &holder,
            contracts.tree_nft(),
            &MintData {
                signed_metadata: SignedMetadata {
                    account:          holder.address,
                    account_nonce:    0,
                    contract_address: contracts.tree_nft(),
                    metadata_url:     nft_multi_rewarded::MetadataUrl {
                        url:  random_metadata.signed_metadata.metadata_url.url,
                        hash: None,
                    },
                },
                signer:          agent.address,
                signature:       serde_json::from_value(random_metadata.signature)
                    .expect("signature deserialization"),
            },
        )
        .expect("mint");
        let block = ParsedBlock {
            block:        ListenerBlock {
                block_slot_time: to_utc(&chain.block_time()),
                block_height:    4.into(),
                block_hash:      [4, 32].to_vec(),
            },
            transactions: vec![ParsedTxn {
                index:          1,
                hash:           [6, 32].to_vec(),
                sender:         holder.address.to_string(),
                contract_calls: to_contract_call_update(&transfer_mint_res),
            }],
        };
        processors
            .process_block(&mut processor_db_conn, &block)
            .await
            .expect("process block");
    }

    let balance: nft_multi_rewarded::types::TokenAmount =
        nft_multi_rewarded_client::balance_of_single(
            &mut chain,
            &holder,
            contracts.tree_nft(),
            concordium_cis2::TokenIdU64(0),
            holder.address.into(),
            nft_multi_rewarded_client::CONTRACT_NAME,
        )
        .expect("balance of");
    assert_eq!(balance, 1.into());

    let poem_openapi::payload::Json(nonce) = api::tree_nft::Api
        .nonce(api_user_claims, Data(&pool), Data(&contracts))
        .await
        .expect("nonce");
    assert_eq!(nonce, 1);
}

fn setup_chain(
    chain: &mut Chain,
    admin: &Account,
    compliant_nationalities: &[&str],
) -> (SystemContractsConfig, Vec<ContractCall>) {
    chain.create_account(admin.clone());

    euroe::deploy_module(chain, admin);
    identity_registry::deploy_module(chain, admin);
    compliance::deploy_module(chain, admin);
    security_sft_single_client::deploy_module(chain, admin);
    nft_multi_rewarded_client::deploy_module(chain, admin);

    let (ir_contract, ir_module_ref, ir_contract_name) =
        identity_registry::init(chain, admin).expect("Error initializing identity registry");
    let ir_contract_call = to_contract_call_init(&ir_contract, ir_module_ref, ir_contract_name);

    let (compliance_module, mod_module_ref, mod_contract_name) = init_nationalities(
        chain,
        admin,
        &concordium_rwa_compliance::compliance_modules::allowed_nationalities::init::InitParams {
            nationalities:     compliant_nationalities
                .iter()
                .map(|n| n.to_string())
                .collect(),
            identity_registry: ir_contract.contract_address,
        },
    )
    .expect("init nationalities module");
    let compliance_module_call =
        to_contract_call_init(&compliance_module, mod_module_ref, mod_contract_name);
    let (compliance, compliance_module_ref, compliance_contract_name) =
        compliance::init(chain, admin, vec![compliance_module.contract_address])
            .expect("init compliance module");
    let compliance_contract_call =
        to_contract_call_init(&compliance, compliance_module_ref, compliance_contract_name);

    let (euroe, euroe_module_ref, euroe_contract_name) =
        euroe::init(chain, admin).expect("euroe init");
    let euroe_contract_call = to_contract_call_init(&euroe, euroe_module_ref, euroe_contract_name);

    let (sft, sft_module_ref, sft_contract_name) =
        security_sft_single_client::init(chain, admin, &security_sft_single::types::InitParam {
            compliance:        compliance.contract_address,
            identity_registry: ir_contract.contract_address,
            sponsors:          None,
            metadata_url:      ContractMetadataUrl {
                url:  REWARD_SFT_METADATA.to_string(),
                hash: None,
            },
        })
        .expect("sft init");
    let sft_contract_call = to_contract_call_init(&sft, sft_module_ref, sft_contract_name);
    let (nft, nft_module_ref, nft_contract_name) =
        nft_multi_rewarded_client::init(chain, admin, &nft_multi_rewarded::types::InitParam {
            reward_token: TokenUId {
                contract: sft.contract_address,
                id:       to_token_id_vec(TokenIdUnit()),
            },
        })
        .expect("nft init");
    let nft_contract_call = to_contract_call_init(&nft, nft_module_ref, nft_contract_name);
    let contract_calls = vec![
        ir_contract_call,
        compliance_module_call,
        compliance_contract_call,
        euroe_contract_call,
        sft_contract_call,
        nft_contract_call,
    ];

    let grant_role_calls = to_contract_call_update(
        &euroe::grant_role(chain, admin, euroe.contract_address, &euroe::RoleTypes {
            mintrole:  admin.address.into(),
            burnrole:  admin.address.into(),
            blockrole: admin.address.into(),
            pauserole: admin.address.into(),
            adminrole: admin.address.into(),
        })
        .expect("grant role"),
    );
    let contract_calls = contract_calls
        .into_iter()
        .chain(grant_role_calls)
        .collect::<Vec<_>>();

    let system_contracts = SystemContractsConfig {
        carbon_credit_contract_index:     Decimal::ZERO,
        carbon_credit_token_id:           Decimal::ZERO,
        compliance_contract_index:        compliance.contract_address.to_decimal(),
        euro_e_contract_index:            euroe.contract_address.to_decimal(),
        euro_e_token_id:                  TokenIdUnit().to_decimal(),
        identity_registry_contract_index: ir_contract.contract_address.to_decimal(),
        tree_ft_contract_index:           sft.contract_address.to_decimal(),
        tree_nft_contract_index:          nft.contract_address.to_decimal(),
    };

    (system_contracts, contract_calls)
}

fn to_utc(timestamp: &Timestamp) -> NaiveDateTime {
    DateTime::from_timestamp_millis(timestamp.timestamp_millis() as i64)
        .expect("block_time_to_utc conversion")
        .naive_utc()
}
