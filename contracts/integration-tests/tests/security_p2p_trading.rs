#![cfg(test)]

use cis2_conversions::to_token_id_vec;
use cis2_security::{Cis2SecurityTestClient, Cis2TestClient};
use compliance::init_nationalities;
use concordium_cis2::{
    BalanceOfQuery, BalanceOfQueryResponse, OperatorUpdate, TokenAmountU64, TokenIdU64,
    TokenIdUnit, UpdateOperator,
};
use concordium_protocols::concordium_cis2_security::{
    AddTokenParams, AgentWithRoles, Identity, SecurityParams, TokenAmountSecurity, TokenUId,
};
use concordium_protocols::rate::Rate;
use concordium_rwa_identity_registry::types::{IdentityAttribute, RegisterIdentityParams};
use concordium_smart_contract_testing::*;
use concordium_std::attributes::NATIONALITY;
use concordium_std::{AccountAddress, Amount};
use contract_base::{ContractPayloads, ContractTestClient};
use euroe::EuroETestClient;
use identity_registry::IdentityRegistryTestClient;
use integration_tests::*;
use security_p2p_trading::{
    Deposit, ExchangeParams, SellPositionOfParams, TransferExchangeParams, TransferSellParams,
};
use security_p2p_trading_client::{P2PTradeTestClient, P2PTradingClientResponses};
use security_sft_multi_client::SftMultiTestClient;

const METADATA_URL_SFT_REWARDS: &str = "example2.com";
const ADMIN: AccountAddress = AccountAddress([0; 32]);
const HOLDER: AccountAddress = AccountAddress([2; 32]);
const HOLDER_2: AccountAddress = AccountAddress([3; 32]);
const COMPLIANT_NATIONALITIES: [&str; 2] = ["IN", "US"];
const DEFAULT_ACC_BALANCE: Amount = Amount {
    micro_ccd: 1_000_000_000_u64,
};

