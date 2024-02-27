#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]

mod utils;
use concordium_cis2::TokenAmountU32;
use concordium_smart_contract_testing::*;
use utils::{
    chain::{create_accounts, init_identity_contracts, init_security_token_contracts},
    consts::*,
    identity_registry::add_identities,
    security_sft::sft_balance_of,
};

use crate::utils::security_sft::sft_mint;

const ADMIN: AccountAddress = AccountAddress([0; 32]);
const TOKEN_OWNER: AccountAddress = AccountAddress([2; 32]);

#[test]
fn sft_fractionalize_via_transfer() {
    let mut chain = Chain::new();
    create_accounts(&mut chain, vec![DEFAULT_INVOKER, ADMIN, TOKEN_OWNER], DEFAULT_ACC_BALANCE);

    let (ir_contract, _, compliance_contract) =
        init_identity_contracts(&mut chain, ADMIN, vec!["IN".to_owned(), "US".to_owned()]);
    let (nft_contract, sft_contract) =
        init_security_token_contracts(&mut chain, ADMIN, ir_contract, compliance_contract, vec![]);
    add_identities(
        &mut chain,
        ir_contract,
        ADMIN,
        vec![
            (Address::Account(TOKEN_OWNER), "US".to_string()),
            (Address::Contract(sft_contract), "US".to_string()),
        ],
    )
    .expect("Add identities");

    let sft_token = sft_mint(&mut chain, ADMIN, nft_contract, sft_contract, TOKEN_OWNER);
    let balance = sft_balance_of(&mut chain, sft_contract, sft_token, TOKEN_OWNER.into());
    assert_eq!(balance, TokenAmountU32(1000));
}
