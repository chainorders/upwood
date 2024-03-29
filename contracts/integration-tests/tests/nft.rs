pub mod utils;

use concordium_cis2::{
    AdditionalData, BalanceOfQuery, BalanceOfQueryParams, BalanceOfQueryResponse, OperatorOfQuery,
    OperatorOfQueryParams, OperatorOfQueryResponse, OperatorUpdate, Receiver, TokenIdU8,
    TokenMetadataQueryParams, TokenMetadataQueryResponse, TransferParams, UpdateOperator,
    UpdateOperatorParams,
};
use concordium_rwa_security_nft::{error::Error, types::*};
use concordium_smart_contract_testing::{ed25519::PublicKey, *};
use concordium_std::{fail, AccountAddress, MetadataUrl, Reject, ACCOUNT_ADDRESS_SIZE};
use utils::{
    cis2_test_contract::ICis2Contract,
    common::{init_identity_contracts, init_security_nft_contract},
    consts::DEFAULT_ACC_BALANCE,
    identity_registry::{IIdentityRegistryContract, IdentityRegistryContract},
    security_nft::{ISecurityNftContract, ISecurityNftContractExt, SecurityNftContract},
    verifier::Verifier,
};

const ADMIN: AccountAddress = AccountAddress([0; ACCOUNT_ADDRESS_SIZE]);

#[test]
fn init() {
    let mut chain = Chain::new();
    let admin = &create_default_admin(&mut chain);
    let (identity_registry, compliance_contract) =
        init_identity_contracts(&mut chain, admin, vec!["IN".to_owned(), "US".to_owned()]);

    let nft_contract = init_security_nft_contract(
        &mut chain,
        admin,
        &identity_registry,
        &compliance_contract,
        vec![],
    );

    // account initializing the contract is the default first agent of the contract
    let is_agent: bool = nft_contract
        .is_agent()
        .invoke(&mut chain, admin, &Address::Account(admin.address))
        .expect("Is Agent")
        .parse_return_value()
        .expect("Parse Is Agent");
    assert!(is_agent);
}

#[test]
fn remove_agent() {
    let mut chain = Chain::new();
    let admin = &create_default_admin(&mut chain);
    let (nft_contract, agent, _) = setup_contract_with_admin(&mut chain, admin);
    let agent_address = Address::Account(agent.address);

    nft_contract.remove_agent().update(&mut chain, admin, &agent_address).expect("Remove Agent");

    let agents: Vec<Address> = nft_contract
        .agents()
        .invoke(&mut chain, &agent, &())
        .expect("Agents")
        .parse_return_value()
        .expect("Parse Agents");
    assert!(!agents.contains(&agent_address));

    let is_agent: bool = nft_contract
        .is_agent()
        .invoke(&mut chain, &agent, &Address::Account(agent.address))
        .expect("Is Agent")
        .parse_return_value()
        .expect("Parse Is Agent");
    assert!(!is_agent);
}

#[test]
fn check_is_agent() {
    let mut chain = Chain::new();
    let admin = &create_default_admin(&mut chain);
    let (nft_contract, agent, _) = setup_contract_with_admin(&mut chain, admin);

    let is_agent: bool = nft_contract
        .is_agent()
        .invoke(&mut chain, &agent, &Address::Account(agent.address))
        .expect("Is Agent")
        .parse_return_value()
        .expect("Parse Is Agent");
    assert!(is_agent);
}

#[test]
fn mint() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let other_account = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(other_account.clone());

    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");

    let balances: BalanceOfQueryResponse<TokenAmount> = nft_contract
        .balance_of()
        .invoke(&mut chain, &agent, &BalanceOfQueryParams {
            queries: vec![
                BalanceOfQuery {
                    address:  Address::Account(owner.address),
                    token_id: token,
                },
                BalanceOfQuery {
                    address:  Address::Account(other_account.address),
                    token_id: token,
                },
            ],
        })
        .expect("Balance of Owner")
        .parse_return_value()
        .expect("Parse Balance of Owner");

    assert_eq!(balances.0[0], 1.into());
    assert_eq!(balances.0[1], 0.into());
}

#[test]
fn mint_non_agent() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, _, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let non_agent = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone(), non_agent.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    let result = nft_contract
        .mint()
        .update(&mut chain, &non_agent, &concordium_rwa_security_nft::types::MintParams {
            owner:  Receiver::Account(owner.address),
            tokens: vec![concordium_rwa_security_nft::types::MintParam {
                metadata_url: ContractMetadataUrl {
                    url:  "ipfs:nft".to_string(),
                    hash: None,
                },
            }],
        })
        .expect_err("Minting without being an agent");

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
fn mint_to_unregistered_account() {
    let mut chain = Chain::new();
    let (nft_contract, agent, _) = setup_contract(&mut chain);
    let owner = AccountAddress([3; ACCOUNT_ADDRESS_SIZE]);
    let result = nft_contract
        .mint()
        .update(&mut chain, &agent, &MintParams {
            owner:  Receiver::Account(owner),
            tokens: vec![MintParam {
                metadata_url: ContractMetadataUrl {
                    url:  "ipfs:nft".to_string(),
                    hash: None,
                },
            }],
        })
        .expect_err("Minting to unregistered account");

    if let ContractInvokeError {
        kind: ContractInvokeErrorKind::ExecutionError {
            failure_kind,
        },
        ..
    } = result
    {
        assert_eq!(failure_kind, InvokeFailure::ContractReject {
            code: Reject::from(Error::UnVerifiedIdentity).error_code.into(),
            data: vec![],
        });
    } else {
        fail!("Expected ContractInvokeErrorKind::ExecutionError");
    }
}

