use concordium_base::smart_contracts::WasmModule;
use concordium_protocols::concordium_cis2_security::AgentWithRoles;
use concordium_smart_contract_testing::*;
use concordium_std::ContractName;
use security_sft_multi_yielder::{
    AgentRole, InitParam, RemoveYieldParams, UpsertYieldParams, YieldParams,
};

use crate::contract_base::{ContractPayloads, ContractTestClient};
use crate::MAX_ENERGY;

pub const MODULE_BYTES: &[u8] = include_bytes!("../../security-sft-multi-yielder/contract.wasm.v1");
pub const CONTRACT_NAME: ContractName =
    ContractName::new_unchecked("init_security_sft_multi_yielder");

pub struct SftMultiYielderTestClient(pub ContractAddress);

impl ContractPayloads<InitParam> for SftMultiYielderTestClient {
    fn module() -> WasmModule { WasmModule::from_slice(MODULE_BYTES).unwrap() }

    fn contract_name() -> OwnedContractName { CONTRACT_NAME.to_owned() }

    fn contract_address(&self) -> ContractAddress { self.0 }
}

impl ContractTestClient<InitParam> for SftMultiYielderTestClient {
    fn new(contract_address: ContractAddress) -> Self { Self(contract_address) }
}

impl SftMultiYielderTestClient {
    pub fn upsert_yield_payload(&self, params: &UpsertYieldParams) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.0,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("upsertYield"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }

    pub fn remove_yield_payload(&self, params: &RemoveYieldParams) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.0,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("removeYield"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }

    pub fn yield_for_payload(&self, params: &YieldParams) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.0,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("yieldFor"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }

    pub fn add_agent_payload(&self, params: &AgentWithRoles<AgentRole>) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.0,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("addAgent"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }

    pub fn remove_agent_payload(&self, params: &Address) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.0,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("removeAgent"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }
}

impl SftMultiYielderTestClient {
    pub fn upsert_yield(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &UpsertYieldParams,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.upsert_yield_payload(params),
        )
    }

    pub fn remove_yield(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &RemoveYieldParams,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.remove_yield_payload(params),
        )
    }

    pub fn yield_for(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &YieldParams,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.yield_for_payload(params),
        )
    }

    pub fn add_agent(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &AgentWithRoles<AgentRole>,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.add_agent_payload(params),
        )
    }

    pub fn remove_agent(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &Address,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.remove_agent_payload(params),
        )
    }
}

pub fn deploy_module(chain: &mut Chain, sender: &Account) -> ModuleDeploySuccess {
    let module = WasmModule::from_slice(MODULE_BYTES).unwrap();
    chain
        .module_deploy_v1(Signer::with_one_key(), sender.address, module)
        .expect("deploying module")
}
