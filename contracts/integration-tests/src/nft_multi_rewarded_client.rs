use concordium_base::smart_contracts::WasmModule;
use concordium_smart_contract_testing::*;
use concordium_std::ContractName;
use nft_multi_rewarded::types::{Agent, InitParam};
use nft_multi_rewarded::TransferMintParams;

pub use super::cis2::*;
use crate::MAX_ENERGY;
const MODULE_BYTES: &[u8] = include_bytes!("../../nft-multi-rewarded/contract.wasm.v1");
pub const CONTRACT_NAME: ContractName = ContractName::new_unchecked("init_nft_multi_rewarded");

pub struct NftMultiRewardedTestClient(pub ContractAddress);
impl NftMultiRewardedTestClient {
    pub fn module() -> WasmModule { WasmModule::from_slice(MODULE_BYTES).unwrap() }

    pub fn init_payload(init_params: &InitParam) -> InitContractPayload {
        InitContractPayload {
            amount:    Amount::zero(),
            init_name: CONTRACT_NAME.to_owned(),
            mod_ref:   Self::module().get_module_ref(),
            param:     OwnedParameter::from_serial(init_params).unwrap(),
        }
    }
}

pub fn deploy_module(chain: &mut Chain, sender: &Account) -> ModuleDeploySuccess {
    let module = WasmModule::from_slice(MODULE_BYTES).unwrap();
    chain
        .module_deploy_v1(Signer::with_one_key(), sender.address, module)
        .expect("deploying module")
}

pub fn init(
    chain: &mut Chain,
    sender: &Account,
    params: &InitParam,
) -> Result<(ContractInitSuccess, ModuleReference, OwnedContractName), ContractInitError> {
    let module_ref = WasmModule::from_slice(MODULE_BYTES)
        .unwrap()
        .get_module_ref();
    let contract_name = CONTRACT_NAME.to_owned();
    let init = chain
        .contract_init(
            Signer::with_one_key(),
            sender.address,
            MAX_ENERGY,
            InitContractPayload {
                amount:    Amount::zero(),
                init_name: contract_name.clone(),
                mod_ref:   module_ref,
                param:     OwnedParameter::from_serial(params).unwrap(),
            },
        )
        .expect("Failed to init nft contract");

    Ok((init, module_ref, contract_name))
}

pub fn add_agent(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &Agent,
) -> Result<ContractInvokeSuccess, ContractInvokeError> {
    chain.contract_update(
        Signer::with_one_key(),
        sender.address,
        sender.address.into(),
        MAX_ENERGY,
        UpdateContractPayload {
            address:      contract,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("addAgent"),
            ),
            message:      OwnedParameter::from_serial(payload).unwrap(),
        },
    )
}

pub fn transfer_mint(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &TransferMintParams,
) -> Result<ContractInvokeSuccess, ContractInvokeError> {
    chain.contract_update(
        Signer::with_one_key(),
        sender.address,
        sender.address.into(),
        MAX_ENERGY,
        UpdateContractPayload {
            address:      contract,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("transferMint"),
            ),
            message:      OwnedParameter::from_serial(payload).unwrap(),
        },
    )
}