#[test]
fn transfer() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let receiver = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone(), receiver.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(owner.address), compliant_nationalities[0].clone()),
            (Address::Account(receiver.address), compliant_nationalities[1].clone()),
        ])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .transfer()
        .update(
            &mut chain,
            &owner,
            &TransferParams(vec![concordium_cis2::Transfer {
                from:     Address::Account(owner.address),
                to:       Receiver::Account(receiver.address),
                token_id: token,
                amount:   1.into(),
                data:     AdditionalData::empty(),
            }]),
        )
        .expect("Transfer");
    let balances: BalanceOfQueryResponse<TokenAmount> = nft_contract
        .balance_of()
        .invoke(&mut chain, &agent, &BalanceOfQueryParams {
            queries: vec![
                BalanceOfQuery {
                    address:  Address::Account(owner.address),
                    token_id: token,
                },
                BalanceOfQuery {
                    address:  Address::Account(receiver.address),
                    token_id: token,
                },
            ],
        })
        .expect("Balances")
        .parse_return_value()
        .expect("Parse Balances");

    assert_eq!(balances.0[0], 0.into());
    assert_eq!(balances.0[1], 1.into());
}

#[test]
fn transfer_non_minted() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, _, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let receiver = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone(), receiver.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(owner.address), compliant_nationalities[0].clone()),
            (Address::Account(receiver.address), compliant_nationalities[1].clone()),
        ])
        .expect("Add Account identities");

    let result = nft_contract
        .transfer()
        .update(
            &mut chain,
            &owner,
            &TransferParams(vec![concordium_cis2::Transfer {
                from:     Address::Account(owner.address),
                to:       Receiver::Account(receiver.address),
                token_id: TokenIdU8(0),
                amount:   1.into(),
                data:     AdditionalData::empty(),
            }]),
        )
        .expect_err("Transfer non minted token");

    if let ContractInvokeError {
        kind: ContractInvokeErrorKind::ExecutionError {
            failure_kind,
        },
        ..
    } = result
    {
        assert_eq!(failure_kind, InvokeFailure::ContractReject {
            code: Reject::from(Error::InvalidTokenId).error_code.into(),
            data: vec![],
        });
    } else {
        fail!("Expected ContractInvokeErrorKind::ExecutionError");
    }
}

#[test]
fn transfer_twice() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let receiver = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone(), receiver.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(owner.address), compliant_nationalities[0].clone()),
            (Address::Account(receiver.address), compliant_nationalities[1].clone()),
        ])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .transfer()
        .update(
            &mut chain,
            &owner,
            &TransferParams(vec![concordium_cis2::Transfer {
                from:     Address::Account(owner.address),
                to:       Receiver::Account(receiver.address),
                token_id: token,
                amount:   1.into(),
                data:     AdditionalData::empty(),
            }]),
        )
        .expect("Transfer");
    let result = nft_contract
        .transfer()
        .update(
            &mut chain,
            &owner,
            &TransferParams(vec![concordium_cis2::Transfer {
                from:     Address::Account(owner.address),
                to:       Receiver::Account(receiver.address),
                token_id: token,
                amount:   1.into(),
                data:     AdditionalData::empty(),
            }]),
        )
        .expect_err("Transfer twice");

    if let ContractInvokeError {
        kind: ContractInvokeErrorKind::ExecutionError {
            failure_kind,
        },
        ..
    } = result
    {
        assert_eq!(failure_kind, InvokeFailure::ContractReject {
            code: Reject::from(Error::InsufficientFunds).error_code.into(),
            data: vec![],
        });
    } else {
        fail!("Expected ContractInvokeErrorKind::ExecutionError");
    }
}

#[test]
fn transfer_to_unregistered_account() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let receiver = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone(), receiver.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    let result = nft_contract
        .transfer()
        .update(
            &mut chain,
            &owner,
            &TransferParams(vec![concordium_cis2::Transfer {
                from:     Address::Account(owner.address),
                to:       Receiver::Account(receiver.address),
                token_id: token,
                amount:   1.into(),
                data:     AdditionalData::empty(),
            }]),
        )
        .expect_err("Transfer to unregistered account");

    if let ContractInvokeError {
        kind: ContractInvokeErrorKind::ExecutionError {
            failure_kind,
        },
        ..
    } = result
    {
        assert_eq!(failure_kind, InvokeFailure::ContractReject {
            code: Reject::from(Error::UnVerifiedIdentity).error_code.into(),
            data: vec![],
        });
    } else {
        fail!("Expected ContractInvokeErrorKind::ExecutionError");
    }
}

