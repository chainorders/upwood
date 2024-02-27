mod utils;
use crate::utils::{
    chain::{init_identity_contracts, init_security_token_contracts},
    consts::DEFAULT_INVOKER,
    security_nft::nft_mint,
    security_sft::{sft_balance_of, sft_mint},
    sponsor::sponsor_deploy_and_init,
};
use concordium_cis2::{AdditionalData, Receiver, TokenAmountU32, Transfer, TransferParams};
use concordium_rwa_security_nft::types::TokenAmount;
use concordium_rwa_sponsor::types::{PermitMessage, PermitParam};
use concordium_smart_contract_testing::*;
use concordium_std::ExpectReport;
use concordium_std::Serial;
use utils::{
    chain::create_accounts,
    consts::DEFAULT_ACC_BALANCE,
    identity_registry::add_identities,
    security_nft::nft_balance_of,
    sponsor::{get_bytes_to_sign, get_nonce},
};

const ADMIN: AccountAddress = AccountAddress([0; 32]);
const TOKEN_OWNER: AccountAddress = AccountAddress([1; 32]);
const TOKEN_RECEIVER: AccountAddress = AccountAddress([2; 32]);
const SPONSOR_ACCOUNT: AccountAddress = AccountAddress([3; 32]);
const NATIONALITY_INDIA: &str = "IN";
const NATIONALITY_USA: &str = "US";

#[test]
fn sponsored_nft_transfer() {
    let mut chain = Chain::new();
    create_accounts(
        &mut chain,
        vec![DEFAULT_INVOKER, ADMIN, TOKEN_RECEIVER, SPONSOR_ACCOUNT],
        DEFAULT_ACC_BALANCE,
    );

    // Create an account for the token owner
    let token_owner_key_pairs = {
        let rng = &mut rand::thread_rng();
        let key_pairs = AccountKeys::singleton(rng);
        chain.create_account(Account::new_with_keys(
            TOKEN_OWNER,
            AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
            (&key_pairs).into(),
        ));
        key_pairs
    };
    let (ir_contract, _, compliance_contract) = init_identity_contracts(
        &mut chain,
        ADMIN,
        vec![NATIONALITY_INDIA.to_string(), NATIONALITY_USA.to_string()],
    );
    add_identities(
        &mut chain,
        ir_contract,
        ADMIN,
        vec![
            (TOKEN_OWNER.into(), NATIONALITY_INDIA.to_string()),
            (TOKEN_RECEIVER.into(), NATIONALITY_USA.to_string()),
        ],
    )
    .expect_report("Add identities");

    let sponsor = sponsor_deploy_and_init(&mut chain, SPONSOR_ACCOUNT);
    let (nft_contract, _) = init_security_token_contracts(
        &mut chain,
        ADMIN,
        ir_contract,
        compliance_contract,
        vec![sponsor],
    );
    let nft_token =
        nft_mint(&mut chain, nft_contract, ADMIN, Receiver::Account(TOKEN_OWNER), "ipfs:url1")
            .expect_report("Security NFT: Mint token 1");

    // Payload for NFT contract
    let payload = {
        let payload: concordium_rwa_security_nft::transfer::ContractTransferParams =
            TransferParams(vec![Transfer {
                from: TOKEN_OWNER.into(),
                amount: 1.into(),
                to: TOKEN_RECEIVER.into(),
                token_id: nft_token,
                data: AdditionalData::empty(),
            }]);
        let mut payload_bytes = Vec::new();
        payload.serial(&mut payload_bytes).expect("Serializing payload");
        payload_bytes
    };

    // Sponsor Contract Params
    let permit_param: PermitParam = {
        let nonce = get_nonce(&mut chain, sponsor, TOKEN_OWNER);
        let permit_message = PermitMessage {
            contract_address: nft_contract,
            entry_point: OwnedEntrypointName::new_unchecked("transfer".to_string()),
            nonce,
            timestamp: chain
                .block_time()
                .checked_add(Duration::from_seconds(1))
                .expect_report("Block time"),
            payload,
        };
        let bytes_to_sign = get_bytes_to_sign(&mut chain, sponsor, TOKEN_OWNER, &permit_message);
        PermitParam {
            signature: token_owner_key_pairs.sign_message(&bytes_to_sign),
            signer: TOKEN_OWNER,
            message: permit_message,
        }
    };

    chain
        .contract_update(
            Signer::with_one_key(),
            SPONSOR_ACCOUNT,
            Address::Account(SPONSOR_ACCOUNT),
            Energy::from(20000),
            UpdateContractPayload {
                amount: Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked("rwa_sponsor.permit".to_string()),
                address: sponsor,
                message: OwnedParameter::from_serial(&permit_param)
                    .expect_report("Serializing permit message"),
            },
        )
        .expect_report("Sponsor: Permit");
    let balance_of_token_owner =
        nft_balance_of(&mut chain, nft_contract, nft_token, TOKEN_OWNER.into());
    let balance_of_token_receiver =
        nft_balance_of(&mut chain, nft_contract, nft_token, TOKEN_RECEIVER.into());
    assert_eq!(balance_of_token_owner, TokenAmount::from(0));
    assert_eq!(balance_of_token_receiver, TokenAmount::from(1));
}

