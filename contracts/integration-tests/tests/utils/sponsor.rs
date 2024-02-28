use super::consts::*;
use concordium_rwa_sponsor::{types::PermitMessage, utils::NonceParam};
use concordium_smart_contract_testing::*;
use concordium_std::ExpectReport;

pub fn sponsor_deploy_and_init(chain: &mut Chain, owner: AccountAddress) -> ContractAddress {
    let module = chain
        .module_deploy_v1(Signer::with_one_key(), owner, module_load_v1(SPONSOR_MODULE).unwrap())
        .unwrap()
        .module_reference;

    chain
        .contract_init(
            Signer::with_one_key(),
            owner,
            Energy::from(30000),
            InitContractPayload {
                mod_ref: module,
                amount: Amount::zero(),
                init_name: OwnedContractName::new_unchecked(SPONSOR_CONTRACT_NAME.to_owned()),
                param: OwnedParameter::empty(),
            },
        )
        .expect_report("Security NFT: Init")
        .contract_address
}

pub fn get_nonce(chain: &mut Chain, contract: ContractAddress, account: AccountAddress) -> u64 {
    chain
        .contract_invoke(
            account,
            account.into(),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked("rwa_sponsor.nonce".to_string()),
                address: contract,
                message: OwnedParameter::from_serial(&NonceParam {
                    account,
                })
                .expect_report("Serializing nonce"),
            },
        )
        .expect_report("Get nonce")
        .parse_return_value::<u64>()
        .expect_report("Parsing nonce")
}

pub fn get_bytes_to_sign(
    chain: &mut Chain,
    contract: ContractAddress,
    account: AccountAddress,
    message: &PermitMessage,
) -> [u8; 32] {
    chain
        .contract_invoke(
            account,
            account.into(),
            Energy::from(10000),
            UpdateContractPayload {
                amount: Amount::zero(),
                receive_name: OwnedReceiveName::new_unchecked(
                    "rwa_sponsor.bytesToSign".to_string(),
                ),
                address: contract,
                message: OwnedParameter::from_serial(message).unwrap(),
            },
        )
        .expect_report("Get bytes to sign")
        .parse_return_value::<[u8; 32]>()
        .expect_report("Parsing bytes to sign")
}