#[test]
fn transfer_paused() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let receiver = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone(), receiver.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(owner.address), compliant_nationalities[0].clone()),
            (Address::Account(receiver.address), compliant_nationalities[1].clone()),
        ])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .pause()
        .update(&mut chain, &agent, &PauseParams {
            tokens: vec![token],
        })
        .expect("Pausing token");

    let is_paused: IsPausedResponse = nft_contract
        .is_paused()
        .invoke(&mut chain, &agent, &PauseParams {
            tokens: vec![token],
        })
        .expect("Is Paused")
        .parse_return_value()
        .expect("Parse IsPausedResponse");
    assert_eq!(is_paused.tokens, vec![true]);

    let result = nft_contract
        .transfer()
        .update(
            &mut chain,
            &owner,
            &TransferParams(vec![concordium_cis2::Transfer {
                from:     Address::Account(owner.address),
                to:       Receiver::Account(receiver.address),
                token_id: token,
                amount:   1.into(),
                data:     AdditionalData::empty(),
            }]),
        )
        .expect_err("Transfer");

    if let ContractInvokeError {
        kind: ContractInvokeErrorKind::ExecutionError {
            failure_kind,
        },
        ..
    } = result
    {
        assert_eq!(failure_kind, InvokeFailure::ContractReject {
            code: Reject::from(Error::PausedToken).error_code.into(),
            data: vec![],
        });
    } else {
        fail!("Expected ContractInvokeErrorKind::ExecutionError");
    }
}

#[test]
fn transfer_un_paused() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let receiver = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone(), receiver.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(owner.address), compliant_nationalities[0].clone()),
            (Address::Account(receiver.address), compliant_nationalities[1].clone()),
        ])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .pause()
        .update(&mut chain, &agent, &PauseParams {
            tokens: vec![token],
        })
        .expect("Pausing token");
    nft_contract
        .un_pause()
        .update(&mut chain, &agent, &PauseParams {
            tokens: vec![token],
        })
        .expect("Un pausing token");
    nft_contract
        .transfer()
        .update(
            &mut chain,
            &owner,
            &TransferParams(vec![concordium_cis2::Transfer {
                from:     Address::Account(owner.address),
                to:       Receiver::Account(receiver.address),
                token_id: token,
                amount:   1.into(),
                data:     AdditionalData::empty(),
            }]),
        )
        .expect("Transfer");
    let balances: BalanceOfQueryResponse<TokenAmount> = nft_contract
        .balance_of()
        .invoke(&mut chain, &agent, &BalanceOfQueryParams {
            queries: vec![
                BalanceOfQuery {
                    address:  Address::Account(owner.address),
                    token_id: token,
                },
                BalanceOfQuery {
                    address:  Address::Account(receiver.address),
                    token_id: token,
                },
            ],
        })
        .expect("Balance of Owner")
        .parse_return_value()
        .expect("Parse Balance of Owner");

    assert_eq!(balances.0[0], 0.into());
    assert_eq!(balances.0[1], 1.into());
}

#[test]
fn transfer_via_operator() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let receiver = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let operator = Account::new_with_keys(
        AccountAddress([5; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone(), receiver.clone(), operator.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(owner.address), compliant_nationalities[0].clone()),
            (Address::Account(receiver.address), compliant_nationalities[1].clone()),
        ])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .update_operator()
        .update(
            &mut chain,
            &owner,
            &UpdateOperatorParams(vec![UpdateOperator {
                operator: Address::Account(operator.address),
                update:   OperatorUpdate::Add,
            }]),
        )
        .expect("Update Operator");
    nft_contract
        .transfer()
        .update(
            &mut chain,
            &operator,
            &TransferParams(vec![concordium_cis2::Transfer {
                from:     Address::Account(owner.address),
                to:       Receiver::Account(receiver.address),
                token_id: token,
                amount:   1.into(),
                data:     AdditionalData::empty(),
            }]),
        )
        .expect("Transfer");
    let balances: BalanceOfQueryResponse<TokenAmount> = nft_contract
        .balance_of()
        .invoke(&mut chain, &agent, &BalanceOfQueryParams {
            queries: vec![
                BalanceOfQuery {
                    address:  Address::Account(owner.address),
                    token_id: token,
                },
                BalanceOfQuery {
                    address:  Address::Account(receiver.address),
                    token_id: token,
                },
            ],
        })
        .expect("Balance of Owner")
        .parse_return_value()
        .expect("Parse Balance of Owner");

    assert_eq!(balances.0[0], 0.into());
    assert_eq!(balances.0[1], 1.into());
}

