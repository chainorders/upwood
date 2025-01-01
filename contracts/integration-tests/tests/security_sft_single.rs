#![cfg(test)]

use cis2_security::{Cis2SecurityTestClient, Cis2TestClient};
use compliance::init_nationalities;
use concordium_cis2::{
    AdditionalData, BalanceOfQuery, BalanceOfQueryParams, BalanceOfQueryResponse, TokenAmountU64,
    TokenIdUnit, Transfer,
};
use concordium_protocols::concordium_cis2_security::{
    AgentWithRoles, BurnParams, FreezeParam, FreezeParams, Identity, PauseParams, SecurityParams,
    TokenAmountSecurity,
};
use concordium_rwa_identity_registry::types::{IdentityAttribute, RegisterIdentityParams};
use concordium_smart_contract_testing::*;
use concordium_std::attributes::NATIONALITY;
use contract_base::{ContractPayloads, ContractTestClient};
use identity_registry::IdentityRegistryTestClient;
use integration_tests::*;
use security_sft_single::types::*;
use security_sft_single_client::SftSingleTestClient;

const TOKEN_ID: TokenIdUnit = TokenIdUnit();
const METADATA_URL: &str = "example.com";
const ADMIN: AccountAddress = AccountAddress([0; 32]);
const AGENT_MINT: AccountAddress = AccountAddress([1; 32]);
const HOLDER: AccountAddress = AccountAddress([2; 32]);
const HOLDER_2: AccountAddress = AccountAddress([3; 32]);
const HOLDER_3: AccountAddress = AccountAddress([4; 32]);
const AGENT_FORCED_TRANSFER: AccountAddress = AccountAddress([5; 32]);
const AGENT_FORCED_BURN: AccountAddress = AccountAddress([6; 32]);
const COMPLIANT_NATIONALITIES: [&str; 2] = ["IN", "US"];
const DEFAULT_ACC_BALANCE: Amount = Amount {
    micro_ccd: 1_000_000_000_u64,
};

#[test]
fn mint() {
    let admin = Account::new(ADMIN, DEFAULT_ACC_BALANCE);
    let mut chain = Chain::new();
    let (_, ir_contract, compliance_contract) =
        setup_chain(&mut chain, &admin, &COMPLIANT_NATIONALITIES);
    let agent_mint = Account::new(AGENT_MINT, DEFAULT_ACC_BALANCE);
    chain.create_account(agent_mint.clone());
    let non_agent_mint = Account::new(AccountAddress([99; 32]), DEFAULT_ACC_BALANCE);
    chain.create_account(non_agent_mint.clone());
    let non_agent = Account::new(AccountAddress([98; 32]), DEFAULT_ACC_BALANCE);
    chain.create_account(non_agent.clone());

    let token_contract = create_token_contract(
        &mut chain,
        &admin,
        compliance_contract,
        ir_contract.contract_address(),
    );

    token_contract
        .add_agent(&mut chain, &admin, &AgentWithRoles {
            address: Address::Account(agent_mint.address),
            roles:   vec![AgentRole::Mint],
        })
        .expect("should add agent");
    token_contract
        .add_agent(&mut chain, &admin, &AgentWithRoles {
            address: Address::Account(non_agent_mint.address),
            roles:   vec![AgentRole::Pause],
        })
        .expect("should add agent");

    let holder = Account::new(HOLDER, DEFAULT_ACC_BALANCE);
    chain.create_account(holder.clone());
    ir_contract
        .register_identity(&mut chain, &admin, RegisterIdentityParams {
            address:  Address::Account(holder.address),
            identity: Identity {
                credentials: vec![],
                attributes:  vec![IdentityAttribute {
                    tag:   NATIONALITY.0,
                    value: COMPLIANT_NATIONALITIES[1].to_string(),
                }],
            },
        })
        .expect("register identity");
    let holder_2 = Account::new(HOLDER_2, DEFAULT_ACC_BALANCE);
    chain.create_account(holder_2.clone());
    // Holder 2 is not registered with the Identity Registry

    token_contract
        .mint(&mut chain, &non_agent_mint, &MintParams {
            owners:   vec![MintParam {
                amount:  TokenAmountSecurity::new_un_frozen(TokenAmountU64(10)),
                address: holder.address.into(),
            }],
            token_id: TOKEN_ID,
        })
        .expect_err("non-agent-mint minted");
    token_contract
        .mint(&mut chain, &non_agent, &MintParams {
            owners:   vec![MintParam {
                amount:  TokenAmountSecurity::new_un_frozen(TokenAmountU64(10)),
                address: holder.address.into(),
            }],
            token_id: TOKEN_ID,
        })
        .expect_err("non-agent minted");
    token_contract
        .mint(&mut chain, &agent_mint, &MintParams {
            owners:   vec![MintParam {
                amount:  TokenAmountSecurity::new_un_frozen(TokenAmountU64(10)),
                address: holder_2.address.into(),
            }],
            token_id: TOKEN_ID,
        })
        .expect_err("non-compliant holder minted");
    token_contract
        .mint(&mut chain, &agent_mint, &MintParams {
            owners:   vec![MintParam {
                amount:  TokenAmountSecurity::new_un_frozen(TokenAmountU64(10)),
                address: holder.address.into(),
            }],
            token_id: TOKEN_ID,
        })
        .expect("should mint");
    assert_eq!(
        token_contract
            .balance_of(&chain, &holder, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  Address::Account(holder.address),
                    token_id: TOKEN_ID,
                },],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![TokenAmountU64(10)])
    );
}