#[test]
pub fn normal_flow_sft_multi() {
    use security_sft_multi::types::*;

    let admin = Account::new(ADMIN, DEFAULT_ACC_BALANCE);
    let mut chain = Chain::new();
    let holder = Account::new(HOLDER, DEFAULT_ACC_BALANCE);
    chain.create_account(holder.clone());
    let holder_2 = Account::new(HOLDER_2, DEFAULT_ACC_BALANCE);
    chain.create_account(holder_2.clone());

    let (euroe_contract, ir_contract, compliance_contract) =
        setup_chain(&mut chain, &admin, &COMPLIANT_NATIONALITIES);

    let euroe_token_id_vec = to_token_id_vec(TokenIdUnit());

    let trading_contract =
        P2PTradeTestClient::init(&mut chain, &admin, &security_p2p_trading::InitParam {
            currency: TokenUId {
                id:       euroe_token_id_vec,
                contract: euroe_contract.contract_address(),
            },
        })
        .expect("init trading contract");
    ir_contract
        .register_identity(&mut chain, &admin, RegisterIdentityParams {
            address:  trading_contract.contract_address().into(),
            identity: Identity {
                credentials: vec![],
                attributes:  vec![IdentityAttribute {
                    tag:   NATIONALITY.0,
                    value: COMPLIANT_NATIONALITIES[1].to_string(),
                }],
            },
        })
        .expect("register identity");

    ir_contract
        .register_identity(&mut chain, &admin, RegisterIdentityParams {
            address:  holder.address.into(),
            identity: Identity {
                credentials: vec![],
                attributes:  vec![IdentityAttribute {
                    tag:   NATIONALITY.0,
                    value: COMPLIANT_NATIONALITIES[1].to_string(),
                }],
            },
        })
        .expect("register identity");
    ir_contract
        .register_identity(&mut chain, &admin, RegisterIdentityParams {
            address:  holder_2.address.into(),
            identity: Identity {
                credentials: vec![],
                attributes:  vec![IdentityAttribute {
                    tag:   NATIONALITY.0,
                    value: COMPLIANT_NATIONALITIES[1].to_string(),
                }],
            },
        })
        .expect("register identity");

    const TOKEN_ID: TokenIdU64 = TokenIdU64(0);
    let security_token_id_vec = to_token_id_vec(TOKEN_ID);
    let token_contract = create_token_contract_multi(
        &mut chain,
        &admin,
        compliance_contract,
        ir_contract.contract_address(),
        vec![AgentWithRoles {
            address: trading_contract.contract_address().into(),
            roles:   vec![security_sft_multi::types::AgentRole::Operator],
        }],
    );
    trading_contract
        .add_market(&mut chain, &admin, &token_contract.contract_address())
        .expect("add market");

    token_contract
        .add_token(&mut chain, &admin, &AddTokenParams {
            token_id:       TOKEN_ID,
            token_metadata: ContractMetadataUrl {
                hash: None,
                url:  METADATA_URL_SFT_REWARDS.to_string(),
            },
        })
        .expect("add token");
    token_contract
        .mint(&mut chain, &admin, &MintParams {
            owners:   vec![MintParam {
                amount:  TokenAmountSecurity::new_un_frozen(50.into()),
                address: holder.address.into(),
            }],
            token_id: TOKEN_ID,
        })
        .expect("mint");
    assert_eq!(
        token_contract
            .balance_of_single(&mut chain, &holder, TOKEN_ID, holder.address.into())
            .expect("balance of"),
        TokenAmountU64(50)
    );
    let rate = Rate::new(1000, 1).unwrap();
    trading_contract
        .transfer_sell(&mut chain, &holder, &TransferSellParams {
            market: token_contract.contract_address(),
            amount: TokenAmountU64(10),
            rate,
            token_id: security_token_id_vec.clone(),
        })
        .expect("transfer sell");
    assert_eq!(
        token_contract
            .balance_of_single(&mut chain, &admin, TOKEN_ID, holder.address.into())
            .expect("balance of"),
        TokenAmountU64(40)
    );

    assert_eq!(
        trading_contract
            .sell_position_of(&mut chain, &admin, &SellPositionOfParams {
                market:   token_contract.contract_address(),
                seller:   holder.address,
                token_id: security_token_id_vec.clone(),
            })
            .expect("sell position of")
            .parse_sell_position_of(),
        Deposit {
            amount: TokenAmountU64(10),
            rate
        }
    );

    euroe_contract
        .mint(&mut chain, &admin, &euroe::MintParams {
            owner:  holder_2.address.into(),
            amount: TokenAmountU64(1000),
        })
        .expect("mint");
    euroe_contract
        .update_operator_single(&mut chain, &holder_2, &UpdateOperator {
            update:   OperatorUpdate::Add,
            operator: trading_contract.contract_address().into(),
        })
        .expect("update operator");
    trading_contract
        .transfer_exchange(&mut chain, &holder_2, &TransferExchangeParams {
            pay: TokenAmountU64(1000),
            get: ExchangeParams {
                market:   token_contract.contract_address(),
                from:     holder.address,
                amount:   TokenAmountU64(1),
                token_id: security_token_id_vec.clone(),
            },
        })
        .expect("transfer exchange");
    assert_eq!(
        euroe_contract
            .balance_of_single(&mut chain, &admin, TokenIdUnit(), holder_2.address.into())
            .expect("balance of"),
        TokenAmountU64(0)
    );
    assert_eq!(
        trading_contract
            .sell_position_of(&mut chain, &admin, &SellPositionOfParams {
                market:   token_contract.contract_address(),
                seller:   holder.address,
                token_id: security_token_id_vec.clone(),
            })
            .expect("sell position of")
            .parse_sell_position_of(),
        Deposit {
            amount: TokenAmountU64(9),
            rate
        }
    );
    assert_eq!(
        token_contract
            .balance_of(&mut chain, &admin, &BalanceOfQueryParams {
                queries: vec![
                    BalanceOfQuery {
                        token_id: TOKEN_ID,
                        address:  holder.address.into(),
                    },
                    BalanceOfQuery {
                        token_id: TOKEN_ID,
                        address:  holder_2.address.into(),
                    }
                ],
            })
            .expect("balance of query"),
        BalanceOfQueryResponse(vec![TokenAmountU64(40), TokenAmountU64(1),])
    );
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
        .expect("grant role");
    (euroe_contract, ir_contract, compliance.contract_address)
}
