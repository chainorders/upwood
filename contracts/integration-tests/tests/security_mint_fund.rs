#![cfg(test)]

use cis2_security::{Cis2SecurityTestClient, Cis2TestClient};
use compliance::init_nationalities;
use concordium_cis2::{
    BalanceOfQuery, BalanceOfQueryParams, BalanceOfQueryResponse, OperatorUpdate, TokenIdU64,
    TokenIdUnit, UpdateOperator,
};
use concordium_protocols::concordium_cis2_security::{
    AddTokenParams, AgentWithRoles, SecurityParams, TokenUId,
};
use concordium_protocols::rate::Rate;
use concordium_rwa_identity_registry::types::{
    Identity, IdentityAttribute, RegisterIdentityParams,
};
use concordium_smart_contract_testing::*;
use concordium_std::attributes::NATIONALITY;
use contract_base::{ContractPayloads, ContractTestClient};
use euroe::EuroETestClient;
use identity_registry::IdentityRegistryTestClient;
use integration_tests::*;
use security_mint_fund::types::{
    AddFundParams, ClaimInvestmentParam, ClaimInvestmentParams, InitParam, TransferInvestParams,
    UpdateFundState, UpdateFundStateParams,
};
use security_mint_fund_client::MintFundTestClient;
use security_sft_multi_client::SftMultiTestClient;
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
    chain.create_account(treasury.clone());
    let investor_1 = Account::new(INVESTOR_1, DEFAULT_ACC_BALANCE);
    chain.create_account(investor_1.clone());
    let investor_2 = Account::new(INVESTOR_2, DEFAULT_ACC_BALANCE);
    chain.create_account(investor_2.clone());
    euroe_contract
        .mint(&mut chain, &admin, &euroe::MintParams {
            owner:  investor_1.address.into(),
            amount: 1000.into(),
        })
        .expect("euroe mint investor 1");
    euroe_contract
        .mint(&mut chain, &admin, &euroe::MintParams {
            owner:  investor_2.address.into(),
            amount: 2000.into(),
        })
        .expect("euroe mint investor 2");

    let fund_contract = MintFundTestClient::init(&mut chain, &admin, &InitParam {
        currency_token: TokenUId {
            id:       TokenIdUnit(),
            contract: euroe_contract.contract_address(),
        },
        agents:         vec![],
    })
    .expect("init fund contract");

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

    let wrapped_token_id = TokenIdU64(0);
    let wrapped_token_contract = create_token_contract_multi(
        &mut chain,
        &admin,
        compliance_contract,
        ir_contract.contract_address(),
    );
    wrapped_token_contract
        .add_token(&mut chain, &admin, &AddTokenParams {
            token_id:       wrapped_token_id,
            token_metadata: concordium_protocols::concordium_cis2_ext::ContractMetadataUrl {
                url:  "example.com".to_string(),
                hash: None,
            },
        })
        .expect("add token wrapped");
    wrapped_token_contract
        .add_agent(&mut chain, &admin, &AgentWithRoles {
            address: fund_contract.contract_address().into(),
            roles:   vec![
                security_sft_multi::types::AgentRole::Mint,
                security_sft_multi::types::AgentRole::ForcedBurn,
            ],
        })
        .expect("add agent wrapped");

    let investment_token_contract = create_token_contract_multi(
        &mut chain,
        &admin,
        compliance_contract,
        ir_contract.contract_address(),
    );
    investment_token_contract
        .add_agent(&mut chain, &admin, &AgentWithRoles {
            address: fund_contract.contract_address().into(),
            roles:   vec![security_sft_multi::types::AgentRole::Mint],
        })
        .expect("add agent investment");

    // Adding investment token. This would the final Security Token
    // which the investors will receive.
    let investment_token_id = TokenIdU64(1);
    investment_token_contract
        .add_token(&mut chain, &admin, &AddTokenParams {
            token_id:       investment_token_id,
            token_metadata: concordium_protocols::concordium_cis2_ext::ContractMetadataUrl {
                url:  "example.com".to_string(),
                hash: None,
            },
        })
        .expect("add token investment");
    let security_token = TokenUId {
        contract: investment_token_contract.contract_address(),
        id:       investment_token_id,
    };
    // Adding fund
    fund_contract
        .add_fund(&mut chain, &admin, &AddFundParams {
            token: TokenUId {
                contract: wrapped_token_contract.contract_address(),
                id:       wrapped_token_id,
            },
            rate: Rate::new(1000, 1).unwrap(),
            security_token,
        })
        .expect("add fund");

    // First Investment
    euroe_contract
        .update_operator_single(&mut chain, &investor_1, &UpdateOperator {
            update:   OperatorUpdate::Add,
            operator: fund_contract.contract_address().into(),
        })
        .expect("update operator investor 1");
    fund_contract
        .transfer_invest(&mut chain, &investor_1, &TransferInvestParams {
            security_token,
            amount: 1000.into(),
        })
        .expect("transfer_invest");

    // Second Investment
    euroe_contract
        .update_operator_single(&mut chain, &investor_2, &UpdateOperator {
            update:   OperatorUpdate::Add,
            operator: fund_contract.contract_address().into(),
        })
        .expect("update operator investor 2");
    fund_contract
        .transfer_invest(&mut chain, &investor_2, &TransferInvestParams {
            security_token,
            amount: 2000.into(),
        })
        .expect("transfer_invest");
    assert_eq!(
        euroe_contract
            .balance_of(&chain, &admin, &BalanceOfQueryParams {
                queries: vec![
                    BalanceOfQuery {
                        address:  investor_1.address.into(),
                        token_id: TokenIdUnit(),
                    },
                    BalanceOfQuery {
                        address:  investor_2.address.into(),
                        token_id: TokenIdUnit(),
                    },
                    BalanceOfQuery {
                        address:  fund_contract.contract_address().into(),
                        token_id: TokenIdUnit(),
                    },
                    BalanceOfQuery {
                        address:  treasury.address.into(),
                        token_id: TokenIdUnit(),
                    }
                ],
            })
            .expect("balance of euroe"),
        BalanceOfQueryResponse(vec![0.into(), 0.into(), 3000.into(), 0.into()])
    );
    assert_eq!(
        wrapped_token_contract
            .balance_of(&chain, &admin, &BalanceOfQueryParams {
                queries: vec![
                    BalanceOfQuery {
                        address:  investor_1.address.into(),
                        token_id: wrapped_token_id,
                    },
                    BalanceOfQuery {
                        address:  investor_2.address.into(),
                        token_id: wrapped_token_id,
                    },
                ],
            })
            .expect("balance of wrapped"),
        BalanceOfQueryResponse(vec![1.into(), 2.into()])
    );

    // Updating fund state to success
    fund_contract
        .update_fund_state(&mut chain, &admin, &UpdateFundStateParams {
            security_token,
            state: UpdateFundState::Success(treasury.address.into()),
        })
        .expect("update_fund_state");

    // Euro e investments is stored in the fund contract
    assert_eq!(
        euroe_contract
            .balance_of(&chain, &admin, &BalanceOfQueryParams {
                queries: vec![
                    BalanceOfQuery {
                        address:  investor_1.address.into(),
                        token_id: TokenIdUnit(),
                    },
                    BalanceOfQuery {
                        address:  investor_2.address.into(),
                        token_id: TokenIdUnit(),
                    },
                    BalanceOfQuery {
                        address:  fund_contract.contract_address().into(),
                        token_id: TokenIdUnit(),
                    },
                    BalanceOfQuery {
                        address:  treasury.address.into(),
                        token_id: TokenIdUnit(),
                    }
                ],
            })
            .expect("balance of euroe"),
        BalanceOfQueryResponse(vec![0.into(), 0.into(), 3000.into(), 0.into()])
    );

    // Claiming investments
    fund_contract
        .claim_investment(&mut chain, &investor_1, &ClaimInvestmentParams {
            investments: vec![ClaimInvestmentParam {
                security_token,
                investor: investor_1.address,
            }],
        })
        .expect("claim_investment investor 1");
    fund_contract
        .claim_investment(&mut chain, &investor_2, &ClaimInvestmentParams {
            investments: vec![ClaimInvestmentParam {
                security_token,
                investor: investor_2.address,
            }],
        })
        .expect("claim_investment investor 2");
    assert_eq!(
        wrapped_token_contract
            .balance_of(&chain, &admin, &BalanceOfQueryParams {
                queries: vec![
                    BalanceOfQuery {
                        address:  investor_1.address.into(),
                        token_id: wrapped_token_id,
                    },
                    BalanceOfQuery {
                        address:  investor_2.address.into(),
                        token_id: wrapped_token_id,
                    },
                ],
            })
            .expect("balance of wrapped"),
        BalanceOfQueryResponse(vec![0.into(), 0.into()])
    );
    assert_eq!(
        investment_token_contract
            .balance_of(&chain, &admin, &BalanceOfQueryParams {
                queries: vec![
                    BalanceOfQuery {
                        address:  investor_1.address.into(),
                        token_id: investment_token_id,
                    },
                    BalanceOfQuery {
                        address:  investor_2.address.into(),
                        token_id: investment_token_id,
                    },
                ],
            })
            .expect("balance of investment"),
        BalanceOfQueryResponse(vec![1.into(), 2.into()])
    );
}

fn create_token_contract_multi(
    chain: &mut Chain,
    admin: &Account,
    compliance_contract: ContractAddress,
    ir_contract: ContractAddress,
) -> SftMultiTestClient {
    SftMultiTestClient::init(chain, admin, &security_sft_multi::types::InitParam {
        security: Some(SecurityParams {
            compliance:        compliance_contract,
            identity_registry: ir_contract,
        }),
        agents:   vec![],
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
