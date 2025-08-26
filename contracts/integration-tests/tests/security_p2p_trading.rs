#![cfg(test)]

use cis2_security::{Cis2SecurityTestClient, Cis2TestClient};
use compliance::init_nationalities;
use concordium_cis2::{
    BalanceOfQuery, BalanceOfQueryParams, BalanceOfQueryResponse, OperatorUpdate, TokenAmountU64,
    TokenIdU64, TokenIdUnit, UpdateOperator,
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
    AddMarketParams, ExchangeParams, Market, MintMarket, MintParams, TokenIdCalculation,
    TransferMarket,
};
use security_p2p_trading_client::P2PTradeTestClient;
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
    let admin = Account::new(ADMIN, DEFAULT_ACC_BALANCE);
    let mut chain = Chain::new();
    let seller = Account::new(HOLDER, DEFAULT_ACC_BALANCE);
    chain.create_account(seller.clone());
    let liquidity_provider = Account::new(HOLDER_2, DEFAULT_ACC_BALANCE);
    chain.create_account(liquidity_provider.clone());

    let (euroe_contract, ir_contract, compliance_contract) =
        setup_chain(&mut chain, &admin, &COMPLIANT_NATIONALITIES);

    let euroe_token_id = TokenIdUnit();
    let trading_contract =
        P2PTradeTestClient::init(&mut chain, &admin, &security_p2p_trading::InitParam {
            currency: TokenUId {
                id:       euroe_token_id,
                contract: euroe_contract.contract_address(),
            },
            agents:   vec![],
        })
        .expect("init trading contract");
    euroe_contract
        .mint(&mut chain, &admin, &euroe::MintParams {
            owner:  liquidity_provider.address.into(),
            amount: TokenAmountU64(30_000),
        })
        .expect("euroe mint");
    euroe_contract
        .update_operator_single(&mut chain, &liquidity_provider, &UpdateOperator {
            update:   OperatorUpdate::Add,
            operator: trading_contract.contract_address().into(),
        })
        .expect("update operator");

    ir_contract
        .register_identity(&mut chain, &admin, &RegisterIdentityParams {
            address:  seller.address.into(),
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
        .register_identity(&mut chain, &admin, &RegisterIdentityParams {
            address:  liquidity_provider.address.into(),
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
    token_contract
        .add_token(&mut chain, &admin, &AddTokenParams {
            token_id:       TOKEN_ID,
            token_metadata: security_sft_multi::types::ContractMetadataUrl {
                hash: None,
                url:  METADATA_URL_SFT_REWARDS.to_string(),
            },
        })
        .expect("add token");
    token_contract
        .mint(&mut chain, &admin, &security_sft_multi::types::MintParams {
            owners:   vec![security_sft_multi::types::MintParam {
                amount:  TokenAmountSecurity::new_un_frozen(50.into()),
                address: seller.address.into(),
            }],
            token_id: TOKEN_ID,
        })
        .expect("mint");

    let rate = Rate::new(1000, 1).unwrap();
    trading_contract
        .add_market(&mut chain, &admin, &AddMarketParams {
            token_contract: token_contract.contract_address(),
            market:         Market::Transfer(TransferMarket {
                token_id:            TOKEN_ID,
                liquidity_provider:  liquidity_provider.address,
                buy_rate:            rate,
                sell_rate:           rate,
                max_currency_amount: TokenAmountU64(10_000),
                max_token_amount:    TokenAmountU64(10),
            }),
        })
        .expect("add market");

    trading_contract
        .sell(&mut chain, &seller, &ExchangeParams {
            amount: TokenAmountU64(10),
            rate,
            contract: token_contract.contract_address(),
        })
        .expect("sell");
    trading_contract
        .sell(&mut chain, &seller, &ExchangeParams {
            amount: TokenAmountU64(10),
            rate,
            contract: token_contract.contract_address(),
        })
        .expect_err("should fail");
    assert_eq!(
        token_contract
            .balance_of(&chain, &admin, &BalanceOfQueryParams {
                queries: vec![
                    BalanceOfQuery {
                        token_id: TOKEN_ID,
                        address:  seller.address.into(),
                    },
                    BalanceOfQuery {
                        token_id: TOKEN_ID,
                        address:  liquidity_provider.address.into(),
                    }
                ],
            })
            .expect("balance of"),
        BalanceOfQueryResponse(vec![40.into(), 10.into()])
    );
    assert_eq!(
        euroe_contract
            .balance_of(&chain, &admin, &BalanceOfQueryParams {
                queries: vec![
                    BalanceOfQuery {
                        token_id: euroe_token_id,
                        address:  seller.address.into(),
                    },
                    BalanceOfQuery {
                        token_id: euroe_token_id,
                        address:  liquidity_provider.address.into(),
                    }
                ],
            })
            .expect("balance of"),
        BalanceOfQueryResponse(vec![10_000.into(), 20_000.into()])
    );
    assert_eq!(
        trading_contract
            .get_market(&mut chain, &admin, &token_contract.contract_address())
            .expect("get market")
            .parse_return_value::<Market>()
            .expect("parse market"),
        Market::Transfer(TransferMarket {
            token_id:            TOKEN_ID,
            liquidity_provider:  liquidity_provider.address,
            buy_rate:            rate,
            sell_rate:           rate,
            // 10 Initial + 10 Sold
            max_token_amount:    TokenAmountU64(20),
            // 10_000 Initial + 10_000 Sold
            max_currency_amount: TokenAmountU64(0),
        })
    );

    let buyer = seller;
    euroe_contract
        .update_operator_single(&mut chain, &buyer, &UpdateOperator {
            update:   OperatorUpdate::Add,
            operator: trading_contract.contract_address().into(),
        })
        .expect("update operator");
    trading_contract
        .buy(&mut chain, &buyer, &ExchangeParams {
            amount: TokenAmountU64(10),
            rate,
            contract: token_contract.contract_address(),
        })
        .expect("buy");
    trading_contract
        .buy(&mut chain, &buyer, &ExchangeParams {
            amount: TokenAmountU64(10),
            rate,
            contract: token_contract.contract_address(),
        })
        .expect_err("should fail");
    assert_eq!(
        token_contract
            .balance_of(&chain, &admin, &BalanceOfQueryParams {
                queries: vec![
                    BalanceOfQuery {
                        token_id: TOKEN_ID,
                        address:  buyer.address.into(),
                    },
                    BalanceOfQuery {
                        token_id: TOKEN_ID,
                        address:  liquidity_provider.address.into(),
                    }
                ],
            })
            .expect("balance of"),
        BalanceOfQueryResponse(vec![50.into(), 0.into()])
    );
    assert_eq!(
        euroe_contract
            .balance_of(&chain, &admin, &BalanceOfQueryParams {
                queries: vec![
                    BalanceOfQuery {
                        token_id: euroe_token_id,
                        address:  buyer.address.into(),
                    },
                    BalanceOfQuery {
                        token_id: euroe_token_id,
                        address:  liquidity_provider.address.into(),
                    }
                ],
            })
            .expect("balance of"),
        BalanceOfQueryResponse(vec![0.into(), 30_000.into()])
    );
    assert_eq!(
        trading_contract
            .get_market(&mut chain, &admin, &token_contract.contract_address())
            .expect("get market")
            .parse_return_value::<Market>()
            .expect("parse market"),
        Market::Transfer(TransferMarket {
            token_id:            TOKEN_ID,
            liquidity_provider:  liquidity_provider.address,
            buy_rate:            rate,
            sell_rate:           rate,
            // 20 Initial + 10 Sold
            max_token_amount:    TokenAmountU64(10),
            // 0 Initial + 10_000 Sold
            max_currency_amount: TokenAmountU64(10_000),
        })
    );
}

#[test]
pub fn test_flow_mint_sft_multi() {
    let admin = Account::new(ADMIN, DEFAULT_ACC_BALANCE);
    let mut chain = Chain::new();
    chain
        .tick_block_time(Duration::from_millis(1))
        .expect("tick block time");
    let buyer = Account::new(HOLDER, DEFAULT_ACC_BALANCE);
    chain.create_account(buyer.clone());
    let liquidity_provider = Account::new(HOLDER_2, DEFAULT_ACC_BALANCE);
    chain.create_account(liquidity_provider.clone());
    let (euroe_contract, ir_contract, compliance_contract) =
        setup_chain(&mut chain, &admin, &COMPLIANT_NATIONALITIES);

    let euroe_token_id = TokenIdUnit();
    let trading_contract =
        P2PTradeTestClient::init(&mut chain, &admin, &security_p2p_trading::InitParam {
            currency: TokenUId {
                id:       euroe_token_id,
                contract: euroe_contract.contract_address(),
            },
            agents:   vec![],
        })
        .expect("init trading contract");
    euroe_contract
        .mint(&mut chain, &admin, &euroe::MintParams {
            owner:  buyer.address.into(),
            amount: TokenAmountU64(20_000),
        })
        .expect("euroe mint");
    euroe_contract
        .update_operator_single(&mut chain, &buyer, &UpdateOperator {
            update:   OperatorUpdate::Add,
            operator: trading_contract.contract_address().into(),
        })
        .expect("update operator");
    ir_contract
        .register_identity(&mut chain, &admin, &RegisterIdentityParams {
            address:  buyer.address.into(),
            identity: Identity {
                credentials: vec![],
                attributes:  vec![IdentityAttribute {
                    tag:   NATIONALITY.0,
                    value: COMPLIANT_NATIONALITIES[1].to_string(),
                }],
            },
        })
        .expect("register identity");
    let token_contract = create_token_contract_multi(
        &mut chain,
        &admin,
        compliance_contract,
        ir_contract.contract_address(),
        vec![AgentWithRoles {
            address: trading_contract.contract_address().into(),
            roles:   vec![
                security_sft_multi::types::AgentRole::AddToken,
                security_sft_multi::types::AgentRole::Mint,
            ],
        }],
    );
    let rate = Rate::new(1000, 1).unwrap();
    let now = chain.block_time();
    trading_contract
        .add_market(&mut chain, &admin, &AddMarketParams {
            token_contract: token_contract.contract_address(),
            market:         Market::Mint(MintMarket {
                liquidity_provider: liquidity_provider.address,
                rate,
                token_id: TokenIdCalculation {
                    diff:   Duration::from_days(1), // 1 day
                    start:         now,
                    base_token_id: TokenIdU64(0),
                },
                token_metadata_url: security_sft_multi::types::ContractMetadataUrl {
                    hash: None,
                    url:  METADATA_URL_SFT_REWARDS.to_string(),
                },
                max_token_amount: TokenAmountU64(20),
            }),
        })
        .expect("add mint market");
    trading_contract
        .mint(&mut chain, &buyer, &MintParams {
            amount: TokenAmountU64(10),
            rate,
            token_contract: token_contract.contract_address(),
        })
        .expect("mint");
    assert_eq!(
        token_contract
            .balance_of(&chain, &admin, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    token_id: TokenIdU64(0),
                    address:  buyer.address.into(),
                }],
            })
            .expect("balance of"),
        BalanceOfQueryResponse(vec![10.into()])
    );
    assert_eq!(
        euroe_contract
            .balance_of(&chain, &admin, &BalanceOfQueryParams {
                queries: vec![
                    BalanceOfQuery {
                        token_id: euroe_token_id,
                        address:  buyer.address.into(),
                    },
                    BalanceOfQuery {
                        token_id: euroe_token_id,
                        address:  liquidity_provider.address.into(),
                    }
                ],
            })
            .expect("balance of"),
        BalanceOfQueryResponse(vec![10_000.into(), 10_000.into()])
    );

    chain
        .tick_block_time(Duration::from_days(1))
        .expect("tick block time");
    trading_contract
        .mint(&mut chain, &buyer, &MintParams {
            amount: TokenAmountU64(10),
            rate,
            token_contract: token_contract.contract_address(),
        })
        .expect("mint");
    assert_eq!(
        token_contract
            .balance_of(&chain, &admin, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    token_id: TokenIdU64(1),
                    address:  buyer.address.into(),
                }],
            })
            .expect("balance of"),
        BalanceOfQueryResponse(vec![10.into()])
    );
    assert_eq!(
        euroe_contract
            .balance_of(&chain, &admin, &BalanceOfQueryParams {
                queries: vec![
                    BalanceOfQuery {
                        token_id: euroe_token_id,
                        address:  buyer.address.into(),
                    },
                    BalanceOfQuery {
                        token_id: euroe_token_id,
                        address:  liquidity_provider.address.into(),
                    }
                ],
            })
            .expect("balance of"),
        BalanceOfQueryResponse(vec![0.into(), 20_000.into()])
    );

    trading_contract
        .mint(&mut chain, &buyer, &MintParams {
            amount: TokenAmountU64(10),
            rate,
            token_contract: token_contract.contract_address(),
        })
        .expect_err("mint should fail");
    chain
        .tick_block_time(Duration::from_days(1))
        .expect("tick block time");
    trading_contract
        .mint(&mut chain, &buyer, &MintParams {
            amount: TokenAmountU64(10),
            rate,
            token_contract: token_contract.contract_address(),
        })
        .expect_err("mint should fail");
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