#[test]
fn transfer_frozen() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let receiver = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone(), receiver.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(owner.address), compliant_nationalities[0].clone()),
            (Address::Account(receiver.address), compliant_nationalities[1].clone()),
        ])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .freeze()
        .update(&mut chain, &agent, &FreezeParams {
            tokens: vec![FreezeParam {
                token_id:     token,
                token_amount: 1.into(),
            }],
            owner:  Address::Account(owner.address),
        })
        .expect("Freeze token");

    let result = nft_contract
        .transfer()
        .update(
            &mut chain,
            &owner,
            &TransferParams(vec![concordium_cis2::Transfer {
                from:     Address::Account(owner.address),
                to:       Receiver::Account(receiver.address),
                token_id: token,
                amount:   1.into(),
                data:     AdditionalData::empty(),
            }]),
        )
        .expect_err("Transfer");

    if let ContractInvokeError {
        kind: ContractInvokeErrorKind::ExecutionError {
            failure_kind,
        },
        ..
    } = result
    {
        assert_eq!(failure_kind, InvokeFailure::ContractReject {
            code: Reject::from(Error::InsufficientFunds).error_code.into(),
            data: vec![],
        });
    } else {
        fail!("Expected ContractInvokeErrorKind::ExecutionError");
    }
}

#[test]
fn forced_transfer() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let receiver = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone(), receiver.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(owner.address), compliant_nationalities[0].clone()),
            (Address::Account(receiver.address), compliant_nationalities[1].clone()),
        ])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");

    nft_contract
        .forced_transfer()
        .update(
            &mut chain,
            &agent,
            &TransferParams(vec![concordium_cis2::Transfer {
                from:     Address::Account(owner.address),
                to:       Receiver::Account(receiver.address),
                token_id: token,
                amount:   1.into(),
                data:     AdditionalData::empty(),
            }]),
        )
        .expect("Forced Transfer");

    let balances: BalanceOfQueryResponse<TokenAmount> = nft_contract
        .balance_of()
        .invoke(&mut chain, &agent, &BalanceOfQueryParams {
            queries: vec![
                BalanceOfQuery {
                    address:  Address::Account(owner.address),
                    token_id: token,
                },
                BalanceOfQuery {
                    address:  Address::Account(receiver.address),
                    token_id: token,
                },
            ],
        })
        .expect("Balances")
        .parse_return_value()
        .expect("Parse Balances");

    assert_eq!(balances.0[0], 0.into());
    assert_eq!(balances.0[1], 1.into());
}

#[test]
fn forced_transfer_frozen() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let receiver = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone(), receiver.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(owner.address), compliant_nationalities[0].clone()),
            (Address::Account(receiver.address), compliant_nationalities[1].clone()),
        ])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .freeze()
        .update(&mut chain, &agent, &FreezeParams {
            tokens: vec![FreezeParam {
                token_id:     token,
                token_amount: 1.into(),
            }],
            owner:  Address::Account(owner.address),
        })
        .expect("Freeze token");

    nft_contract
        .forced_transfer()
        .update(
            &mut chain,
            &agent,
            &TransferParams(vec![concordium_cis2::Transfer {
                from:     Address::Account(owner.address),
                to:       Receiver::Account(receiver.address),
                token_id: token,
                amount:   1.into(),
                data:     AdditionalData::empty(),
            }]),
        )
        .expect("Forced Transfer");

    let balances: BalanceOfQueryResponse<TokenAmount> = nft_contract
        .balance_of()
        .invoke(&mut chain, &agent, &BalanceOfQueryParams {
            queries: vec![
                BalanceOfQuery {
                    address:  Address::Account(owner.address),
                    token_id: token,
                },
                BalanceOfQuery {
                    address:  Address::Account(receiver.address),
                    token_id: token,
                },
            ],
        })
        .expect("Balances")
        .parse_return_value()
        .expect("Parse Balances");

    assert_eq!(balances.0[0], 0.into());
    assert_eq!(balances.0[1], 1.into());
}

#[test]
fn forced_transfer_non_agent() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let receiver = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone(), receiver.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(owner.address), compliant_nationalities[0].clone()),
            (Address::Account(receiver.address), compliant_nationalities[1].clone()),
        ])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");

    let result = nft_contract
        .forced_transfer()
        .update(
            &mut chain,
            &owner,
            &TransferParams(vec![concordium_cis2::Transfer {
                from:     Address::Account(owner.address),
                to:       Receiver::Account(receiver.address),
                token_id: token,
                amount:   1.into(),
                data:     AdditionalData::empty(),
            }]),
        )
        .expect_err("Forced Transfer");

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
fn transfer_unfrozen() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let receiver = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone(), receiver.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(owner.address), compliant_nationalities[0].clone()),
            (Address::Account(receiver.address), compliant_nationalities[1].clone()),
        ])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .freeze()
        .update(&mut chain, &agent, &FreezeParams {
            tokens: vec![FreezeParam {
                token_id:     token,
                token_amount: 1.into(),
            }],
            owner:  Address::Account(owner.address),
        })
        .expect("Freeze token");
    nft_contract
        .un_freeze()
        .update(&mut chain, &agent, &FreezeParams {
            tokens: vec![FreezeParam {
                token_id:     token,
                token_amount: 1.into(),
            }],
            owner:  Address::Account(owner.address),
        })
        .expect("Un freeze token");
    nft_contract
        .transfer()
        .update(
            &mut chain,
            &owner,
            &TransferParams(vec![concordium_cis2::Transfer {
                from:     Address::Account(owner.address),
                to:       Receiver::Account(receiver.address),
                token_id: token,
                amount:   1.into(),
                data:     AdditionalData::empty(),
            }]),
        )
        .expect("Transfer");
    let balances: BalanceOfQueryResponse<TokenAmount> = nft_contract
        .balance_of()
        .invoke(&mut chain, &agent, &BalanceOfQueryParams {
            queries: vec![
                BalanceOfQuery {
                    address:  Address::Account(owner.address),
                    token_id: token,
                },
                BalanceOfQuery {
                    address:  Address::Account(receiver.address),
                    token_id: token,
                },
            ],
        })
        .expect("Balance of Owner")
        .parse_return_value()
        .expect("Parse Balance of Owner");

    assert_eq!(balances.0[0], 0.into());
    assert_eq!(balances.0[1], 1.into());
}