#[test]
fn burn() {
    let admin = Account::new(ADMIN, DEFAULT_ACC_BALANCE);
    let mut chain = Chain::new();
    let (_, ir_contract, compliance_contract) =
        setup_chain(&mut chain, &admin, &COMPLIANT_NATIONALITIES);
    let token_contract = create_token_contract(
        &mut chain,
        &admin,
        compliance_contract,
        ir_contract.contract_address(),
    );
    let holder = Account::new(HOLDER, DEFAULT_ACC_BALANCE);
    chain.create_account(holder.clone());
    let holder_2 = Account::new(HOLDER_2, DEFAULT_ACC_BALANCE);
    chain.create_account(holder_2.clone());

    ir_contract
        .register_identity(&mut chain, &admin, RegisterIdentityParams {
            address:  Address::Account(holder.address),
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
            address:  Address::Account(holder_2.address),
            identity: Identity {
                credentials: vec![],
                attributes:  vec![IdentityAttribute {
                    tag:   NATIONALITY.0,
                    value: COMPLIANT_NATIONALITIES[1].to_string(),
                }],
            },
        })
        .expect("register identity");

    token_contract
        .mint(&mut chain, &admin, &MintParams {
            owners:   vec![MintParam {
                amount:  TokenAmountSecurity::new_un_frozen(TokenAmountU64(50)),
                address: holder.address.into(),
            }],
            token_id: TOKEN_ID,
        })
        .expect("should mint");
    token_contract
        .freeze(&mut chain, &admin, &FreezeParams {
            owner:  Address::Account(holder.address),
            tokens: vec![FreezeParam {
                token_id:     TOKEN_ID,
                token_amount: TokenAmountU64(10),
            }],
        })
        .expect("should freeze");
    token_contract
        .burn(
            &mut chain,
            &holder_2,
            &BurnParams(vec![Burn {
                amount:   TokenAmountU64(5),
                owner:    Address::Account(holder.address),
                token_id: TOKEN_ID,
            }]),
        )
        .expect_err("non-owner burned");
    token_contract
        .burn(
            &mut chain,
            &holder,
            &BurnParams(vec![Burn {
                amount:   TokenAmountU64(41),
                owner:    Address::Account(holder.address),
                token_id: TOKEN_ID,
            }]),
        )
        .expect_err("burned frozen");
    token_contract
        .burn(
            &mut chain,
            &holder,
            &BurnParams(vec![Burn {
                amount:   TokenAmountU64(5),
                owner:    Address::Account(holder.address),
                token_id: TOKEN_ID,
            }]),
        )
        .expect("should burn");

    assert_eq!(
        token_contract
            .balance_of(&chain, &holder, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  Address::Account(holder.address),
                    token_id: TOKEN_ID,
                },],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![TokenAmountU64(45)])
    );
    token_contract
        .burn(
            &mut chain,
            &holder,
            &BurnParams(vec![Burn {
                amount:   TokenAmountU64(46),
                owner:    Address::Account(holder.address),
                token_id: TOKEN_ID,
            }]),
        )
        .expect_err("burned more than minted");
    token_contract
        .burn(
            &mut chain,
            &holder_2,
            &BurnParams(vec![Burn {
                amount:   TokenAmountU64(5),
                owner:    Address::Account(holder_2.address),
                token_id: TOKEN_ID,
            }]),
        )
        .expect_err("non-existing holder burned");
    token_contract
        .un_freeze(&mut chain, &admin, &FreezeParams {
            owner:  Address::Account(holder.address),
            tokens: vec![FreezeParam {
                token_id:     TOKEN_ID,
                token_amount: TokenAmountU64(9),
            }],
        })
        .expect("should unfreeze");
    token_contract
        .burn(
            &mut chain,
            &holder,
            &BurnParams(vec![Burn {
                amount:   TokenAmountU64(41),
                owner:    Address::Account(holder.address),
                token_id: TOKEN_ID,
            }]),
        )
        .expect("should burn");
    assert_eq!(
        token_contract
            .balance_of(&chain, &holder, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  Address::Account(holder.address),
                    token_id: TOKEN_ID,
                },],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![TokenAmountU64(4)])
    );

    token_contract
        .pause(&mut chain, &admin, &PauseParams {
            tokens: vec![PauseParam { token_id: TOKEN_ID }],
        })
        .expect("should pause");
    token_contract
        .burn(
            &mut chain,
            &holder,
            &BurnParams(vec![Burn {
                amount:   TokenAmountU64(1),
                owner:    Address::Account(holder.address),
                token_id: TOKEN_ID,
            }]),
        )
        .expect_err("burned paused token");
}

