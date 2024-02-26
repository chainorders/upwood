use super::consts::*;
use concordium_cis2::Cis2Event;
use concordium_rwa_security_sft::{
    event::Event,
    init::InitParam,
    mint::{AddParam, AddParams},
};
use concordium_rwa_utils::cis2_types::SftTokenId;
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
