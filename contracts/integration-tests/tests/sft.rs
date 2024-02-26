#![allow(clippy::diverging_sub_expression, clippy::too_many_arguments)]

mod utils;
use concordium_cis2::{AdditionalData, Receiver, TokenAmountU8, TokenIdU8, TransferParams};

use concordium_rwa_security_sft::{
    mint::{AddParam, MintParam},
    types::NftTokenUId,
};
use concordium_rwa_utils::cis2_conversions::Rate;
use concordium_smart_contract_testing::*;
use concordium_std::ExpectReport;
use utils::{
    chain::{create_accounts, init_identity_contracts, init_security_token_contracts},
    consts::*,
    identity_registry::add_identities,
    security_nft::nft_mint,
    security_sft::sft_add_token,
};

const ADMIN: AccountAddress = AccountAddress([0; 32]);
const SELLER_ACC: AccountAddress = AccountAddress([2; 32]);

#[test]
fn sft_fractionalize_via_transfer() {
    let mut chain = Chain::new();
    create_accounts(&mut chain, vec![ADMIN, SELLER_ACC], DEFAULT_ACC_BALANCE);
    
    let (ir_contract, _, compliance_contract) =
        init_identity_contracts(&mut chain, ADMIN, vec!["IN".to_owned(), "US".to_owned()]);
    let (security_nft_contract, security_sft_contract) =
        init_security_token_contracts(&mut chain, ADMIN, ir_contract, compliance_contract, vec![]);
    add_identities(
        &mut chain,
        ir_contract,
        ADMIN,
        vec![
            (Address::Account(SELLER_ACC), "US".to_string()),
            (Address::Contract(security_sft_contract), "US".to_string()),
        ],
    )
    .expect("Add identities");

    let buy_token = nft_mint(
        &mut chain,
        security_nft_contract,
        ADMIN,
        Receiver::Account(SELLER_ACC),
        "ipfs:url1",
    )
    .expect_report("Security NFT: Mint token 1");

    sft_add_token(
        &mut chain,
        security_sft_contract,
        ADMIN,
        AddParam {
            deposit_token_id: NftTokenUId {
                contract: security_nft_contract,
                id: to_token_id_vec(buy_token),
            },
            metadata_url: concordium_rwa_security_sft::types::ContractMetadataUrl {
                url: "ipfs:url2".to_string(),
                hash: None,
            },
            fractions_rate: Rate::new(1, 10).expect_report("Rate"),
        },
    )
    .expect_report("SFT: Add token");

    sft_transfer_and_list(
        &mut chain,
        security_nft_contract,
        SELLER_ACC,
        SELLER_ACC,
        buy_token,
        security_sft_contract,
    )
    .expect_report("SFT: Transfer and list");
}

fn sft_transfer_and_list(
    chain: &mut Chain,
    security_nft_contract: ContractAddress,
    from: AccountAddress,
    to: AccountAddress,
    token_id: TokenIdU8,
    sft_contract: ContractAddress,
) -> Result<ContractInvokeSuccess, ContractInvokeError> {
    let sft_mint_params = MintParam {
        deposited_token_id: NftTokenUId {
            id: to_token_id_vec(token_id),
            contract: security_nft_contract,
        },
        deposited_amount: TokenAmountU8(1),
        deposited_token_owner: from,
        owner: to.into(),
    };
    chain.contract_update(
        Signer::with_one_key(),
        from,
        Address::Account(from),
        Energy::from(30000),
        UpdateContractPayload {
            amount: Amount::zero(),
            receive_name: OwnedReceiveName::new_unchecked("rwa_security_nft.transfer".to_string()),
            address: security_nft_contract,
            message: OwnedParameter::from_serial(&TransferParams(vec![
                concordium_cis2::Transfer {
                    token_id,
                    amount: TokenAmountU8(1),
                    from: Address::Account(from),
                    to: Receiver::Contract(
                        sft_contract,
                        OwnedEntrypointName::new_unchecked("deposit".to_string()),
                    ),
                    data: AdditionalData::from(to_bytes(&sft_mint_params)),
                },
            ]))
            .unwrap(),
        },
    )
}