#[test]
fn burn() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .burn()
        .update(&mut chain, &owner, &BurnParams {
            0: vec![Burn {
                amount:   1.into(),
                token_id: token,
                owner:    Address::Account(owner.address),
            }],
        })
        .expect("Burn");
    let balances: BalanceOfQueryResponse<TokenAmount> = nft_contract
        .balance_of()
        .invoke(&mut chain, &agent, &BalanceOfQueryParams {
            queries: vec![BalanceOfQuery {
                address:  Address::Account(owner.address),
                token_id: token,
            }],
        })
        .expect("Balance of Owner")
        .parse_return_value()
        .expect("Parse Balance of Owner");

    assert_eq!(balances.0[0], 0.into());
}

#[test]
fn burn_via_operator() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let operator = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone(), operator.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .update_operator()
        .update(
            &mut chain,
            &owner,
            &UpdateOperatorParams(vec![UpdateOperator {
                operator: Address::Account(operator.address),
                update:   OperatorUpdate::Add,
            }]),
        )
        .expect("Update Operator");
    nft_contract
        .burn()
        .update(&mut chain, &operator, &BurnParams {
            0: vec![Burn {
                amount:   1.into(),
                token_id: token,
                owner:    Address::Account(owner.address),
            }],
        })
        .expect("Burn");
    let balances: BalanceOfQueryResponse<TokenAmount> = nft_contract
        .balance_of()
        .invoke(&mut chain, &agent, &BalanceOfQueryParams {
            queries: vec![BalanceOfQuery {
                address:  Address::Account(owner.address),
                token_id: token,
            }],
        })
        .expect("Balance of Owner")
        .parse_return_value()
        .expect("Parse Balance of Owner");

    assert_eq!(balances.0[0], 0.into());
}

#[test]
fn burn_twice() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .burn()
        .update(&mut chain, &owner, &BurnParams {
            0: vec![Burn {
                amount:   1.into(),
                token_id: token,
                owner:    Address::Account(owner.address),
            }],
        })
        .expect("Burn");
    let result = nft_contract
        .burn()
        .update(&mut chain, &owner, &BurnParams {
            0: vec![Burn {
                amount:   1.into(),
                token_id: token,
                owner:    Address::Account(owner.address),
            }],
        })
        .expect_err("Burn twice");

    if let ContractInvokeError {
        kind: ContractInvokeErrorKind::ExecutionError {
            failure_kind,
        },
        ..
    } = result
    {
        assert_eq!(failure_kind, InvokeFailure::ContractReject {
            code: Reject::from(Error::InsufficientFunds).error_code.into(),
            data: vec![],
        });
    } else {
        fail!("Expected ContractInvokeErrorKind::ExecutionError");
    }
}

#[test]
fn burn_non_minted() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, _, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    let result = nft_contract
        .burn()
        .update(&mut chain, &owner, &BurnParams {
            0: vec![Burn {
                amount:   1.into(),
                token_id: TokenIdU8(0),
                owner:    Address::Account(owner.address),
            }],
        })
        .expect_err("Burn non minted token");

    if let ContractInvokeError {
        kind: ContractInvokeErrorKind::ExecutionError {
            failure_kind,
        },
        ..
    } = result
    {
        assert_eq!(failure_kind, InvokeFailure::ContractReject {
            code: Reject::from(Error::InvalidTokenId).error_code.into(),
            data: vec![],
        });
    } else {
        fail!("Expected ContractInvokeErrorKind::ExecutionError");
    }
}

#[test]
fn burn_transferred() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let receiver = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone(), receiver.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(owner.address), compliant_nationalities[0].clone()),
            (Address::Account(receiver.address), compliant_nationalities[1].clone()),
        ])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .transfer()
        .update(
            &mut chain,
            &owner,
            &TransferParams(vec![concordium_cis2::Transfer {
                from:     Address::Account(owner.address),
                to:       Receiver::Account(receiver.address),
                token_id: token,
                amount:   1.into(),
                data:     AdditionalData::empty(),
            }]),
        )
        .expect("Transfer");
    let result = nft_contract
        .burn()
        .update(&mut chain, &owner, &BurnParams {
            0: vec![Burn {
                amount:   1.into(),
                token_id: token,
                owner:    Address::Account(owner.address),
            }],
        })
        .expect_err("Burn transferred token");

    if let ContractInvokeError {
        kind: ContractInvokeErrorKind::ExecutionError {
            failure_kind,
        },
        ..
    } = result
    {
        assert_eq!(failure_kind, InvokeFailure::ContractReject {
            code: Reject::from(Error::InsufficientFunds).error_code.into(),
            data: vec![],
        });
    } else {
        fail!("Expected ContractInvokeErrorKind::ExecutionError");
    }
}

