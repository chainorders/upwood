use concordium_cis2::{TokenIdU64, TokenIdUnit, UpdateOperator};
use concordium_protocols::concordium_cis2_ext::ContractMetadataUrl;
use concordium_protocols::concordium_cis2_security::{
    AddTokenParams, AgentWithRoles, MintParam, SecurityParams, TokenAmountSecurity,
};
use concordium_protocols::rate::Rate;
use concordium_rwa_identity_registry::types::{
    Identity, IdentityAttribute, RegisterIdentityParams,
};
use concordium_smart_contract_testing::{Account, Chain};
use concordium_std::attributes::NATIONALITY;
use concordium_std::{AccountAddress, Amount, ContractAddress};
use integration_tests::cis2_conversions::to_token_id_vec;
use integration_tests::cis2_security::{Cis2SecurityTestClient, Cis2TestClient};
use integration_tests::compliance::init_nationalities;
use integration_tests::contract_base::{ContractPayloads, ContractTestClient};
use integration_tests::euroe::{self, EuroETestClient};
use integration_tests::identity_registry::{self, IdentityRegistryTestClient};
use integration_tests::security_sft_multi_client::SftMultiTestClient;
use integration_tests::security_sft_multi_yielder_client::{self, SftMultiYielderTestClient};
use integration_tests::security_sft_single_client::SftSingleTestClient;
use integration_tests::{
    compliance, security_mint_fund_client, security_p2p_trading_client, security_sft_multi_client,
    security_sft_single_client,
};
use security_sft_multi_yielder::{YieldCalculation, YieldParam, YieldState};

const ADMIN: AccountAddress = AccountAddress([0; 32]);
const INVESTOR_1: AccountAddress = AccountAddress([2; 32]);
const INVESTOR_2: AccountAddress = AccountAddress([3; 32]);
const TREASURY: AccountAddress = AccountAddress([4; 32]);
const COMPLIANT_NATIONALITIES: [&str; 2] = ["IN", "US"];
const DEFAULT_ACC_BALANCE: Amount = Amount {
    micro_ccd: 1_000_000_000_u64,
};

