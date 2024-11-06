use std::sync::Arc;

use chrono::{DateTime, Utc};
use cis2_conversions::to_token_id_vec;
use concordium_cis2::{TokenIdUnit, UpdateOperator};
use concordium_protocols::concordium_cis2_security::TokenUId;
use concordium_rust_sdk::types::WalletAccount;
use concordium_smart_contract_testing::*;
use diesel::r2d2::ConnectionManager;
use integration_tests::*;
use nft_multi_rewarded::types::{Agent, ContractMetadataUrl};
use nft_multi_rewarded::{MintData, SignedMetadata};
use poem::web::Data;
use shared::db::DbPool;
use upwood::api::tree_nft_metadata::AddMetadataRequest;
use upwood::api::{self, BearerAuthorization, TreeNftContractAddress};
use upwood::utils::aws::cognito::Claims;

const ADMIN: AccountAddress = AccountAddress([0; 32]);
const HOLDER: AccountAddress = AccountAddress([1; 32]);
const NFT_AGENT: AccountAddress = AccountAddress([2; 32]);
const DEFAULT_BALANCE_AMOUNT: Amount = Amount::from_micro_ccd(1_000_000_000);
const COMPLIANT_NATIONALITIES: [&str; 3] = ["IN", "US", "GB"];
const REWARD_SFT_METADATA: &str = "https://metadata.com/rewarded_sft_token";
const NFT_METADATA: &str = "https://metadata.com/nft_token";