#[test]
fn burn_paused() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .pause()
        .update(&mut chain, &agent, &PauseParams {
            tokens: vec![token],
        })
        .expect("Pausing token");

    let result = nft_contract
        .burn()
        .update(&mut chain, &owner, &BurnParams {
            0: vec![Burn {
                amount:   1.into(),
                token_id: token,
                owner:    Address::Account(owner.address),
            }],
        })
        .expect_err("Burn");

    if let ContractInvokeError {
        kind: ContractInvokeErrorKind::ExecutionError {
            failure_kind,
        },
        ..
    } = result
    {
        assert_eq!(failure_kind, InvokeFailure::ContractReject {
            code: Reject::from(Error::PausedToken).error_code.into(),
            data: vec![],
        });
    } else {
        fail!("Expected ContractInvokeErrorKind::ExecutionError");
    }
}

#[test]
fn burn_un_paused() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .pause()
        .update(&mut chain, &agent, &PauseParams {
            tokens: vec![token],
        })
        .expect("Pausing token");
    nft_contract
        .un_pause()
        .update(&mut chain, &agent, &PauseParams {
            tokens: vec![token],
        })
        .expect("Un pausing token");
    nft_contract
        .burn()
        .update(&mut chain, &owner, &BurnParams {
            0: vec![Burn {
                amount:   1.into(),
                token_id: token,
                owner:    Address::Account(owner.address),
            }],
        })
        .expect("Burn");
    let balances: BalanceOfQueryResponse<TokenAmount> = nft_contract
        .balance_of()
        .invoke(&mut chain, &agent, &BalanceOfQueryParams {
            queries: vec![BalanceOfQuery {
                address:  Address::Account(owner.address),
                token_id: token,
            }],
        })
        .expect("Balance of Owner")
        .parse_return_value()
        .expect("Parse Balance of Owner");

    assert_eq!(balances.0[0], 0.into());
}

#[test]
fn burn_frozen() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .freeze()
        .update(&mut chain, &agent, &FreezeParams {
            tokens: vec![FreezeParam {
                token_id:     token,
                token_amount: 1.into(),
            }],
            owner:  Address::Account(owner.address),
        })
        .expect("Freeze token");

    let result = nft_contract
        .burn()
        .update(&mut chain, &owner, &BurnParams {
            0: vec![Burn {
                amount:   1.into(),
                token_id: token,
                owner:    Address::Account(owner.address),
            }],
        })
        .expect_err("Burn");

    if let ContractInvokeError {
        kind: ContractInvokeErrorKind::ExecutionError {
            failure_kind,
        },
        ..
    } = result
    {
        assert_eq!(failure_kind, InvokeFailure::ContractReject {
            code: Reject::from(Error::InsufficientFunds).error_code.into(),
            data: vec![],
        });
    } else {
        fail!("Expected ContractInvokeErrorKind::ExecutionError");
    }
}

#[test]
fn burn_unfrozen() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .freeze()
        .update(&mut chain, &agent, &FreezeParams {
            tokens: vec![FreezeParam {
                token_id:     token,
                token_amount: 1.into(),
            }],
            owner:  Address::Account(owner.address),
        })
        .expect("Freeze token");
    nft_contract
        .un_freeze()
        .update(&mut chain, &agent, &FreezeParams {
            tokens: vec![FreezeParam {
                token_id:     token,
                token_amount: 1.into(),
            }],
            owner:  Address::Account(owner.address),
        })
        .expect("Un freeze token");
    nft_contract
        .burn()
        .update(&mut chain, &owner, &BurnParams {
            0: vec![Burn {
                amount:   1.into(),
                token_id: token,
                owner:    Address::Account(owner.address),
            }],
        })
        .expect("Burn");
    let balances: BalanceOfQueryResponse<TokenAmount> = nft_contract
        .balance_of()
        .invoke(&mut chain, &agent, &BalanceOfQueryParams {
            queries: vec![BalanceOfQuery {
                address:  Address::Account(owner.address),
                token_id: token,
            }],
        })
        .expect("Balance of Owner")
        .parse_return_value()
        .expect("Parse Balance of Owner");

    assert_eq!(balances.0[0], 0.into());
}

#[test]
fn pause() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .pause()
        .update(&mut chain, &agent, &PauseParams {
            tokens: vec![token],
        })
        .expect("Pausing token");
    let is_paused: IsPausedResponse = nft_contract
        .is_paused()
        .invoke(&mut chain, &agent, &PauseParams {
            tokens: vec![token],
        })
        .expect("Is Paused")
        .parse_return_value()
        .expect("Parse IsPausedResponse");
    assert_eq!(is_paused.tokens, vec![true]);
}