#[test]
fn forced_burn() {
    let admin = Account::new(ADMIN, DEFAULT_ACC_BALANCE);
    let mut chain = Chain::new();
    let (_, ir_contract, compliance_contract) =
        setup_chain(&mut chain, &admin, &COMPLIANT_NATIONALITIES);
    let token_contract = create_token_contract(
        &mut chain,
        &admin,
        compliance_contract,
        ir_contract.contract_address(),
    );

    let agent_forced_burn = Account::new(AGENT_FORCED_BURN, DEFAULT_ACC_BALANCE);
    chain.create_account(agent_forced_burn.clone());

    token_contract
        .add_agent(&mut chain, &admin, &AgentWithRoles {
            address: Address::Account(agent_forced_burn.address),
            roles:   vec![AgentRole::ForcedBurn],
        })
        .expect("should add agent");
    let holder = Account::new(HOLDER, DEFAULT_ACC_BALANCE);
    chain.create_account(holder.clone());
    let holder_2 = Account::new(HOLDER_2, DEFAULT_ACC_BALANCE);
    chain.create_account(holder_2.clone());

    ir_contract
        .register_identity(&mut chain, &admin, RegisterIdentityParams {
            address:  Address::Account(holder.address),
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
            address:  Address::Account(holder_2.address),
            identity: Identity {
                credentials: vec![],
                attributes:  vec![IdentityAttribute {
                    tag:   NATIONALITY.0,
                    value: COMPLIANT_NATIONALITIES[1].to_string(),
                }],
            },
        })
        .expect("register identity");

    token_contract
        .mint(&mut chain, &admin, &MintParams {
            owners:   vec![MintParam {
                amount:  TokenAmountSecurity::new_un_frozen(TokenAmountU64(50)),
                address: holder.address.into(),
            }],
            token_id: TOKEN_ID,
        })
        .expect("should mint");
    token_contract
        .freeze(&mut chain, &admin, &FreezeParams {
            owner:  Address::Account(holder.address),
            tokens: vec![FreezeParam {
                token_id:     TOKEN_ID,
                token_amount: 10.into(),
            }],
        })
        .expect("should freeze");
    assert_eq!(
        token_contract
            .balance_of_frozen(&mut chain, &holder, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  holder.address.into(),
                    token_id: TOKEN_ID,
                },],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![10.into()])
    );
    assert_eq!(
        token_contract
            .balance_of_un_frozen(&mut chain, &holder, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  holder.address.into(),
                    token_id: TOKEN_ID,
                },],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![40.into()])
    );
    token_contract
        .burn(
            &mut chain,
            &holder,
            &BurnParams(vec![Burn {
                token_id: TOKEN_ID,
                amount:   50.into(),
                owner:    holder.address.into(),
            }]),
        )
        .expect_err("non-agent forced burn");
    token_contract
        .burn(
            &mut chain,
            &agent_forced_burn,
            &BurnParams(vec![Burn {
                token_id: TOKEN_ID,
                amount:   10.into(),
                owner:    holder.address.into(),
            }]),
        )
        .expect("should burn");
    assert_eq!(
        token_contract
            .balance_of_frozen(&mut chain, &holder, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  holder.address.into(),
                    token_id: TOKEN_ID,
                },],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![10.into()])
    );
    assert_eq!(
        token_contract
            .balance_of_un_frozen(&mut chain, &holder, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  holder.address.into(),
                    token_id: TOKEN_ID,
                },],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![30.into()])
    );

    token_contract
        .burn(
            &mut chain,
            &agent_forced_burn,
            &BurnParams(vec![Burn {
                token_id: TOKEN_ID,
                amount:   30.into(),
                owner:    holder.address.into(),
            }]),
        )
        .expect("should burn");
    assert_eq!(
        token_contract
            .balance_of_frozen(&mut chain, &holder, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  holder.address.into(),
                    token_id: TOKEN_ID,
                },],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![10.into()])
    );
    assert_eq!(
        token_contract
            .balance_of_un_frozen(&mut chain, &holder, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  holder.address.into(),
                    token_id: TOKEN_ID,
                },],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![0.into()])
    );
    token_contract
        .un_freeze(&mut chain, &admin, &FreezeParams {
            owner:  holder.address.into(),
            tokens: vec![FreezeParam {
                token_id:     TOKEN_ID,
                token_amount: 10.into(),
            }],
        })
        .expect("should unfreeze");
    assert_eq!(
        token_contract
            .balance_of_frozen(&mut chain, &holder, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  holder.address.into(),
                    token_id: TOKEN_ID,
                },],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![0.into()])
    );
    assert_eq!(
        token_contract
            .balance_of_un_frozen(&mut chain, &holder, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  holder.address.into(),
                    token_id: TOKEN_ID,
                },],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![10.into()])
    );
    token_contract
        .burn(
            &mut chain,
            &agent_forced_burn,
            &BurnParams(vec![Burn {
                token_id: TOKEN_ID,
                amount:   11.into(),
                owner:    holder.address.into(),
            }]),
        )
        .expect_err("burned more than minted");

    token_contract
        .pause(&mut chain, &admin, &PauseParams {
            tokens: vec![PauseParam { token_id: TOKEN_ID }],
        })
        .expect("should pause");
}

