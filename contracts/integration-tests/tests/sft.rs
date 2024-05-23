#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]
pub mod utils;

use concordium_cis2::{
    AdditionalData, BalanceOfQuery, BalanceOfQueryParams, BalanceOfQueryResponse, Receiver,
    TokenAmountU32, TokenMetadataQueryParams, TokenMetadataQueryResponse, Transfer, TransferParams,
};
use concordium_rwa_security_nft::types::ContractMetadataUrl;
use concordium_rwa_security_sft::{error::Error, types::*};
use concordium_smart_contract_testing::{ed25519::PublicKey, *};
use concordium_std::{fail, Reject, ACCOUNT_ADDRESS_SIZE};
use utils::{
    cis2_conversions::to_token_id_vec,
    cis2_security_test_contract::ICis2SecurityTestContract,
    cis2_test_contract::{ICis2Contract, ICis2ContractExt},
    common::{
        init_identity_contracts, init_security_sft_contract,
        init_security_token_contracts,
    },
    consts::*,
    identity_registry::{IIdentityRegistryContract, IdentityRegistryContract},
    security_nft::ISecurityNftContractExt,
    security_sft::SecuritySftContract,
    test_contract_client::ITestContract,
    verifier::Verifier,
};

use crate::utils::security_sft::{ISecuritySftContract, ISecuritySftContractExt};

/// The address of the admin account
const ADMIN: AccountAddress = AccountAddress([0; ACCOUNT_ADDRESS_SIZE]);
/// Identity Registry Agent
const IR_AGENT: AccountAddress = AccountAddress([1; ACCOUNT_ADDRESS_SIZE]);
/// Token Contract Agent
const AGENT: AccountAddress = AccountAddress([2; ACCOUNT_ADDRESS_SIZE]);

#[test]
fn init() {
    let mut chain = Chain::new();
    let admin = &create_default_admin(&mut chain);
    let (identity_registry, compliance_contract) =
        init_identity_contracts(&mut chain, admin, vec!["IN".to_owned(), "US".to_owned()]);

    let token_contract = init_security_sft_contract(
        &mut chain,
        admin,
        &identity_registry,
        &compliance_contract,
        vec![],
    );

    // account initializing the contract is the default first agent of the contract
    let is_agent: bool = token_contract
        .is_agent()
        .invoke(&mut chain, admin, &Address::Account(admin.address))
        .expect("Is Agent")
        .parse_return_value()
        .expect("Parse Is Agent");
    assert!(is_agent);
}