#[test]
fn pause_non_minted() {
    let mut chain = Chain::new();
    let (nft_contract, agent, _) = setup_contract(&mut chain);
    let result = nft_contract
        .pause()
        .update(&mut chain, &agent, &PauseParams {
            tokens: vec![TokenIdU8(0)],
        })
        .expect_err("Pause non minted token");

    if let ContractInvokeError {
        kind: ContractInvokeErrorKind::ExecutionError {
            failure_kind,
        },
        ..
    } = result
    {
        assert_eq!(failure_kind, InvokeFailure::ContractReject {
            code: Reject::from(Error::InvalidTokenId).error_code.into(),
            data: vec![],
        });
    } else {
        fail!("Expected ContractInvokeErrorKind::ExecutionError");
    }
}

#[test]
fn un_pause() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .pause()
        .update(&mut chain, &agent, &PauseParams {
            tokens: vec![token],
        })
        .expect("Pausing token");
    nft_contract
        .un_pause()
        .update(&mut chain, &agent, &PauseParams {
            tokens: vec![token],
        })
        .expect("Un pausing token");
    let is_paused: IsPausedResponse = nft_contract
        .is_paused()
        .invoke(&mut chain, &agent, &PauseParams {
            tokens: vec![token],
        })
        .expect("Is Paused")
        .parse_return_value()
        .expect("Parse IsPausedResponse");
    assert_eq!(is_paused.tokens, vec![false]);
}

#[test]
fn pause_burned() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .burn()
        .update(&mut chain, &owner, &BurnParams {
            0: vec![Burn {
                amount:   1.into(),
                token_id: token,
                owner:    Address::Account(owner.address),
            }],
        })
        .expect("Burn");

    nft_contract
        .pause()
        .update(&mut chain, &agent, &PauseParams {
            tokens: vec![token],
        })
        // Burning tokens only burns the token balances
        // The actual token being paused still exists
        .expect("Pause burned token");
}

#[test]
fn add_operator() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, _, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let operator = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone(), operator.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    nft_contract
        .update_operator()
        .update(
            &mut chain,
            &owner,
            &UpdateOperatorParams(vec![UpdateOperator {
                operator: Address::Account(operator.address),
                update:   OperatorUpdate::Add,
            }]),
        )
        .expect("Update Operator");
    let operators: OperatorOfQueryResponse = nft_contract
        .operator_of()
        .invoke(&mut chain, &owner, &OperatorOfQueryParams {
            queries: vec![OperatorOfQuery {
                address: Address::Account(operator.address),
                owner:   Address::Account(owner.address),
            }],
        })
        .expect("Operators")
        .parse_return_value()
        .expect("Parse OperatorsResponse");

    assert_eq!(operators, OperatorOfQueryResponse(vec![true]));
}

#[test]
fn remove_operator() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, _, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let operator = Account::new_with_keys(
        AccountAddress([4; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone(), operator.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    nft_contract
        .update_operator()
        .update(
            &mut chain,
            &owner,
            &UpdateOperatorParams(vec![UpdateOperator {
                operator: Address::Account(operator.address),
                update:   OperatorUpdate::Add,
            }]),
        )
        .expect("Update Operator");
    nft_contract
        .update_operator()
        .update(
            &mut chain,
            &owner,
            &UpdateOperatorParams(vec![UpdateOperator {
                operator: Address::Account(operator.address),
                update:   OperatorUpdate::Remove,
            }]),
        )
        .expect("Update Operator");
    let operators: OperatorOfQueryResponse = nft_contract
        .operator_of()
        .invoke(&mut chain, &owner, &OperatorOfQueryParams {
            queries: vec![OperatorOfQuery {
                address: Address::Account(operator.address),
                owner:   Address::Account(owner.address),
            }],
        })
        .expect("Operators")
        .parse_return_value()
        .expect("Parse OperatorsResponse");

    assert_eq!(operators, OperatorOfQueryResponse(vec![false]));
}

#[test]
fn token_metadata() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    let metadata: TokenMetadataQueryResponse = nft_contract
        .token_metadata()
        .invoke(&mut chain, &agent, &TokenMetadataQueryParams {
            queries: vec![token],
        })
        .expect("Token Metadata")
        .parse_return_value()
        .expect("Parse TokenMetadataResponse");

    assert_eq!(metadata.0[0], MetadataUrl {
        url:  "ipfs:nft".to_string(),
        hash: None,
    });
}

#[test]
fn token_metadata_non_minted() {
    let mut chain = Chain::new();
    let (nft_contract, agent, _) = setup_contract(&mut chain);
    let result = nft_contract
        .token_metadata()
        .invoke(&mut chain, &agent, &TokenMetadataQueryParams {
            queries: vec![TokenIdU8(0)],
        })
        .expect_err("Token Metadata non minted token");

    if let ContractInvokeError {
        kind: ContractInvokeErrorKind::ExecutionError {
            failure_kind,
        },
        ..
    } = result
    {
        assert_eq!(failure_kind, InvokeFailure::ContractReject {
            code: Reject::from(Error::InvalidTokenId).error_code.into(),
            data: vec![],
        });
    } else {
        fail!("Expected ContractInvokeErrorKind::ExecutionError");
    }
}

#[test]
fn token_metadata_burned() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");

    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .burn()
        .update(&mut chain, &owner, &BurnParams {
            0: vec![Burn {
                amount:   1.into(),
                token_id: token,
                owner:    Address::Account(owner.address),
            }],
        })
        .expect("Burn");

    let metadata: TokenMetadataQueryResponse = nft_contract
        .token_metadata()
        .invoke(&mut chain, &agent, &TokenMetadataQueryParams {
            queries: vec![token],
        })
        .expect("Token Metadata")
        .parse_return_value()
        .expect("Parse TokenMetadataResponse");

    assert_eq!(metadata.0[0], MetadataUrl {
        url:  "ipfs:nft".to_string(),
        hash: None,
    });
}

