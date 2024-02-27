use super::{consts::*, security_nft::nft_mint};
use concordium_cis2::{
    AdditionalData, BalanceOfQuery, BalanceOfQueryParams, BalanceOfQueryResponse, Cis2Event,
    Receiver, TokenAmountU8, TokenIdU8, TransferParams,
};
use concordium_rwa_market::types::Rate;
use concordium_rwa_security_sft::{
    event::Event,
    init::InitParam,
    mint::{AddParam, AddParams, MintParam},
    types::NftTokenUId,
};
use concordium_rwa_utils::cis2_types::{SftTokenAmount, SftTokenId};
use concordium_smart_contract_testing::*;
use concordium_std::ExpectReport;

pub fn security_sft_deploy_and_init(
    chain: &mut Chain,
    owner: AccountAddress,
    compliance: ContractAddress,
    identity_registry: ContractAddress,
    sponsors: Vec<ContractAddress>,
) -> ContractAddress {
    let security_nft_module = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            owner,
            module_load_v1(SECURITY_SFT_MODULE).unwrap(),
        )
        .unwrap()
        .module_reference;

    chain
        .contract_init(
            Signer::with_one_key(),
            owner,
            Energy::from(30000),
            InitContractPayload {
                mod_ref: security_nft_module,
                amount: Amount::zero(),
                init_name: OwnedContractName::new_unchecked(SECURITY_SFT_CONTRACT_NAME.to_owned()),
                param: OwnedParameter::from_serial(&InitParam {
                    compliance,
                    identity_registry,
                    sponsors,
                })
                .expect_report("Security SFT: Init"),
            },
        )
        .expect_report("Security SFT: Init")
        .contract_address
}

pub fn sft_add_token(
    chain: &mut Chain,
    sft_contract: ContractAddress,
    agent: AccountAddress,
    param: AddParam,
) -> Result<SftTokenId, ContractInvokeError> {
    chain
        .contract_update(
            Signer::with_one_key(),
            agent,
            Address::Account(agent),
            Energy::from(30000),
            UpdateContractPayload {
                amount: Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked(
                    "rwa_security_sft.addTokens".to_string(),
                ),
                address: sft_contract,
                message: OwnedParameter::from_serial(&AddParams {
                    tokens: vec![param],
                })
                .unwrap(),
            },
        )
        .map(|res| {
            res.events()
                .filter_map(|(contract_address, events)| {
                    if contract_address != sft_contract {
                        return None;
                    }

                    let binding = events
                        .iter()
                        .filter_map(|e| {
                            let e: Event = e.parse().unwrap();
                            if let Event::Cis2(Cis2Event::TokenMetadata(e)) = e {
                                Some(e.token_id)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    Some(binding)
                })
                .flatten()
                .collect::<Vec<_>>()
        })
        .map(|token_ids| *token_ids.first().expect_report("Added Token"))
}

pub fn sft_transfer_and_fractionalize(
    chain: &mut Chain,
    security_nft_contract: ContractAddress,
    from: AccountAddress,
    to: AccountAddress,
    nft_token_id: TokenIdU8,
    sft_contract: ContractAddress,
) -> Result<ContractInvokeSuccess, ContractInvokeError> {
    let sft_mint_params = MintParam {
        deposited_token_id: NftTokenUId {
            id: to_token_id_vec(nft_token_id),
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
                    token_id: nft_token_id,
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

pub fn sft_balance_of(
    chain: &mut Chain,
    security_sft_contract: ContractAddress,
    security_sft_token_id: SftTokenId,
    owner: Address,
) -> SftTokenAmount {
    let balances = chain
        .contract_invoke(
            DEFAULT_INVOKER,
            owner,
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked(
                    "rwa_security_sft.balanceOf".to_string(),
                ),
                address: security_sft_contract,
                message: OwnedParameter::from_serial(&BalanceOfQueryParams {
                    queries: vec![BalanceOfQuery {
                        address: owner,
                        token_id: security_sft_token_id,
                    }],
                })
                .expect_report("Serialize Balance Of Query Params"),
            },
        )
        .expect_report("Security Nft: Balance Of")
        .parse_return_value::<BalanceOfQueryResponse<SftTokenAmount>>()
        .expect_report("Parsed Balance of Security Nft Token");

    let balance = balances.0.first().expect_report("Balance of Security Nft Token");
    *balance
}

pub fn sft_mint(
    chain: &mut Chain,
    agent: AccountAddress,
    nft_contract: ContractAddress,
    sft_contract: ContractAddress,
    owner: AccountAddress,
) -> concordium_cis2::TokenIdU32 {
    let nft_token = nft_mint(chain, nft_contract, agent, Receiver::Account(owner), "ipfs:url1")
        .expect_report("Security NFT: Mint token");

    let sft_token = sft_add_token(
        chain,
        sft_contract,
        agent,
        AddParam {
            deposit_token_id: NftTokenUId {
                contract: nft_contract,
                id: to_token_id_vec(nft_token),
            },
            metadata_url: concordium_rwa_security_sft::types::ContractMetadataUrl {
                url: "ipfs:url2".to_string(),
                hash: None,
            },
            fractions_rate: Rate::new(1000, 1).expect_report("Rate"),
        },
    )
    .expect_report("SFT: Add token");

    sft_transfer_and_fractionalize(chain, nft_contract, owner, owner, nft_token, sft_contract)
        .expect_report("SFT: Transfer and list");

    sft_token
}