#[tokio::test]
async fn signature_tests() {
    // Setup Database
    let (db_url, _container) = shared_tests::create_new_database_container().await;
    events_listener::db_setup::run_migrations(&db_url);
    upwood::db::db_setup::run_migrations(&db_url);
    let pool: DbPool = r2d2::Pool::builder()
        .max_size(10)
        .build(ConnectionManager::new(db_url))
        .expect("Error creating database pool");

    // processor / listener
    let nft_processor =
        events_listener::txn_processor::nft_multi_rewarded::processor::process_events;
    let mut processor_db_conn = pool.get().expect("db connection");

    // api
    let metadata_admin_api = api::tree_nft_metadata::AdminApi;
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
    let now = Utc::now();
    let mut chain = Chain::new_with_time(Timestamp {
        millis: now.timestamp_millis() as u64,
    });
    let admin = Account::new(ADMIN, DEFAULT_BALANCE_AMOUNT);
    chain.create_account(admin.clone());
    let holder = Account::new(HOLDER, DEFAULT_BALANCE_AMOUNT);
    chain.create_account(holder.clone());

    let agent_account_keys = AccountKeys::singleton(&mut rand::thread_rng());
    let agent = Account::new_with_keys(
        NFT_AGENT,
        AccountBalance::new(DEFAULT_BALANCE_AMOUNT, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::from(&agent_account_keys),
    );
    chain.create_account(agent.clone());

    let (_, identity_registry, compliance) =
        setup_chain(&mut chain, &admin, &COMPLIANT_NATIONALITIES);
    identity_registry::register_nationalities(&mut chain, &admin, &identity_registry, vec![(
        holder.address.into(),
        COMPLIANT_NATIONALITIES[0],
    )]);

    let sft_contract = security_sft_single_client::init(
        &mut chain,
        &admin,
        &security_sft_single::types::InitParam {
            compliance,
            identity_registry,
            sponsors: None,
            metadata_url: ContractMetadataUrl {
                url:  REWARD_SFT_METADATA.to_string(),
                hash: None,
            },
        },
    )
    .contract_address;
    let init_res = nft_multi_rewarded_client::init(
        &mut chain,
        &admin,
        &nft_multi_rewarded::types::InitParam {
            reward_token: TokenUId {
                contract: sft_contract,
                id:       to_token_id_vec(TokenIdUnit()),
            },
        },
    )
    .expect("nft init");
    let nft_contract = init_res.contract_address;
    nft_processor(
        &mut processor_db_conn,
        to_utc(&chain.block_time()),
        &nft_contract,
        &init_res.events,
    )
    .expect("process init events");
    chain.tick_block_time(Duration::from_seconds(2)).unwrap();
    identity_registry::register_nationalities(&mut chain, &admin, &identity_registry, vec![(
        nft_contract.into(),
        COMPLIANT_NATIONALITIES[0],
    )]);
    chain.tick_block_time(Duration::from_seconds(2)).unwrap();

    let add_agent_res =
        nft_multi_rewarded_client::add_agent(&mut chain, &admin, nft_contract, &Agent {
            address: NFT_AGENT.into(),
        })
        .expect("add agent");
    let tree_nft_config = api::tree_nft_metadata::TreeNftConfig {
        agent: Arc::new(api::tree_nft_metadata::TreeNftAgent(WalletAccount {
            address: agent.address,
            keys:    agent_account_keys,
        })),
    };
    nft_processor(
        &mut processor_db_conn,
        to_utc(&chain.block_time()),
        &nft_contract,
        &add_agent_res
            .events()
            .filter(|(contract, _)| contract.eq(&nft_contract))
            .flat_map(|(_, events)| events)
            .cloned()
            .collect::<Vec<_>>(),
    )
    .expect("process add agent events");
    security_sft_single_client::mint(
        &mut chain,
        &admin,
        &sft_contract,
        &security_sft_single::types::MintParams {
            token_id: TokenIdUnit(),
            owners:   vec![security_sft_single::types::MintParam {
                amount:  1.into(),
                address: holder.address,
            }],
        },
    )
    .expect("sft mint");
    security_sft_single_client::update_operator_single(
        &mut chain,
        &holder,
        sft_contract,
        UpdateOperator {
            update:   concordium_cis2::OperatorUpdate::Add,
            operator: nft_contract.into(),
        },
    )
    .expect("update operator");

    // Api Interactions
    let api_user_claims = BearerAuthorization(Claims {
        sub:            "NORMAL_USER_ID".to_string(),
        cognito_groups: None,
        email_verified: Some(true),
        email:          "normal_user@example.com".to_string(),
        account:        None,
    });

    let api = api::tree_nft_metadata::Api;
    let poem_openapi::payload::Json(random_metadata) = api
        .metadata_get_random(
            api_user_claims.clone(),
            Data(&pool),
            Data(&tree_nft_config),
            Data(&TreeNftContractAddress(nft_contract)),
        )
        .await
        .expect("metadata get random");

    let transfer_mint_res =
        nft_multi_rewarded_client::transfer_mint(&mut chain, &holder, nft_contract, &MintData {
            signed_metadata: SignedMetadata {
                account:          holder.address,
                account_nonce:    0,
                contract_address: nft_contract,
                metadata_url:     nft_multi_rewarded::MetadataUrl {
                    url:  random_metadata.signed_metadata.metadata_url.url,
                    hash: None,
                },
            },
            signer:          agent.address,
            signature:       serde_json::from_value(random_metadata.signature)
                .expect("signature deserialization"),
        })
        .expect("mint");
    nft_processor(
        &mut processor_db_conn,
        to_utc(&chain.block_time()),
        &nft_contract,
        &transfer_mint_res
            .events()
            .filter(|(contract, _)| contract.eq(&nft_contract))
            .flat_map(|(_, events)| events)
            .cloned()
            .collect::<Vec<_>>(),
    )
    .expect("process mint events");

    let balance: nft_multi_rewarded::types::TokenAmount =
        nft_multi_rewarded_client::balance_of_single(
            &mut chain,
            &holder,
            nft_contract,
            concordium_cis2::TokenIdU64(0),
            holder.address.into(),
            nft_multi_rewarded_client::CONTRACT_NAME,
        )
        .expect("balance of");
    assert_eq!(balance, 1.into());

    let poem_openapi::payload::Json(nonce) = api::tree_nft::Api
        .nonce(
            api_user_claims,
            Data(&pool),
            Data(&TreeNftContractAddress(nft_contract)),
        )
        .await
        .expect("nonce");
    assert_eq!(nonce, 1);
}
fn setup_chain(
    chain: &mut Chain,
    admin: &Account,
    compliant_nationalities: &[&str],
) -> (ContractAddress, ContractAddress, ContractAddress) {
    chain.create_account(admin.clone());

    euroe::deploy_module(chain, admin);
    identity_registry::deploy_module(chain, admin);
    compliance::deploy_module(chain, admin);
    security_sft_single_client::deploy_module(chain, admin);
    nft_multi_rewarded_client::deploy_module(chain, admin);

    let ir_contract = identity_registry::init(chain, admin).contract_address;
    let compliance_contract =
        compliance::init_all(chain, admin, ir_contract, compliant_nationalities).contract_address;
    let euroe = euroe::init(chain, admin).contract_address;
    euroe::grant_role(chain, admin, euroe, &euroe::RoleTypes {
        mintrole:  admin.address.into(),
        burnrole:  admin.address.into(),
        blockrole: admin.address.into(),
        pauserole: admin.address.into(),
        adminrole: admin.address.into(),
    });
    (euroe, ir_contract, compliance_contract)
}

fn to_utc(timestamp: &Timestamp) -> DateTime<Utc> {
    DateTime::from_timestamp_millis(timestamp.timestamp_millis() as i64)
        .expect("block_time_to_utc conversion")
}