#[test]
fn freeze() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");
    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .freeze()
        .update(&mut chain, &agent, &FreezeParams {
            tokens: vec![FreezeParam {
                token_id:     token,
                token_amount: 1.into(),
            }],
            owner:  Address::Account(owner.address),
        })
        .expect("Freeze token");

    let frozen_response: FrozenResponse = nft_contract
        .balance_of_frozen()
        .invoke(&mut chain, &agent, &FrozenParams {
            owner:  Address::Account(owner.address),
            tokens: vec![token],
        })
        .expect("Is Frozen")
        .parse_return_value()
        .expect("Parse FrozenResponse");
    assert_eq!(frozen_response, FrozenResponse {
        tokens: vec![1.into()],
    });

    let un_frozen_response: FrozenResponse = nft_contract
        .balance_of_un_frozen()
        .invoke(&mut chain, &agent, &FrozenParams {
            owner:  Address::Account(owner.address),
            tokens: vec![token],
        })
        .expect("Is UnFrozen")
        .parse_return_value()
        .expect("Parse UnFrozenResponse");
    assert_eq!(un_frozen_response, FrozenResponse {
        tokens: vec![0.into()],
    });
}

#[test]
fn un_freeze() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let (nft_contract, agent, verifier) = setup_contract(&mut chain);
    let owner = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    [owner.clone()].iter().for_each(|a| {
        chain.create_account(a.clone());
    });
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Account(owner.address),
            compliant_nationalities[0].clone(),
        )])
        .expect("Add Account identities");
    let token = nft_contract
        .mint_single_update(
            &mut chain,
            &agent,
            Receiver::Account(owner.address),
            ContractMetadataUrl {
                url:  "ipfs:nft".to_string(),
                hash: None,
            },
        )
        .expect("Minting to registered account");
    nft_contract
        .freeze()
        .update(&mut chain, &agent, &FreezeParams {
            tokens: vec![FreezeParam {
                token_id:     token,
                token_amount: 1.into(),
            }],
            owner:  Address::Account(owner.address),
        })
        .expect("Freeze token");
    nft_contract
        .un_freeze()
        .update(&mut chain, &agent, &FreezeParams {
            tokens: vec![FreezeParam {
                token_id:     token,
                token_amount: 1.into(),
            }],
            owner:  Address::Account(owner.address),
        })
        .expect("UnFreeze token");

    let frozen_response: FrozenResponse = nft_contract
        .balance_of_frozen()
        .invoke(&mut chain, &agent, &FrozenParams {
            owner:  Address::Account(owner.address),
            tokens: vec![token],
        })
        .expect("Is Frozen")
        .parse_return_value()
        .expect("Parse FrozenResponse");
    assert_eq!(frozen_response, FrozenResponse {
        tokens: vec![0.into()],
    });

    let un_frozen_response: FrozenResponse = nft_contract
        .balance_of_un_frozen()
        .invoke(&mut chain, &agent, &FrozenParams {
            owner:  Address::Account(owner.address),
            tokens: vec![token],
        })
        .expect("Is UnFrozen")
        .parse_return_value()
        .expect("Parse UnFrozenResponse");
    assert_eq!(un_frozen_response, FrozenResponse {
        tokens: vec![1.into()],
    });
}

fn setup_contract(
    chain: &mut Chain,
) -> (SecurityNftContract, Account, Verifier<IdentityRegistryContract>) {
    let admin = create_default_admin(chain);
    setup_contract_with_admin(chain, &admin)
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
) -> (SecurityNftContract, Account, Verifier<IdentityRegistryContract>) {
    let (identity_registry, compliance_contract) =
        init_identity_contracts(chain, admin, vec!["IN".to_owned(), "US".to_owned()]);
    let ir_agent = Account::new_with_keys(
        AccountAddress([1; ACCOUNT_ADDRESS_SIZE]),
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
        init_security_nft_contract(chain, admin, &identity_registry, &compliance_contract, vec![]);

    let nft_agent = Account::new_with_keys(
        AccountAddress([2; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    chain.create_account(nft_agent.clone());
    contract
        .add_agent()
        .update(chain, admin, &Address::Account(nft_agent.address))
        .expect("Adding Nft Agent");

    (contract, nft_agent, verifier)
}