#[test]
fn transfer() {
    let admin = Account::new(ADMIN, DEFAULT_ACC_BALANCE);
    let mut chain = Chain::new();
    let (_, ir_contract, compliance_contract) =
        setup_chain(&mut chain, &admin, &COMPLIANT_NATIONALITIES);
    let token_contract = create_token_contract(
        &mut chain,
        &admin,
        compliance_contract,
        ir_contract.contract_address(),
    );
    let holder = Account::new(HOLDER, DEFAULT_ACC_BALANCE);
    chain.create_account(holder.clone());
    let holder_2 = Account::new(HOLDER_2, DEFAULT_ACC_BALANCE);
    chain.create_account(holder_2.clone());
    let holder_3 = Account::new(HOLDER_3, DEFAULT_ACC_BALANCE);
    chain.create_account(holder_2.clone());

    ir_contract
        .register_identity(&mut chain, &admin, RegisterIdentityParams {
            address:  Address::Account(holder.address),
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
            address:  Address::Account(holder_2.address),
            identity: Identity {
                credentials: vec![],
                attributes:  vec![IdentityAttribute {
                    tag:   NATIONALITY.0,
                    value: COMPLIANT_NATIONALITIES[1].to_string(),
                }],
            },
        })
        .expect("register identity");
    // holder 3 is not registered with the Identity Registry

    token_contract
        .mint(&mut chain, &admin, &MintParams {
            owners:   vec![MintParam {
                amount:  TokenAmountSecurity::new_un_frozen(TokenAmountU64(50)),
                address: holder.address.into(),
            }],
            token_id: TOKEN_ID,
        })
        .expect("should mint");

    token_contract
        .transfer_single(&mut chain, &holder, Transfer {
            token_id: TOKEN_ID,
            amount:   51.into(),
            from:     Address::Account(holder.address),
            to:       holder_2.address.into(),
            data:     AdditionalData::empty(),
        })
        .expect_err("transferred more than minted");
    token_contract
        .transfer_single(&mut chain, &holder, Transfer {
            token_id: TOKEN_ID,
            amount:   0.into(),
            from:     Address::Account(holder.address),
            to:       holder_2.address.into(),
            data:     AdditionalData::empty(),
        })
        .expect_err("transferred 0");
    token_contract
        .freeze(&mut chain, &admin, &FreezeParams {
            owner:  Address::Account(holder.address),
            tokens: vec![FreezeParam {
                token_id:     TOKEN_ID,
                token_amount: 10.into(),
            }],
        })
        .expect("should freeze");
    token_contract
        .transfer_single(&mut chain, &holder, Transfer {
            token_id: TOKEN_ID,
            amount:   41.into(),
            from:     Address::Account(holder.address),
            to:       holder_2.address.into(),
            data:     AdditionalData::empty(),
        })
        .expect_err("transferred frozen");
    token_contract
        .transfer_single(&mut chain, &holder, Transfer {
            token_id: TOKEN_ID,
            amount:   1.into(),
            from:     Address::Account(holder.address),
            to:       holder_3.address.into(),
            data:     AdditionalData::empty(),
        })
        .expect_err("transferred to non-compliant");
    token_contract
        .transfer_single(&mut chain, &holder, Transfer {
            token_id: TOKEN_ID,
            amount:   30.into(),
            from:     Address::Account(holder.address),
            to:       holder_2.address.into(),
            data:     AdditionalData::empty(),
        })
        .expect("should transfer");
    assert_eq!(
        token_contract
            .balance_of(&chain, &holder, &BalanceOfQueryParams {
                queries: vec![
                    BalanceOfQuery {
                        address:  holder.address.into(),
                        token_id: TOKEN_ID,
                    },
                    BalanceOfQuery {
                        address:  holder_2.address.into(),
                        token_id: TOKEN_ID,
                    }
                ],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![20.into(), 30.into()])
    );

    token_contract
        .pause(&mut chain, &admin, &PauseParams {
            tokens: vec![PauseParam { token_id: TOKEN_ID }],
        })
        .expect("should pause");
    token_contract
        .transfer_single(&mut chain, &holder, Transfer {
            token_id: TOKEN_ID,
            amount:   1.into(),
            from:     Address::Account(holder.address),
            to:       holder_2.address.into(),
            data:     AdditionalData::empty(),
        })
        .expect_err("transferred paused token");
}

#[test]
fn forced_transfer() {
    let admin = Account::new(ADMIN, DEFAULT_ACC_BALANCE);
    let mut chain = Chain::new();
    let (_, ir_contract, compliance_contract) =
        setup_chain(&mut chain, &admin, &COMPLIANT_NATIONALITIES);
    let token_contract = create_token_contract(
        &mut chain,
        &admin,
        compliance_contract,
        ir_contract.contract_address(),
    );
    let agent_forced_transfer = Account::new(AGENT_FORCED_TRANSFER, DEFAULT_ACC_BALANCE);
    chain.create_account(agent_forced_transfer.clone());

    token_contract
        .add_agent(&mut chain, &admin, &AgentWithRoles {
            address: Address::Account(agent_forced_transfer.address),
            roles:   vec![AgentRole::ForcedTransfer],
        })
        .expect("should add agent");
    let holder = Account::new(HOLDER, DEFAULT_ACC_BALANCE);
    chain.create_account(holder.clone());
    let holder_2 = Account::new(HOLDER_2, DEFAULT_ACC_BALANCE);
    chain.create_account(holder_2.clone());
    let holder_3 = Account::new(HOLDER_3, DEFAULT_ACC_BALANCE);
    chain.create_account(holder_2.clone());

    ir_contract
        .register_identity(&mut chain, &admin, RegisterIdentityParams {
            address:  Address::Account(holder.address),
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
            address:  Address::Account(holder_2.address),
            identity: Identity {
                credentials: vec![],
                attributes:  vec![IdentityAttribute {
                    tag:   NATIONALITY.0,
                    value: COMPLIANT_NATIONALITIES[1].to_string(),
                }],
            },
        })
        .expect("register identity");
    // holder 3 is not registered with the Identity Registry

    token_contract
        .mint(&mut chain, &admin, &MintParams {
            owners:   vec![MintParam {
                amount:  TokenAmountSecurity::new_un_frozen(TokenAmountU64(50)),
                address: holder.address.into(),
            }],
            token_id: TOKEN_ID,
        })
        .expect("should mint");
    token_contract
        .freeze(&mut chain, &admin, &FreezeParams {
            owner:  Address::Account(holder.address),
            tokens: vec![FreezeParam {
                token_id:     TOKEN_ID,
                token_amount: 10.into(),
            }],
        })
        .expect("should freeze");
    assert_eq!(
        token_contract
            .balance_of_frozen(&mut chain, &holder, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  holder.address.into(),
                    token_id: TOKEN_ID,
                },],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![10.into()])
    );
    assert_eq!(
        token_contract
            .balance_of_un_frozen(&mut chain, &holder, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  holder.address.into(),
                    token_id: TOKEN_ID,
                },],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![40.into()])
    );

    token_contract
        .transfer_single(&mut chain, &agent_forced_transfer, Transfer {
            token_id: TOKEN_ID,
            amount:   51.into(),
            from:     holder.address.into(),
            to:       holder_2.address.into(),
            data:     AdditionalData::empty(),
        })
        .expect_err("transferred more than minted");
    token_contract
        .transfer_single(&mut chain, &agent_forced_transfer, Transfer {
            token_id: TOKEN_ID,
            amount:   1.into(),
            from:     holder.address.into(),
            to:       holder_3.address.into(),
            data:     AdditionalData::empty(),
        })
        .expect_err("transferred to non-compliant");
    token_contract
        .transfer_single(&mut chain, &agent_forced_transfer, Transfer {
            token_id: TOKEN_ID,
            amount:   1.into(),
            from:     holder.address.into(),
            to:       holder_2.address.into(),
            data:     AdditionalData::empty(),
        })
        .expect("should transfer");
    assert_eq!(
        token_contract
            .balance_of(&chain, &holder, &BalanceOfQueryParams {
                queries: vec![
                    BalanceOfQuery {
                        address:  holder.address.into(),
                        token_id: TOKEN_ID,
                    },
                    BalanceOfQuery {
                        address:  holder_2.address.into(),
                        token_id: TOKEN_ID,
                    }
                ],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![49.into(), 1.into()])
    );
    assert_eq!(
        token_contract
            .balance_of_frozen(&mut chain, &holder, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  holder.address.into(),
                    token_id: TOKEN_ID,
                },],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![10.into()])
    );
    assert_eq!(
        token_contract
            .balance_of_un_frozen(&mut chain, &holder, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  holder.address.into(),
                    token_id: TOKEN_ID,
                },],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![39.into()])
    );

    token_contract
        .transfer_single(&mut chain, &agent_forced_transfer, Transfer {
            token_id: TOKEN_ID,
            amount:   49.into(),
            from:     holder.address.into(),
            to:       holder_2.address.into(),
            data:     AdditionalData::empty(),
        })
        .expect("should transfer frozen");
    assert_eq!(
        token_contract
            .balance_of_frozen(&mut chain, &holder, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  holder.address.into(),
                    token_id: TOKEN_ID,
                },],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![0.into()])
    );
    assert_eq!(
        token_contract
            .balance_of_un_frozen(&mut chain, &holder, &BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  holder.address.into(),
                    token_id: TOKEN_ID,
                },],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![0.into()])
    );
    assert_eq!(
        token_contract
            .balance_of(&chain, &holder, &BalanceOfQueryParams {
                queries: vec![
                    BalanceOfQuery {
                        address:  holder.address.into(),
                        token_id: TOKEN_ID,
                    },
                    BalanceOfQuery {
                        address:  holder_2.address.into(),
                        token_id: TOKEN_ID,
                    }
                ],
            })
            .unwrap(),
        BalanceOfQueryResponse(vec![0.into(), 50.into()])
    );

    token_contract
        .pause(&mut chain, &admin, &PauseParams {
            tokens: vec![PauseParam { token_id: TOKEN_ID }],
        })
        .expect("should pause");
    token_contract
        .transfer_single(&mut chain, &agent_forced_transfer, Transfer {
            token_id: TOKEN_ID,
            amount:   1.into(),
            from:     holder_2.address.into(),
            to:       holder.address.into(),
            data:     AdditionalData::empty(),
        })
        .expect_err("transferred paused token");
}

