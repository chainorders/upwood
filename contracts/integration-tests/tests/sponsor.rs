pub mod utils;
use concordium_cis2::{AdditionalData, Receiver, TokenAmountU32, Transfer, TransferParams};
use concordium_rwa_sponsor::types::{PermitMessage, PermitParam};
use concordium_smart_contract_testing::{ed25519::PublicKey, *};
use concordium_std::{ExpectReport, Serial, ACCOUNT_ADDRESS_SIZE};
use utils::{
    cis2_test_contract::{ICis2Contract, ICis2ContractExt},
    common::{init_identity_contracts, init_security_token_contracts},
    consts::{DEFAULT_ACC_BALANCE, SPONSOR_MODULE},
    security_nft::ISecurityNftContractExt,
    security_sft::sft_mint,
    sponsor::{ISponsorContract, ISponsorModule, SponsorContract, SponsorModule},
    test_contract_client::{ITestContract, ITestModule},
    verifier::Verifier,
};

#[test]
fn sponsored_nft_transfer() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let admin = Account::new_with_keys(
        AccountAddress([0; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let token_receiver = Account::new_with_keys(
        AccountAddress([1; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let rng = &mut rand_core::OsRng;
    let token_owner_key_pairs = AccountKeys::singleton(rng);
    let token_owner = Account::new_with_keys(
        AccountAddress([2; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        (&token_owner_key_pairs).into(),
    );
    let sponsor_account = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    vec![admin.clone(), token_receiver.clone(), sponsor_account.clone(), token_owner.clone()]
        .iter()
        .for_each(|a| {
            chain.create_account(a.clone());
        });
    let (ir_contract, compliance_contract) =
        init_identity_contracts(&mut chain, &admin, compliant_nationalities.to_vec());
    let verifier = Verifier {
        account:           admin.clone(),
        identity_registry: ir_contract.clone(),
    };
    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(token_receiver.address), compliant_nationalities[0].clone()),
            (Address::Account(token_owner.address), compliant_nationalities[1].clone()),
        ])
        .expect("Add Account identities");

    let sponsor_module = SponsorModule {
        module_path: SPONSOR_MODULE.to_owned(),
    };
    sponsor_module.deploy(&mut chain, &admin).expect("deploy sponsor module");
    let sponsor = sponsor_module
        .rwa_sponsor()
        .init_without_params(&mut chain, &sponsor_account)
        .map(|s| SponsorContract(s.contract_address))
        .expect("Sponsor: init");

    let (token_contract, _) = init_security_token_contracts(
        &mut chain,
        &admin,
        &ir_contract,
        &compliance_contract,
        vec![sponsor.contract_address()],
    )
    .expect("Init security token contracts");

    let token_id = token_contract
        .mint_single_update(
            &mut chain,
            &admin,
            Receiver::Account(token_owner.address),
            concordium_rwa_security_nft::types::ContractMetadataUrl {
                url:  "ipfs:url1".to_string(),
                hash: None,
            },
        )
        .expect("nft: mint token");

    // Payload for NFT contract
    let payload: concordium_rwa_security_nft::types::TransferParams =
        TransferParams(vec![Transfer {
            from: token_owner.address.into(),
            amount: 1.into(),
            to: token_receiver.address.into(),
            token_id,
            data: AdditionalData::empty(),
        }]);
    // Sponsor Contract Params
    let permit_param: PermitParam = {
        let nonce = sponsor
            .nonce()
            .invoke(&mut chain, &token_owner, &concordium_rwa_sponsor::utils::NonceParam {
                account: token_owner.address,
            })
            .map(|res| sponsor.nonce().parse_return_value(&res).expect("Parsing nonce"))
            .expect("sponsor: nonce");
        let permit_message = PermitMessage {
            contract_address: token_contract.contract_address(),
            entry_point: token_contract.transfer().entrypoint_name,
            nonce,
            timestamp: chain
                .block_time()
                .checked_add(Duration::from_seconds(1))
                .expect_report("Block time"),
            payload: to_bytes(&payload),
        };
        let bytes_to_sign = sponsor
            .bytes_to_sign()
            .invoke(&mut chain, &token_owner, &permit_message)
            .map(|res| {
                sponsor.bytes_to_sign().parse_return_value(&res).expect("Parsing bytes to sign")
            })
            .expect("sponsor: bytes to sign");

        PermitParam {
            signature: token_owner_key_pairs.sign_message(&bytes_to_sign),
            signer:    token_owner.address,
            message:   permit_message,
        }
    };

    sponsor.permit().update(&mut chain, &sponsor_account, &permit_param).expect("sponsor: permit");

    let balance_of_token_owner = token_contract
        .balance_of_single_invoke(&mut chain, &token_owner, token_id, token_owner.address.into())
        .expect("nft: balance of token owner");
    assert_eq!(balance_of_token_owner, 0.into());

    let balance_of_token_receiver = token_contract
        .balance_of_single_invoke(
            &mut chain,
            &token_receiver,
            token_id,
            token_receiver.address.into(),
        )
        .expect("nft: balance of token owner");
    assert_eq!(balance_of_token_receiver, 1.into());
}

#[test]
pub fn sponsored_sft_transfer() {
    let compliant_nationalities = ["IN".to_owned(), "US".to_owned()];
    let mut chain = Chain::new();
    let admin = Account::new_with_keys(
        AccountAddress([0; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let token_receiver = Account::new_with_keys(
        AccountAddress([1; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    let rng = &mut rand_core::OsRng;
    let token_owner_key_pairs = AccountKeys::singleton(rng);
    let token_owner = Account::new_with_keys(
        AccountAddress([2; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        (&token_owner_key_pairs).into(),
    );
    let sponsor_account = Account::new_with_keys(
        AccountAddress([3; ACCOUNT_ADDRESS_SIZE]),
        AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
        AccountAccessStructure::singleton(PublicKey::default()),
    );
    vec![admin.clone(), token_receiver.clone(), sponsor_account.clone(), token_owner.clone()]
        .iter()
        .for_each(|a| {
            chain.create_account(a.clone());
        });
    let (ir_contract, compliance_contract) =
        init_identity_contracts(&mut chain, &admin, compliant_nationalities.to_vec());
    let verifier = Verifier {
        account:           admin.clone(),
        identity_registry: ir_contract.clone(),
    };
    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(token_receiver.address), compliant_nationalities[0].clone()),
            (Address::Account(token_owner.address), compliant_nationalities[1].clone()),
        ])
        .expect("Add Account identities");

    let sponsor_module = SponsorModule {
        module_path: SPONSOR_MODULE.to_owned(),
    };
    sponsor_module.deploy(&mut chain, &admin).expect("deploy sponsor module");
    let sponsor = sponsor_module
        .rwa_sponsor()
        .init_without_params(&mut chain, &sponsor_account)
        .map(|s| SponsorContract(s.contract_address))
        .expect("Sponsor: init");
    let (nft_contract, token_contract) = init_security_token_contracts(
        &mut chain,
        &admin,
        &ir_contract,
        &compliance_contract,
        vec![sponsor.contract_address()],
    )
    .expect("Init security token contracts");
    verifier
        .register_nationalities(&mut chain, vec![(
            Address::Contract(token_contract.contract_address()),
            "US".to_string(),
        )])
        .expect("Register nationalities");
    let token_id = sft_mint(
        &mut chain,
        &admin,
        &nft_contract,
        &token_contract,
        &token_owner,
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
    let balance_of_token_owner = token_contract
        .balance_of_single_invoke(&mut chain, &admin, token_id, token_owner.address.into())
        .expect("sft: balance of token owner");
    assert_eq!(balance_of_token_owner, 1000.into());

    let balance_of_token_receiver = token_contract
        .balance_of_single_invoke(&mut chain, &admin, token_id, token_receiver.address.into())
        .expect("sft: balance of token receiver");
    assert_eq!(balance_of_token_receiver, 0.into());

    // Payload for SFT contract
    let payload = {
        let payload: concordium_rwa_security_sft::types::ContractTransferParams =
            TransferParams(vec![Transfer {
                from: token_owner.address.into(),
                amount: TokenAmountU32(100),
                to: token_receiver.address.into(),
                token_id,
                data: AdditionalData::empty(),
            }]);
        let mut payload_bytes = Vec::new();
        payload.serial(&mut payload_bytes).expect("Serializing payload");
        payload_bytes
    };

    // Sponsor Contract Params
    let permit_param: PermitParam = {
        let nonce = sponsor
            .nonce()
            .invoke(&mut chain, &token_owner, &concordium_rwa_sponsor::utils::NonceParam {
                account: token_owner.address,
            })
            .map(|res| sponsor.nonce().parse_return_value(&res).expect("Parsing nonce"))
            .expect("sponsor: nonce");
        let permit_message = PermitMessage {
            contract_address: token_contract.contract_address(),
            entry_point: token_contract.transfer().entrypoint_name,
            nonce,
            timestamp: chain
                .block_time()
                .checked_add(Duration::from_seconds(1))
                .expect_report("Block time"),
            payload,
        };
        let bytes_to_sign = sponsor
            .bytes_to_sign()
            .invoke(&mut chain, &token_owner, &permit_message)
            .map(|res| {
                sponsor.bytes_to_sign().parse_return_value(&res).expect("Parsing bytes to sign")
            })
            .expect("sponsor: bytes to sign");

        PermitParam {
            signature: token_owner_key_pairs.sign_message(&bytes_to_sign),
            signer:    token_owner.address,
            message:   permit_message,
        }
    };

    sponsor.permit().update(&mut chain, &sponsor_account, &permit_param).expect("sponsor: permit");

    let balance_of_token_owner = token_contract
        .balance_of_single_invoke(&mut chain, &admin, token_id, token_owner.address.into())
        .expect("sft: balance of token owner");
    assert_eq!(balance_of_token_owner, 900.into());

    let balance_of_token_receiver = token_contract
        .balance_of_single_invoke(&mut chain, &admin, token_id, token_receiver.address.into())
        .expect("sft: balance of token receiver");
    assert_eq!(balance_of_token_receiver, 100.into());
}
