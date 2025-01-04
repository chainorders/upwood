use concordium_base::smart_contracts::WasmModule;
use concordium_smart_contract_testing::*;
use concordium_std::ContractName;
use nft_multi_rewarded::types::{Agent, InitParam};
use nft_multi_rewarded::TransferMintParams;

pub use super::cis2::*;
use crate::contract_base::{ContractPayloads, ContractTestClient};
use crate::MAX_ENERGY;
const MODULE_BYTES: &[u8] = include_bytes!("../../nft-multi-rewarded/contract.wasm.v1");
pub const CONTRACT_NAME: ContractName = ContractName::new_unchecked("init_nft_multi_rewarded");

pub struct NftMultiRewardedTestClient(pub ContractAddress);

impl ContractTestClient<InitParam> for NftMultiRewardedTestClient {
    fn new(contract_address: ContractAddress) -> Self { Self(contract_address) }
}

impl ContractPayloads<InitParam> for NftMultiRewardedTestClient {
    fn module() -> WasmModule { WasmModule::from_slice(MODULE_BYTES).unwrap() }

    fn contract_name() -> OwnedContractName { CONTRACT_NAME.to_owned() }

    fn contract_address(&self) -> ContractAddress { self.0 }
}

impl NftMultiRewardedClientPayloads for NftMultiRewardedTestClient {}

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
    let res = chain.contract_init(
        Signer::with_one_key(),
        sender.address,
        MAX_ENERGY,
        NftMultiRewardedTestClient::init_payload(params),
    )?;
    Ok((
        res,
        NftMultiRewardedTestClient::module().get_module_ref(),
        NftMultiRewardedTestClient::contract_name(),
    ))
}

pub trait NftMultiRewardedClientPayloads: ContractPayloads<InitParam> {
    fn add_agent_payload(&self, params: &Agent) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("addAgent"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }

    fn transfer_mint_payload(&self, params: &TransferMintParams) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("transferMint"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }
}

impl NftMultiRewardedTestClient {
    pub fn add_agent(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &Agent,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.add_agent_payload(params),
        )
    }

    pub fn transfer_mint(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &TransferMintParams,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.transfer_mint_payload(params),
        )
    }
}