fn create_token_contract(
    chain: &mut Chain,
    admin: &Account,
    compliance_contract: ContractAddress,
    ir_contract: ContractAddress,
) -> SftSingleTestClient {
    SftSingleTestClient::init(chain, admin, &InitParam {
        security:     Some(SecurityParams {
            compliance:        compliance_contract,
            identity_registry: ir_contract,
        }),
        metadata_url: ContractMetadataUrl {
            hash: None,
            url:  METADATA_URL.to_string(),
        },
        agents:       vec![],
    })
    .expect("init token contract")
}

fn setup_chain(
    chain: &mut Chain,
    admin: &Account,
    compliant_nationalities: &[&str],
) -> (ContractAddress, IdentityRegistryTestClient, ContractAddress) {
    chain.create_account(admin.clone());

    euroe::deploy_module(chain, admin);
    identity_registry::deploy_module(chain, admin);
    compliance::deploy_module(chain, admin);
    security_sft_single_client::deploy_module(chain, admin);
    security_p2p_trading_client::deploy_module(chain, admin);
    security_mint_fund_client::deploy_module(chain, admin);

    let euroe_contract = euroe::init(chain, admin)
        .expect("euroe init")
        .0
        .contract_address;
    let ir_contract =
        IdentityRegistryTestClient::init(chain, admin, &()).expect("identity registry init");

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

    (euroe_contract, ir_contract, compliance.contract_address)
}
