use super::consts::*;
use concordium_cis2::{
    BalanceOfQuery, BalanceOfQueryParams, BalanceOfQueryResponse, Cis2Event, Receiver, TokenAmountU8, TokenIdU8
};
use concordium_rwa_security_nft::{
    event::Event, init::InitParam, mint::{MintParam, MintParams}, types::{ContractMetadataUrl, TokenId}
};
use concordium_smart_contract_testing::*;
use concordium_std::ExpectReport;

pub fn security_nft_deploy_and_init(
    chain: &mut Chain,
    owner: AccountAddress,
    compliance_contract: ContractAddress,
    ir_contract: ContractAddress,
) -> ContractAddress {
    let security_nft_module = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            owner,
            module_load_v1(SECURITY_NFT_MODULE).unwrap(),
        )
        .unwrap()
        .module_reference;

    chain
        .contract_init(Signer::with_one_key(), owner, Energy::from(30000), InitContractPayload {
            mod_ref:   security_nft_module,
            amount:    Amount::zero(),
            init_name: OwnedContractName::new_unchecked(SECURITY_NFT_CONTRACT_NAME.to_owned()),
            param:     OwnedParameter::from_serial(&InitParam {
                compliance:        compliance_contract,
                identity_registry: ir_contract,
                sponsors:          vec![],
            })
            .expect_report("Security NFT: Init"),
        })
        .expect_report("Security NFT: Init")
        .contract_address
}

pub fn nft_balance_of(
    chain: &mut Chain,
    security_nft_contract: ContractAddress,
    security_nft_token_id: TokenIdU8,
    owner: Address,
) -> TokenAmountU8 {
    let balances = chain
        .contract_invoke(DEFAULT_INVOKER, owner, Energy::from(10000), UpdateContractPayload {
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::new_unchecked("rwa_security_nft.balanceOf".to_string()),
            address:      security_nft_contract,
            message:      OwnedParameter::from_serial(&BalanceOfQueryParams {
                queries: vec![BalanceOfQuery {
                    address:  owner,
                    token_id: security_nft_token_id,
                }],
            })
            .expect_report("Serialize Balance Of Query Params"),
        })
        .expect_report("Security Nft: Balance Of")
        .parse_return_value::<BalanceOfQueryResponse<TokenAmountU8>>()
        .expect_report("Parsed Balance of Security Nft Token");

    let balance = balances.0.first().expect_report("Balance of Security Nft Token");
    *balance
}

pub fn nft_mint(
    chain: &mut Chain,
    security_nft_contract: ContractAddress,
    agent: AccountAddress,
    owner: Receiver,
    metadata_url: &str,
) -> Result<TokenId, ContractInvokeError> {
    chain
        .contract_update(
            Signer::with_one_key(),
            agent,
            Address::Account(agent),
            Energy::from(30000),
            UpdateContractPayload {
                amount:       Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked("rwa_security_nft.mint".to_string()),
                address:      security_nft_contract,
                message:      OwnedParameter::from_serial(&MintParams {
                    owner,
                    tokens: vec![MintParam {
                        metadata_url: ContractMetadataUrl {
                            url:  metadata_url.to_string(),
                            hash: None,
                        },
                    }],
                })
                .unwrap(),
            },
        )
        .map(|res| {
            res.events()
                .filter_map(|(contract_address, events)| {
                    if contract_address != security_nft_contract {
                        return None;
                    }

                    let binding = events
                        .iter()
                        .filter_map(|e| {
                            let e: Event = e.parse().unwrap();
                            if let Event::Cis2(Cis2Event::Mint(e)) = e {
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
        .map(|token_ids| *token_ids.first().expect_report("Minted Token"))
}