#[test]
fn add_agent() {
    let mut chain = Chain::new();
    let admin = &create_default_admin(&mut chain);
    let new_agent = Account::new_with_keys(
        AccountAddress([1; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(new_agent.clone());

    let (token_contract, _, _) = setup_contract_with_admin(&mut chain, admin);
    let agent_address = Address::Account(new_agent.address);

    token_contract.add_agent().update(&mut chain, admin, &agent_address).expect("Add Agent");

    let agents: Vec<Address> = token_contract
        .agents()
        .invoke(&mut chain, admin, &())
        .expect("Agents")
        .parse_return_value()
        .expect("Parse Agents");
    assert!(agents.contains(&agent_address));

    let is_agent: bool = token_contract
        .is_agent()
        .invoke(&mut chain, admin, &agent_address)
        .expect("Is Agent")
        .parse_return_value()
        .expect("Parse Is Agent");
    assert!(is_agent);
}

#[test]
fn add_agent_non_admin() {
    let mut chain = Chain::new();
    let admin = &create_default_admin(&mut chain);
    let new_agent = Account::new_with_keys(
        AccountAddress([1; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(new_agent.clone());

    let (token_contract, _, _) = setup_contract_with_admin(&mut chain, admin);
    let agent_address = Address::Account(new_agent.address);

    let result = token_contract
        .add_agent()
        .update(&mut chain, &new_agent, &agent_address)
        .expect_err("Add Agent");

    if let ContractInvokeError {
        kind: ContractInvokeErrorKind::ExecutionError {
            failure_kind,
        },
        ..
    } = result
    {
        assert_eq!(failure_kind, InvokeFailure::ContractReject {
            code: Reject::from(Error::Unauthorized).error_code.into(),
            data: vec![],
        });
    } else {
        fail!("Expected ContractInvokeErrorKind::ExecutionError");
    }
}

#[test]
fn remove_agent() {
    let mut chain = Chain::new();
    let admin = &create_default_admin(&mut chain);
    let (token_contract, agent, _) = setup_contract_with_admin(&mut chain, admin);
    let agent_address = Address::Account(agent.address);

    token_contract.remove_agent().update(&mut chain, admin, &agent_address).expect("Remove Agent");

    let agents: Vec<Address> = token_contract
        .agents()
        .invoke(&mut chain, &agent, &())
        .expect("Agents")
        .parse_return_value()
        .expect("Parse Agents");
    assert!(!agents.contains(&agent_address));

    let is_agent: bool = token_contract
        .is_agent()
        .invoke(&mut chain, &agent, &Address::Account(agent.address))
        .expect("Is Agent")
        .parse_return_value()
        .expect("Parse Is Agent");
    assert!(!is_agent);
}

#[test]
fn remove_agent_non_admin() {
    let mut chain = Chain::new();
    let admin = &create_default_admin(&mut chain);
    let (token_contract, agent, _) = setup_contract_with_admin(&mut chain, admin);
    let agent_address = Address::Account(agent.address);

    let result = token_contract
        .remove_agent()
        .update(&mut chain, &agent, &agent_address)
        .expect_err("Remove Agent");

    if let ContractInvokeError {
        kind: ContractInvokeErrorKind::ExecutionError {
            failure_kind,
        },
        ..
    } = result
    {
        assert_eq!(failure_kind, InvokeFailure::ContractReject {
            code: Reject::from(Error::Unauthorized).error_code.into(),
            data: vec![],
        });
    } else {
        fail!("Expected ContractInvokeErrorKind::ExecutionError");
    }
}

#[test]
fn check_is_agent() {
    let mut chain = Chain::new();
    let admin = &create_default_admin(&mut chain);
    let (token_contract, agent, _) = setup_contract_with_admin(&mut chain, admin);

    let is_agent: bool = token_contract
        .is_agent()
        .invoke(&mut chain, &agent, &Address::Account(agent.address))
        .expect("Is Agent")
        .parse_return_value()
        .expect("Parse Is Agent");
    assert!(is_agent);
}

#[test]
fn add_token() {
    let compliant_nationalities = vec!["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let owner = Account::new_with_keys(
        AccountAddress([2; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(owner.clone());

    let admin = Account::new_with_keys(
        AccountAddress([0; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(admin.clone());

    let ir_agent = Account::new_with_keys(
        AccountAddress([1; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(ir_agent.clone());

    let (ir_contract, compliance_contract) =
        init_identity_contracts(&mut chain, &admin, compliant_nationalities.clone());
    ir_contract
        .add_agent()
        .update(&mut chain, &admin, &Address::Account(ir_agent.address))
        .expect("Add agent");
    let verifier = Verifier {
        account:           ir_agent.clone(),
        identity_registry: ir_contract.clone(),
    };

    let (nft_contract, sft_contract) = init_security_token_contracts(
        &mut chain,
        &admin,
        &ir_contract,
        &compliance_contract,
        vec![],
    )
    .expect("Init security token contracts");

    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(owner.address), compliant_nationalities[1].clone()),
            (
                Address::Contract(sft_contract.contract_address()),
                compliant_nationalities[1].clone(),
            ),
        ])
        .expect("Register nationalities");

    let (nft_token, sft_token) = sft_contract.add_single_token_nft_mint(
        &mut chain,
        &admin,
        &nft_contract,
        &owner,
        ContractMetadataUrl {
            url:  "ipfs:nft".to_string(),
            hash: None,
        },
        ContractMetadataUrl {
            url:  "ipfs:sft".to_string(),
            hash: None,
        },
        1000,
    );

    let balance_of_nft = nft_contract
        .balance_of_invoke(&mut chain, &owner, &BalanceOfQueryParams {
            queries: vec![BalanceOfQuery {
                address:  Address::Account(owner.address),
                token_id: nft_token,
            }],
        })
        .expect("Balance of");
    // Actual transfer of token is still not done. Only the token is minted and
    // added to the sft / fractionalizer contract.
    assert_eq!(balance_of_nft, BalanceOfQueryResponse(vec![1.into()]));

    let sft_metadata: TokenMetadataQueryResponse = sft_contract
        .token_metadata()
        .invoke(&mut chain, &owner, &TokenMetadataQueryParams {
            queries: vec![sft_token],
        })
        .expect("Token Metadata")
        .parse_return_value()
        .expect("Parse Token Metadata");

    assert_eq!(
        sft_metadata.0.first(),
        Some(
            &ContractMetadataUrl {
                url:  "ipfs:sft".to_string(),
                hash: None,
            }
            .into()
        )
    )
}

#[test]
fn add_token_non_agent() {
    let compliant_nationalities = vec!["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let owner = Account::new_with_keys(
        AccountAddress([2; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(owner.clone());

    let admin = Account::new_with_keys(
        AccountAddress([0; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(admin.clone());

    let ir_agent = Account::new_with_keys(
        AccountAddress([1; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(ir_agent.clone());

    let (ir_contract, compliance_contract) =
        init_identity_contracts(&mut chain, &admin, compliant_nationalities.clone());
    ir_contract
        .add_agent()
        .update(&mut chain, &admin, &Address::Account(ir_agent.address))
        .expect("Add agent");
    let verifier = Verifier {
        account:           ir_agent.clone(),
        identity_registry: ir_contract.clone(),
    };

    let (nft_contract, sft_contract) = init_security_token_contracts(
        &mut chain,
        &admin,
        &ir_contract,
        &compliance_contract,
        vec![],
    )
    .expect("Init security token contracts");

    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(owner.address), compliant_nationalities[1].clone()),
            (
                Address::Contract(sft_contract.contract_address()),
                compliant_nationalities[1].clone(),
            ),
        ])
        .expect("Register nationalities");
    let nft_token_id = nft_contract
        .mint_single_update(
            &mut chain,
            &admin,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("NFT: Mint");
    let result = sft_contract
        .add_tokens()
        .update(&mut chain, &owner, &AddParams {
            tokens: vec![AddParam {
                deposit_token_id: NftTokenUId {
                    contract: nft_contract.contract_address(),
                    id:       to_token_id_vec(nft_token_id),
                },
                metadata_url:     ContractMetadataUrl {
                    url:  "ipfs:sft".to_string(),
                    hash: None,
                },
                fractions_rate:   Rate {
                    numerator:   1000,
                    denominator: 1,
                },
            }],
        })
        .expect_err("Add token");

    if let ContractInvokeError {
        kind: ContractInvokeErrorKind::ExecutionError {
            failure_kind,
        },
        ..
    } = result
    {
        assert_eq!(failure_kind, InvokeFailure::ContractReject {
            code: Reject::from(Error::Unauthorized).error_code.into(),
            data: vec![],
        });
    } else {
        fail!("Expected ContractInvokeErrorKind::ExecutionError");
    }
}

#[test]
fn mint_via_transfer() {
    let compliant_nationalities = vec!["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let owner = Account::new_with_keys(
        AccountAddress([2; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(owner.clone());

    let admin = Account::new_with_keys(
        AccountAddress([0; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(admin.clone());

    let agent = Account::new_with_keys(
        AccountAddress([1; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(agent.clone());

    let (ir_contract, compliance_contract) =
        init_identity_contracts(&mut chain, &admin, compliant_nationalities.clone());
    ir_contract
        .add_agent()
        .update(&mut chain, &admin, &Address::Account(agent.address))
        .expect("Add agent");
    let verifier = Verifier {
        account:           agent.clone(),
        identity_registry: ir_contract.clone(),
    };

    let (nft_contract, sft_contract) = init_security_token_contracts(
        &mut chain,
        &admin,
        &ir_contract,
        &compliance_contract,
        vec![],
    )
    .expect("Init security token contracts");

    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(owner.address), compliant_nationalities[1].clone()),
            (
                Address::Contract(sft_contract.contract_address()),
                compliant_nationalities[1].clone(),
            ),
        ])
        .expect("Register nationalities");

    let (nft_token, sft_token) = sft_contract.mint_via_transfer(
        &mut chain,
        &admin,
        &nft_contract,
        &owner,
        concordium_rwa_security_nft::types::ContractMetadataUrl {
            url:  "ipfs:nft".to_string(),
            hash: None,
        },
        concordium_rwa_security_sft::types::ContractMetadataUrl {
            url:  "ipfs:sft".to_string(),
            hash: None,
        },
        1000,
    );

    let balance = *sft_contract
        .balance_of()
        .invoke(
            &mut chain,
            &owner,
            &concordium_rwa_security_sft::types::ContractBalanceOfQueryParams {
                queries: vec![concordium_rwa_security_sft::types::ContractBalanceOfQuery {
                    token_id: sft_token,
                    address:  Address::Account(owner.address),
                }],
            },
        )
        .map(|r| sft_contract.balance_of().parse_return_value(&r).expect("Parsing balance of"))
        .expect("Balance of")
        .0
        .first()
        .expect("First balance");
    assert_eq!(balance, TokenAmountU32(1000));

    let balance_deposited: NftTokenAmount = sft_contract
        .balance_of_deposited()
        .invoke(&mut chain, &owner, &BalanceOfDepositParams {
            token_id: NftTokenUId {
                contract: nft_contract.contract_address(),
                id:       to_token_id_vec(nft_token),
            },
            address:  owner.address,
        })
        .expect("Balance of deposited")
        .parse_return_value()
        .expect("Parse balance of deposited");
    assert_eq!(balance_deposited, 1.into());
}

#[test]
fn mint_via_deposit() {
    let compliant_nationalities = vec!["IN".to_owned(), "US".to_owned()];
    let admin = Account::new_with_keys(
        AccountAddress([0; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let agent = Account::new_with_keys(
        AccountAddress([1; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let owner = Account::new_with_keys(
        AccountAddress([2; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );

    let mut chain = Chain::new();
    [admin.clone(), agent.clone(), owner.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });

    let (ir_contract, compliance_contract) =
        init_identity_contracts(&mut chain, &admin, compliant_nationalities.clone());
    ir_contract
        .add_agent()
        .update(&mut chain, &admin, &Address::Account(agent.address))
        .expect("Add agent");
    let verifier = Verifier {
        account:           agent.clone(),
        identity_registry: ir_contract.clone(),
    };

    let (nft_contract, sft_contract) = init_security_token_contracts(
        &mut chain,
        &admin,
        &ir_contract,
        &compliance_contract,
        vec![],
    )
    .expect("Init security token contracts");

    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(owner.address), "US".to_string()),
            (Address::Contract(sft_contract.contract_address()), "US".to_string()),
        ])
        .expect("Register nationalities");

    let (nft_token, sft_token) = sft_contract.mint_via_deposit(
        &mut chain,
        &admin,
        &nft_contract,
        &owner,
        concordium_rwa_security_nft::types::ContractMetadataUrl {
            url:  "ipfs:nft".to_string(),
            hash: None,
        },
        concordium_rwa_security_sft::types::ContractMetadataUrl {
            url:  "ipfs:sft".to_string(),
            hash: None,
        },
        1000,
    );

    let balance = sft_contract
        .balance_of()
        .invoke(
            &mut chain,
            &owner,
            &concordium_rwa_security_sft::types::ContractBalanceOfQueryParams {
                queries: vec![concordium_rwa_security_sft::types::ContractBalanceOfQuery {
                    token_id: sft_token,
                    address:  Address::Account(owner.address),
                }],
            },
        )
        .map(|r| sft_contract.balance_of().parse_return_value(&r).expect("Parsing balance of"))
        .expect("Balance of");
    assert_eq!(balance, BalanceOfQueryResponse(vec![TokenAmountU32(1000)]));

    let balance: NftTokenAmount = sft_contract
        .balance_of_deposited()
        .invoke(&mut chain, &owner, &BalanceOfDepositParams {
            token_id: NftTokenUId {
                contract: nft_contract.contract_address(),
                id:       to_token_id_vec(nft_token),
            },
            address:  owner.address,
        })
        .expect("Balance of deposited")
        .parse_return_value()
        .expect("Parse balance of deposited");
    assert_eq!(balance, 1.into());
}

#[test]
fn transfer() {
    let compliant_nationalities = vec!["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let owner = Account::new_with_keys(
        AccountAddress([2; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(owner.clone());

    let admin = Account::new_with_keys(
        AccountAddress([0; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let agent = Account::new_with_keys(
        AccountAddress([1; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [admin.clone(), agent.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    let (ir_contract, compliance_contract) =
        init_identity_contracts(&mut chain, &admin, compliant_nationalities.clone());
    ir_contract
        .add_agent()
        .update(&mut chain, &admin, &Address::Account(agent.address))
        .expect("Add agent");
    let verifier = Verifier {
        account:           agent.clone(),
        identity_registry: ir_contract.clone(),
    };

    let admin = Account::new_with_keys(
        AccountAddress([5; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(admin.clone());
    let (nft_contract, sft_contract) = init_security_token_contracts(
        &mut chain,
        &admin,
        &ir_contract,
        &compliance_contract,
        vec![],
    )
    .expect("Init security token contracts");
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Contract(sft_contract.contract_address()),
            compliant_nationalities[1].clone(),
        )])
        .expect("Register nationalities");
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[1].clone(),
        )])
        .expect("Register Owner");
    let receiver = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(receiver.clone());
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(receiver.address),
            compliant_nationalities[1].clone(),
        )])
        .expect("Register Owner");

    let (nft_token, sft_token) = sft_contract.mint_via_transfer(
        &mut chain,
        &admin,
        &nft_contract,
        &owner,
        concordium_rwa_security_nft::types::ContractMetadataUrl {
            url:  "ipfs:nft".to_string(),
            hash: None,
        },
        concordium_rwa_security_sft::types::ContractMetadataUrl {
            url:  "ipfs:sft".to_string(),
            hash: None,
        },
        100,
    );
    let nft_balance = nft_contract
        .balance_of_single_invoke(
            &mut chain,
            &admin,
            nft_token,
            Address::Contract(sft_contract.contract_address()),
        )
        .expect("Balance of nft");
    assert_eq!(nft_balance, 1.into());

    sft_contract
        .transfer()
        .update(
            &mut chain,
            &owner,
            &TransferParams(vec![Transfer {
                from:     Address::Account(owner.address),
                to:       Receiver::Account(receiver.address),
                token_id: sft_token,
                amount:   10.into(),
                data:     AdditionalData::empty(),
            }]),
        )
        .expect("Transfer");

    let sft_balances: BalanceOfQueryResponse<TokenAmount> = sft_contract
        .balance_of()
        .invoke(&mut chain, &agent, &BalanceOfQueryParams {
            queries: vec![
                BalanceOfQuery {
                    address:  Address::Account(owner.address),
                    token_id: sft_token,
                },
                BalanceOfQuery {
                    address:  Address::Account(receiver.address),
                    token_id: sft_token,
                },
            ],
        })
        .expect("Balances")
        .parse_return_value()
        .expect("Parse Balances");
    assert_eq!(sft_balances, BalanceOfQueryResponse(vec![90.into(), 10.into()]));
}

fn create_default_admin(chain: &mut Chain) -> Account {
    let admin = Account::new_with_keys(
        ADMIN,
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(admin.clone());
    admin
}

fn setup_contract_with_admin(
    chain: &mut Chain,
    admin: &Account,
) -> (SecuritySftContract, Account, Verifier<IdentityRegistryContract>) {
    let (identity_registry, compliance_contract) =
        init_identity_contracts(chain, admin, vec!["IN".to_owned(), "US".to_owned()]);
    let ir_agent = Account::new_with_keys(
        IR_AGENT,
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(ir_agent.clone());
    identity_registry
        .add_agent()
        .update(chain, admin, &Address::Account(ir_agent.address))
        .expect("Add Agent identity");
    let verifier = Verifier {
        account:           ir_agent.clone(),
        identity_registry: identity_registry.clone(),
    };

    let contract =
        init_security_sft_contract(chain, admin, &identity_registry, &compliance_contract, vec![]);

    let agent = Account::new_with_keys(
        AGENT,
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(agent.clone());
    contract
        .add_agent()
        .update(chain, admin, &Address::Account(agent.address))
        .expect("Adding Nft Agent");

    (contract, agent, verifier)
}
