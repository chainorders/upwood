mod test_utils;

use std::sync::Arc;

use chrono::{DateTime, NaiveDateTime};
use cis2_security::Cis2SecurityTestClient;
use concordium_cis2::TokenIdUnit;
use concordium_protocols::concordium_cis2_security::{
    AgentWithRoles, TokenAmountSecurity, TokenUId,
};
use concordium_rust_sdk::types::WalletAccount;
use concordium_smart_contract_testing::*;
use contract_base::{ContractPayloads, ContractTestClient};
use diesel::r2d2::ConnectionManager;
use events_listener::listener::{ContractCall, ParsedBlock, ParsedTxn};
use events_listener::processors::cis2_utils::{ContractAddressToDecimal, TokenIdToDecimal};
use events_listener::processors::Processors;
use integration_tests::cis2_security::Cis2SecurityPayloads;
use integration_tests::*;
use nft_multi_rewarded::types::{Agent, ContractMetadataUrl};
use nft_multi_rewarded::{MintAgentParams, SignedMetadata};
use nft_multi_rewarded_client::{NftMultiRewardedClientPayloads, NftMultiRewardedTestClient};
use poem::web::Data;
use poem_openapi::payload::Json;
use rust_decimal::Decimal;
use security_sft_single::types::AgentRole;
use security_sft_single_client::SftSingleTestClient;
use shared::db::txn_listener::ListenerBlock;
use shared::db_setup;
use shared::db_shared::DbPool;
use test_utils::conversions::{to_contract_call_init, to_contract_call_update};
use tracing_test::traced_test;
use upwood::api::tree_nft::{self, AddMetadataRequest};
use upwood::api::{self, BearerAuthorization, SystemContractsConfig};
use upwood::utils::aws::cognito::Claims;

