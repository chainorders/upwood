use std::error::Error;

use concordium_rwa_identity_registry::identities::RegisterIdentityParams;
use concordium_rwa_identity_registry::types::{Identity, IdentityAttribute};
use concordium_smart_contract_testing::{
    module_load_v1, Account, Chain, ContractInitSuccess, ContractInvokeSuccess,
    InitContractPayload, ModuleDeploySuccess, Signer, UpdateContractPayload,
};
use concordium_std::{Address, Amount, ContractAddress, OwnedContractName, OwnedParameter};

use super::MAX_ENERGY;

pub type ContractResult<T> = Result<T, dyn Error>;
const MODULE_PATH: &str = "../identity-registry/contract.wasm.v1";
const NATIONALITY_ATTRIBUTE_TAG: u8 = 5;

pub fn deploy_module(chain: &mut Chain, sender: &Account) -> ModuleDeploySuccess {
    chain
        .module_deploy_v1(
            Signer::with_one_key(),
            sender.address,
            module_load_v1(MODULE_PATH).unwrap(),
        )
        .expect("deploying module")
}

pub fn init(chain: &mut Chain, sender: &Account) -> ContractInitSuccess {
    chain
        .contract_init(
            Signer::with_one_key(),
            sender.address,
            MAX_ENERGY,
            InitContractPayload {
                amount:    Amount::zero(),
                init_name: OwnedContractName::new_unchecked(
                    "init_rwa_identity_registry".to_string(),
                ),
                mod_ref:   module_load_v1(MODULE_PATH).unwrap().get_module_ref(),
                param:     OwnedParameter::empty(),
            },
        )
        .expect("init")
}

pub fn registry_identity(
    chain: &mut Chain,
    sender: &Account,
    contract: &ContractAddress,
    params: &RegisterIdentityParams,
) -> ContractInvokeSuccess {
    chain
        .contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      *contract,
                amount:       Amount::zero(),
                receive_name: "registry_identity".parse().unwrap(),
                message:      OwnedParameter::from_serial(params).unwrap(),
            },
        )
        .expect("registry identity")
}

pub fn register_nationalities(
    chain: &mut Chain,
    sender: &Account,
    contract: &ContractAddress,
    nationalities: Vec<(Address, String)>,
) -> Vec<ContractInvokeSuccess> {
    nationalities
        .iter()
        .map(|(address, nationality)| {
            registry_identity(chain, sender, contract, &RegisterIdentityParams {
                address:  *address,
                identity: Identity {
                    attributes:  vec![IdentityAttribute {
                        tag:   NATIONALITY_ATTRIBUTE_TAG,
                        value: nationality.to_owned(),
                    }],
                    credentials: vec![],
                },
            })
        })
        .collect()
}

pub fn add_agent(
    chain: &mut Chain,
    sender: &Account,
    contract: &ContractAddress,
    agent_address: &Address,
) -> ContractInvokeSuccess {
    chain
        .contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            UpdateContractPayload {
                address:      *contract,
                amount:       Amount::zero(),
                receive_name: "registry_identity".parse().unwrap(),
                message:      OwnedParameter::from_serial(agent_address).unwrap(),
            },
        )
        .expect("add agent")
}
