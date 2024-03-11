#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]
mod utils;

use concordium_cis2::TokenAmountU32;
use concordium_smart_contract_testing::*;
use integration_tests::{
    cis2_test_contract::ICis2Contract,
    identity_registry::IIdentityRegistryContract,
    security_sft::sft_mint,
    test_contract_client::ITestContract,
    verifier::Verifier,
};
use utils::{
    chain::create_accounts,
    common::{init_identity_contracts, init_security_token_contracts},
    consts::*,
};

const ADMIN: AccountAddress = AccountAddress([0; 32]);
const AGENT: AccountAddress = AccountAddress([1; 32]);
const TOKEN_OWNER: AccountAddress = AccountAddress([2; 32]);

#[test]
fn sft_fractionalize_via_transfer() {
    let mut chain = Chain::new();
    let accounts =
        create_accounts(&mut chain, vec![(ADMIN), (AGENT), (TOKEN_OWNER)], DEFAULT_ACC_BALANCE);
    let admin = accounts.first().expect("Admin account");
    let agent = accounts.get(1).expect("Agent account");
    let owner = accounts.get(2).expect("Owner account");

    let (ir_contract, compliance_contract) =
        init_identity_contracts(&mut chain, admin, vec!["IN".to_owned(), "US".to_owned()]);
    ir_contract.add_agent().update(&mut chain, admin, &Address::Account(AGENT)).expect("Add agent");
    let verifier = Verifier {
        account:           agent.clone(),
        identity_registry: ir_contract.clone(),
    };

    let (nft_contract, sft_contract) = init_security_token_contracts(
        &mut chain,
        admin,
        &ir_contract,
        &compliance_contract,
        vec![],
    )
    .expect("Init security token contracts");

    verifier
        .register_nationalities(&mut chain, vec![
            (Address::Account(TOKEN_OWNER), "US".to_string()),
            (Address::Contract(sft_contract.contract_address()), "US".to_string()),
        ])
        .expect("Register nationalities");

    let sft_token = sft_mint(
        &mut chain,
        admin,
        &nft_contract,
        &sft_contract,
        owner,
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
            owner,
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
}