const ADMIN: AccountAddress = AccountAddress([0; 32]);
const HOLDER: AccountAddress = AccountAddress([1; 32]);
const NFT_AGENT: AccountAddress = AccountAddress([2; 32]);
const DEFAULT_BALANCE_AMOUNT: Amount = Amount::from_micro_ccd(1_000_000_000);
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
    let metadata_admin_api = api::tree_nft::Api;
    let admin_api_claims = BearerAuthorization(Claims {
        sub:               "USER_ID".to_string(),
        email:             "admin@example.com".to_string(),
        cognito_groups:    Some(vec!["admin".to_string()]),
        email_verified:    Some(true),
        account:           None,
        first_name:        None,
        last_name:         None,
        nationality:       None,
        affiliate_account: None,
        company_id:        None,
    });

    metadata_admin_api
        .admin_tree_nft_metadata_insert(
            admin_api_claims.clone(),
            Data(&pool),
            poem_openapi::payload::Json(AddMetadataRequest {
                metadata_url:          api::tree_nft::MetadataUrl {
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
        let (contracts, contract_calls) = setup_chain(&mut chain, &admin);
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
    let tree_ft_contract = SftSingleTestClient::new(contracts.tree_ft());
    let tree_nft_contract = NftMultiRewardedTestClient::new(contracts.tree_nft());

    {
        chain.tick_block_time(block_duration).unwrap();
        let add_nft_agent_res = chain
            .contract_update(
                Signer::with_one_key(),
                admin.address,
                admin.address.into(),
                30_000.into(),
                tree_nft_contract.add_agent_payload(&Agent {
                    address: NFT_AGENT.into(),
                }),
            )
            .map(|call| to_contract_call_update(&call))
            .expect("add nft agent");
        let add_ft_agent_res = chain
            .contract_update(
                Signer::with_one_key(),
                admin.address,
                admin.address.into(),
                30_000.into(),
                tree_ft_contract.add_agent_payload(&AgentWithRoles {
                    address: tree_nft_contract.contract_address().into(),
                    roles:   vec![AgentRole::Operator],
                }),
            )
            .map(|call| to_contract_call_update(&call))
            .expect("add ft agent");

        let mint_res = tree_ft_contract
            .mint(
                &mut chain,
                &admin,
                &security_sft_single::types::MintParams {
                    token_id: TokenIdUnit(),
                    owners:   vec![security_sft_single::types::MintParam {
                        amount:  TokenAmountSecurity::new_un_frozen(2.into()),
                        address: holder.address.into(),
                    }],
                },
            )
            .map(|call| to_contract_call_update(&call))
            .expect("sft mint");
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
                    contract_calls: add_nft_agent_res,
                },
                ParsedTxn {
                    index:          2,
                    hash:           [4, 32].to_vec(),
                    sender:         admin.address.to_string(),
                    contract_calls: mint_res,
                },
                ParsedTxn {
                    index:          3,
                    hash:           [5, 32].to_vec(),
                    sender:         admin.address.to_string(),
                    contract_calls: add_ft_agent_res,
                },
            ],
        };
        processors
            .process_block(&mut processor_db_conn, &block)
            .await
            .expect("process block");
    }

    // Api Interactions
    let tree_nft_config = tree_nft::TreeNftConfig {
        agent: Arc::new(tree_nft::TreeNftAgent(WalletAccount {
            address: agent.address,
            keys:    agent_account_keys,
        })),
    };
    let api_user_claims = BearerAuthorization(Claims {
        sub:               "NORMAL_USER_ID".to_string(),
        cognito_groups:    None,
        email_verified:    Some(true),
        email:             "normal_user@example.com".to_string(),
        account:           Some(holder.address.to_string()),
        first_name:        None,
        last_name:         None,
        nationality:       None,
        affiliate_account: None,
        company_id:        None,
    });

    let mint_res = {
        let Json(random_metadata) = tree_nft::Api
            .get_metadata_random_signed(
                api_user_claims.clone(),
                Data(&pool),
                Data(&tree_nft_config),
                Data(&contracts),
            )
            .await
            .expect("metadata get random signed");

        chain
            .contract_update(
                Signer::with_one_key(),
                holder.address,
                holder.address.into(),
                30_000.into(),
                tree_nft_contract.mint_payload(&nft_multi_rewarded::MintParams {
                    signed_metadata: SignedMetadata {
                        account:          holder.address,
                        account_nonce:    0,
                        contract_address: contracts.tree_nft(),
                        metadata_url:     nft_multi_rewarded::MetadataUrl {
                            url:  random_metadata.signed_metadata.metadata_url.url,
                            hash: random_metadata.signed_metadata.metadata_url.hash.map(|h| {
                                hex::decode(h)
                                    .expect("hash decode")
                                    .try_into()
                                    .expect("hash")
                            }),
                        },
                    },
                    signer:          agent.address,
                    signature:       serde_json::from_value(random_metadata.signature)
                        .expect("signature deserialization"),
                }),
            )
            .map(|call| to_contract_call_update(&call))
            .expect("mint")
    };

    let mint_agent_res = {
        let Json(random_metadata) = tree_nft::Api
            .get_metadata_random_unsigned(admin_api_claims.clone(), Data(&pool))
            .await
            .expect("metadata get random unsigned");

        chain
            .contract_update(
                Signer::with_one_key(),
                agent.address,
                agent.address.into(),
                30_000.into(),
                tree_nft_contract.mint_agent_payload(&MintAgentParams {
                    account:      holder.address,
                    metadata_url: ContractMetadataUrl {
                        url:  random_metadata.url,
                        hash: random_metadata.hash.map(|h| {
                            hex::decode(h)
                                .expect("hash decode")
                                .try_into()
                                .expect("hash")
                        }),
                    },
                }),
            )
            .map(|call| to_contract_call_update(&call))
            .expect("agent mint")
    };

    {
        let block = ParsedBlock {
            block:        ListenerBlock {
                block_slot_time: to_utc(&chain.block_time()),
                block_height:    4.into(),
                block_hash:      [4, 32].to_vec(),
            },
            transactions: vec![
                ParsedTxn {
                    index:          1,
                    hash:           [6, 32].to_vec(),
                    sender:         holder.address.to_string(),
                    contract_calls: mint_res,
                },
                ParsedTxn {
                    index:          1,
                    hash:           [7, 32].to_vec(),
                    sender:         holder.address.to_string(),
                    contract_calls: mint_agent_res,
                },
            ],
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
        .tree_nft_contract_self_nonce(api_user_claims, Data(&pool), Data(&contracts))
        .await
        .expect("nonce");
    assert_eq!(nonce, 2);
}

fn setup_chain(chain: &mut Chain, admin: &Account) -> (SystemContractsConfig, Vec<ContractCall>) {
    chain.create_account(admin.clone());

    security_sft_single_client::deploy_module(chain, admin);
    nft_multi_rewarded_client::deploy_module(chain, admin);

    let (sft_contract_call, sft) =
        security_sft_single_client::init(chain, admin, &security_sft_single::types::InitParam {
            security:     None,
            metadata_url: ContractMetadataUrl {
                url:  REWARD_SFT_METADATA.to_string(),
                hash: None,
            },
            agents:       vec![
                // todo: add agents
            ],
        })
        .map(|(res, mod_ref, name)| {
            (
                to_contract_call_init(&res, mod_ref, name),
                SftSingleTestClient(res.contract_address),
            )
        })
        .expect("sft init");
    let (nft_contract_call, nft) =
        nft_multi_rewarded_client::init(chain, admin, &nft_multi_rewarded::types::InitParam {
            reward_token: TokenUId {
                contract: sft.contract_address(),
                id:       TokenIdUnit(),
            },
        })
        .map(|(res, mod_ref, name)| {
            (
                to_contract_call_init(&res, mod_ref, name),
                NftMultiRewardedTestClient(res.contract_address),
            )
        })
        .expect("nft init");
    let contract_calls = vec![sft_contract_call, nft_contract_call];

    let contract_calls = contract_calls.into_iter().collect::<Vec<_>>();

    let system_contracts = SystemContractsConfig {
        carbon_credit_contract_index:     Decimal::ZERO,
        carbon_credit_token_id:           Decimal::ZERO,
        compliance_contract_index:        Decimal::ZERO,
        euro_e_contract_index:            Decimal::ZERO,
        euro_e_token_id:                  TokenIdUnit().to_decimal(),
        identity_registry_contract_index: Decimal::ZERO,
        tree_ft_contract_index:           sft.contract_address().to_decimal(),
        tree_nft_contract_index:          nft.contract_address().to_decimal(),
        offchain_rewards_contract_index:  Decimal::ZERO,
        mint_funds_contract_index:        Decimal::ZERO,
        trading_contract_index:           Decimal::ZERO,
        yielder_contract_index:           Decimal::ZERO,
    };

    (system_contracts, contract_calls)
}

fn to_utc(timestamp: &Timestamp) -> NaiveDateTime {
    DateTime::from_timestamp_millis(timestamp.timestamp_millis() as i64)
        .expect("block_time_to_utc conversion")
        .naive_utc()
}
