use concordium_base::smart_contracts::WasmModule;
use concordium_smart_contract_testing::*;
use offchain_rewards::types::{Agent, ClaimRequest, InitParam};

const MODULE_BYTES: &[u8] = include_bytes!("../../offchain-rewards/contract.wasm.v1");
pub const CONTRACT_NAME: ContractName = ContractName::new_unchecked("init_offchain_rewards");

pub struct OffchainRewardsTestClient(pub ContractAddress);

impl OffchainRewardsTestClient {
    pub fn module() -> WasmModule { WasmModule::from_slice(MODULE_BYTES).unwrap() }

    pub fn init_payload(init_params: &InitParam) -> InitContractPayload {
        InitContractPayload {
            amount:    Amount::zero(),
            init_name: CONTRACT_NAME.to_owned(),
            mod_ref:   Self::module().get_module_ref(),
            param:     OwnedParameter::from_serial(init_params).unwrap(),
        }
    }

    pub fn add_agent_payload(&self, agent: &Agent) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.0,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("addAgent"),
            ),
            message:      OwnedParameter::from_serial(agent).unwrap(),
        }
    }

    pub fn claim_reward_payload(&self, claim_req: &ClaimRequest) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.0,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("claimReward"),
            ),
            message:      OwnedParameter::from_serial(claim_req).unwrap(),
        }
    }
}