#[test]
fn normal_flow() {
    let admin = Account::new(ADMIN, DEFAULT_ACC_BALANCE);
    let mut chain = Chain::new();
    let (euroe_contract, ir_contract, compliance_contract) =
        setup_chain(&mut chain, &admin, &COMPLIANT_NATIONALITIES);
    let treasury = Account::new(TREASURY, DEFAULT_ACC_BALANCE);
    let carbon_credits_contract = create_carbon_credits_contract(&mut chain, &admin);

    chain.create_account(treasury.clone());
    let investor_1 = Account::new(INVESTOR_1, DEFAULT_ACC_BALANCE);
    chain.create_account(investor_1.clone());
    let investor_2 = Account::new(INVESTOR_2, DEFAULT_ACC_BALANCE);
    chain.create_account(investor_2.clone());

    let yielder_contract = SftMultiYielderTestClient::init(
        &mut chain,
        &admin,
        &security_sft_multi_yielder::InitParam {
            agents:   vec![],
            treasury: treasury.address.into(),
        },
    )
    .expect("init yielder contract");
    euroe_contract
        .update_operator_single(&mut chain, &treasury, &UpdateOperator {
            operator: yielder_contract.contract_address().into(),
            update:   concordium_cis2::OperatorUpdate::Add,
        })
        .expect("add yielder as operator");
    carbon_credits_contract
        .update_operator_single(&mut chain, &treasury, &UpdateOperator {
            operator: yielder_contract.contract_address().into(),
            update:   concordium_cis2::OperatorUpdate::Add,
        })
        .expect("add yielder as operator");

    euroe_contract
        .mint(&mut chain, &admin, &euroe::MintParams {
            owner:  treasury.address.into(),
            amount: 10_000_000_000.into(),
        })
        .expect("euroe mint to treasury");
    carbon_credits_contract
        .mint(
            &mut chain,
            &admin,
            &security_sft_single::types::MintParams {
                token_id: TokenIdUnit(),
                owners:   vec![MintParam {
                    address: treasury.address.into(),
                    amount:  TokenAmountSecurity::new_un_frozen(10_000.into()),
                }],
            },
        )
        .expect("mint carbon credits treasury");

    ir_contract
        .register_identity(&mut chain, &admin, &RegisterIdentityParams {
            address:  investor_1.address.into(),
            identity: Identity {
                credentials: vec![],
                attributes:  vec![IdentityAttribute {
                    tag:   NATIONALITY.0,
                    value: COMPLIANT_NATIONALITIES[1].to_string(),
                }],
            },
        })
        .expect("register identity investor 1");
    ir_contract
        .register_identity(&mut chain, &admin, &RegisterIdentityParams {
            address:  investor_2.address.into(),
            identity: Identity {
                credentials: vec![],
                attributes:  vec![IdentityAttribute {
                    tag:   NATIONALITY.0,
                    value: COMPLIANT_NATIONALITIES[1].to_string(),
                }],
            },
        })
        .expect("register identity investor 2");

    let token_ver_0 = TokenIdU64(0);
    let security_token_contract = create_token_contract_multi(
        &mut chain,
        &admin,
        compliance_contract,
        ir_contract.contract_address(),
        vec![AgentWithRoles {
            address: yielder_contract.contract_address().into(),
            roles:   vec![
                security_sft_multi::types::AgentRole::Operator,
                security_sft_multi::types::AgentRole::Mint,
            ],
        }],
    );
    security_token_contract
        .add_token(&mut chain, &admin, &AddTokenParams {
            token_id:       token_ver_0,
            token_metadata: concordium_protocols::concordium_cis2_ext::ContractMetadataUrl {
                url:  "example.com".to_string(),
                hash: None,
            },
        })
        .expect("add token wrapped");
    security_token_contract
        .mint(&mut chain, &admin, &security_sft_multi::types::MintParams {
            token_id: token_ver_0,
            owners:   vec![MintParam {
                address: investor_1.address.into(),
                amount:  TokenAmountSecurity::new_un_frozen(1000.into()),
            }],
        })
        .expect("mint wrapped token investor 1");
    security_token_contract
        .mint(&mut chain, &admin, &security_sft_multi::types::MintParams {
            token_id: token_ver_0,
            owners:   vec![MintParam {
                address: investor_2.address.into(),
                amount:  TokenAmountSecurity::new_un_frozen(2000.into()),
            }],
        })
        .expect("mint wrapped token investor 2");

    // Add yield 1 at Tick 100 for wrapped token
    let token_ver_100 = TokenIdU64(100);
    security_token_contract
        .add_token(&mut chain, &admin, &AddTokenParams {
            // Tick difference between the two tokens is 100
            token_id:       token_ver_100,
            token_metadata: ContractMetadataUrl {
                url:  "example.com".to_string(),
                hash: None,
            },
        })
        .expect("add token wrapped 100");
    yielder_contract
        .upsert_yield(
            &mut chain,
            &admin,
            &security_sft_multi_yielder::UpsertYieldParams {
                token_contract: security_token_contract.contract_address(),
                token_id:       token_ver_100,
                yields:         vec![
                    YieldState {
                        contract:    euroe_contract.contract_address(),
                        token_id:    to_token_id_vec(TokenIdUnit()),
                        calculation: YieldCalculation::SimpleInterest(
                            // 10 EUR per 1 wrapped token
                            Rate::new(10, 1).expect("valid rate"),
                        ),
                    },
                    YieldState {
                        contract:    carbon_credits_contract.contract_address(),
                        token_id:    to_token_id_vec(TokenIdUnit()),
                        calculation: YieldCalculation::Quantity(
                            Rate::new(5, 1).expect("valid rate"),
                        ),
                    },
                ],
            },
        )
        .expect("upsert yield 1");

    // Add yield 2 at Tick 200 for wrapped token
    let token_ver_200 = TokenIdU64(200);
    security_token_contract
        .add_token(&mut chain, &admin, &AddTokenParams {
            // Tick difference between the two tokens is 100
            token_id:       token_ver_200,
            token_metadata: ContractMetadataUrl {
                url:  "example.com".to_string(),
                hash: None,
            },
        })
        .expect("add token wrapped 200");
    yielder_contract
        .upsert_yield(
            &mut chain,
            &admin,
            &security_sft_multi_yielder::UpsertYieldParams {
                token_contract: security_token_contract.contract_address(),
                token_id:       token_ver_200,
                yields:         vec![YieldState {
                    token_id:    to_token_id_vec(TokenIdUnit()),
                    contract:    euroe_contract.contract_address(),
                    calculation: YieldCalculation::SimpleInterest(
                        // 10 EUR per 1 wrapped token
                        Rate::new(20, 1).expect("valid rate"),
                    ),
                }],
            },
        )
        .expect("upsert yield 2");

    // Yield for investor 1
    yielder_contract
        .yield_for(
            &mut chain,
            &investor_1,
            &security_sft_multi_yielder::YieldParams {
                owner:  investor_1.address,
                yields: vec![YieldParam {
                    amount:         1000.into(),
                    token_contract: security_token_contract.contract_address(),
                    token_ver_from: token_ver_0,
                    token_ver_to:   token_ver_200,
                }],
            },
        )
        .expect("yield for investor 1");
    // yeild assertions for investor 1
    assert_eq!(
        security_token_contract
            .balance_of_single(&chain, &investor_1, token_ver_0, investor_1.address.into(),)
            .expect("balance of investor 1"),
        0.into()
    );
    assert_eq!(
        security_token_contract
            .balance_of_single(
                &chain,
                &investor_1,
                token_ver_200,
                investor_1.address.into(),
            )
            .expect("balance of investor 1"),
        1000.into()
    );
    assert_eq!(
        carbon_credits_contract
            .balance_of_single(
                &chain,
                &investor_1,
                TokenIdUnit(),
                investor_1.address.into(),
            )
            .expect("carbon credits balance of investor 1"),
        (5 * 1000).into()
    );
    assert_eq!(
        euroe_contract
            .balance_of_single(
                &chain,
                &investor_1,
                TokenIdUnit(),
                investor_1.address.into(),
            )
            .expect("euroe balance of investor 1"),
        (100 * (10 + 20) * 1000).into()
    );
}