#[test]
pub fn sponsored_sft_transfer() {
    let mut chain = Chain::new();
    create_accounts(
        &mut chain,
        vec![DEFAULT_INVOKER, ADMIN, TOKEN_RECEIVER, SPONSOR_ACCOUNT],
        DEFAULT_ACC_BALANCE,
    );

    // Create an account for the token owner
    let token_owner_key_pairs = {
        let rng = &mut rand::thread_rng();
        let key_pairs = AccountKeys::singleton(rng);
        chain.create_account(Account::new_with_keys(
            TOKEN_OWNER,
            AccountBalance::new(DEFAULT_ACC_BALANCE, Amount::zero(), Amount::zero()).unwrap(),
            (&key_pairs).into(),
        ));
        key_pairs
    };
    let (ir_contract, _, compliance_contract) = init_identity_contracts(
        &mut chain,
        ADMIN,
        vec![NATIONALITY_INDIA.to_string(), NATIONALITY_USA.to_string()],
    );
    add_identities(
        &mut chain,
        ir_contract,
        ADMIN,
        vec![
            (TOKEN_OWNER.into(), NATIONALITY_INDIA.to_string()),
            (TOKEN_RECEIVER.into(), NATIONALITY_USA.to_string()),
        ],
    )
    .expect_report("Add identities");

    let sponsor = sponsor_deploy_and_init(&mut chain, SPONSOR_ACCOUNT);
    let (sft_contract, sft_token) = {
        let (nft_contract, sft_contract) = init_security_token_contracts(
            &mut chain,
            ADMIN,
            ir_contract,
            compliance_contract,
            vec![sponsor],
        );
        add_identities(
            &mut chain,
            ir_contract,
            ADMIN,
            vec![(sft_contract.into(), NATIONALITY_INDIA.to_string())],
        )
        .expect_report("Add SFT identity");
        let sft_token = sft_mint(&mut chain, ADMIN, nft_contract, sft_contract, TOKEN_OWNER);
        (sft_contract, sft_token)
    };

    // Payload for SFT contract
    let payload = {
        let payload: concordium_rwa_security_sft::transfer::ContractTransferParams =
            TransferParams(vec![Transfer {
                from: TOKEN_OWNER.into(),
                amount: TokenAmountU32(100),
                to: TOKEN_RECEIVER.into(),
                token_id: sft_token,
                data: AdditionalData::empty(),
            }]);
        let mut payload_bytes = Vec::new();
        payload.serial(&mut payload_bytes).expect("Serializing payload");
        payload_bytes
    };

    // Sponsor Contract Params
    let permit_param: PermitParam = {
        let nonce = get_nonce(&mut chain, sponsor, TOKEN_OWNER);
        let permit_message = PermitMessage {
            contract_address: sft_contract,
            entry_point: OwnedEntrypointName::new_unchecked("transfer".to_string()),
            nonce,
            timestamp: chain
                .block_time()
                .checked_add(Duration::from_seconds(1))
                .expect_report("Block time"),
            payload,
        };
        let bytes_to_sign = get_bytes_to_sign(&mut chain, sponsor, TOKEN_OWNER, &permit_message);
        PermitParam {
            signature: token_owner_key_pairs.sign_message(&bytes_to_sign),
            signer: TOKEN_OWNER,
            message: permit_message,
        }
    };

    chain
        .contract_update(
            Signer::with_one_key(),
            SPONSOR_ACCOUNT,
            Address::Account(SPONSOR_ACCOUNT),
            Energy::from(20000),
            UpdateContractPayload {
                amount: Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked("rwa_sponsor.permit".to_string()),
                address: sponsor,
                message: OwnedParameter::from_serial(&permit_param)
                    .expect_report("Serializing permit message"),
            },
        )
        .expect_report("Sponsor: Permit");
    let balance_of_token_owner =
        sft_balance_of(&mut chain, sft_contract, sft_token, TOKEN_OWNER.into());
    let balance_of_token_receiver =
        sft_balance_of(&mut chain, sft_contract, sft_token, TOKEN_RECEIVER.into());
    assert_eq!(balance_of_token_owner, 900.into());
    assert_eq!(balance_of_token_receiver, 100.into());
}
