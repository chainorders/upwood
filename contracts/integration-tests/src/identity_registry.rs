#![allow(unused)]

use concordium_base::smart_contracts::WasmModule;
use concordium_rwa_identity_registry::types::{
    Identity, IdentityAttribute, RegisterIdentityParams,
};
use concordium_smart_contract_testing::{
    module_load_v1, Account, Chain, ContractEvent, ContractInitError, ContractInitSuccess,
    ContractInvokeError, ContractInvokeSuccess, InitContractPayload, ModuleDeploySuccess, Signer,
    UpdateContractPayload,
};
use concordium_std::{
    Address, Amount, ContractAddress, ContractName, EntrypointName, ModuleReference,
    OwnedContractName, OwnedParameter, OwnedReceiveName,
};

use super::MAX_ENERGY;
const SIGNER: Signer = Signer::with_one_key();
use crate::contract_base::{ContractPayloads, ContractTestClient};

const MODULE_BYTES: &[u8] = include_bytes!("../../identity-registry/contract.wasm.v1");
const CONTRACT_NAME: ContractName = ContractName::new_unchecked("init_rwa_identity_registry");

const NATIONALITY_ATTRIBUTE_TAG: u8 = 5;

pub trait IdentityRegistryPayloads: ContractPayloads<()> {
    fn add_agent_payload(&self, agent: Address) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("addAgent"),
            ),
            message:      OwnedParameter::from_serial(&agent).unwrap(),
        }
    }
    fn register_identity_payload(&self, params: RegisterIdentityParams) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("registerIdentity"),
            ),
            message:      OwnedParameter::from_serial(&params).unwrap(),
        }
    }
}
#[derive(Clone, Copy)]
pub struct IdentityRegistryTestClient(pub ContractAddress);
impl ContractPayloads<()> for IdentityRegistryTestClient {
    fn module() -> WasmModule { WasmModule::from_slice(MODULE_BYTES).unwrap() }

    fn contract_name() -> OwnedContractName { CONTRACT_NAME.to_owned() }

    fn contract_address(&self) -> ContractAddress { self.0 }
}
impl ContractTestClient<()> for IdentityRegistryTestClient {
    fn new(contract_address: ContractAddress) -> Self { Self(contract_address) }
}

impl IdentityRegistryTestClient {
    pub fn add_agent(
        &self,
        chain: &mut Chain,
        sender: &Account,
        agent: Address,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            SIGNER,
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.add_agent_payload(agent),
        )
    }

    pub fn register_identity(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: RegisterIdentityParams,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            SIGNER,
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.register_identity_payload(params),
        )
    }
}
impl IdentityRegistryPayloads for IdentityRegistryTestClient {}

pub fn deploy_module(chain: &mut Chain, sender: &Account) -> ModuleDeploySuccess {
    let module = WasmModule::from_slice(MODULE_BYTES).unwrap();
    chain
        .module_deploy_v1(Signer::with_one_key(), sender.address, module)
        .expect("deploying module")
}