fn create_carbon_credits_contract(chain: &mut Chain, admin: &Account) -> SftSingleTestClient {
    SftSingleTestClient::init(chain, admin, &security_sft_single::types::InitParam {
        security:     None,
        metadata_url: ContractMetadataUrl {
            hash: None,
            url:  "example.com/carbon_credits".to_string(),
        },
        agents:       vec![],
    })
    .expect("init token contract")
}

fn create_token_contract_multi(
    chain: &mut Chain,
    admin: &Account,
    compliance_contract: ContractAddress,
    ir_contract: ContractAddress,
    agents: Vec<AgentWithRoles<security_sft_multi::types::AgentRole>>,
) -> SftMultiTestClient {
    SftMultiTestClient::init(chain, admin, &security_sft_multi::types::InitParam {
        security: Some(SecurityParams {
            compliance:        compliance_contract,
            identity_registry: ir_contract,
        }),
        agents,
    })
    .expect("init token contract")
}

fn setup_chain(
    chain: &mut Chain,
    admin: &Account,
    compliant_nationalities: &[&str],
) -> (EuroETestClient, IdentityRegistryTestClient, ContractAddress) {
    chain.create_account(admin.clone());

    euroe::deploy_module(chain, admin);
    identity_registry::deploy_module(chain, admin);
    compliance::deploy_module(chain, admin);
    security_sft_single_client::deploy_module(chain, admin);
    security_sft_multi_client::deploy_module(chain, admin);
    security_p2p_trading_client::deploy_module(chain, admin);
    security_mint_fund_client::deploy_module(chain, admin);
    security_sft_multi_yielder_client::deploy_module(chain, admin);

    let euroe_contract = EuroETestClient::init(chain, admin, &()).expect("init euroe");
    let ir_contract =
        IdentityRegistryTestClient::init(chain, admin, &()).expect("init identity registry");

    let (compliance_module, ..) = init_nationalities(
        chain,
        admin,
        &concordium_rwa_compliance::compliance_modules::allowed_nationalities::types::InitParams {
            nationalities:     compliant_nationalities
                .iter()
                .map(|n| n.to_string())
                .collect(),
            identity_registry: ir_contract.contract_address(),
        },
    )
    .expect("init nationalities module");

    let (compliance, ..) = compliance::init(chain, admin, vec![compliance_module.contract_address])
        .expect("init compliance module");
    euroe_contract
        .grant_role(chain, admin, &euroe::RoleTypes {
            adminrole: admin.address.into(),
            blockrole: admin.address.into(),
            burnrole:  admin.address.into(),
            mintrole:  admin.address.into(),
            pauserole: admin.address.into(),
        })
        .expect("grant_role euroe");
    (euroe_contract, ir_contract, compliance.contract_address)
}
