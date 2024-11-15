#![allow(unused)]

use concordium_base::smart_contracts::WasmModule;
use concordium_rwa_identity_registry::identities::RegisterIdentityParams;
use concordium_rwa_identity_registry::types::{Identity, IdentityAttribute};
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

const MODULE_BYTES: &[u8] = include_bytes!("../../identity-registry/contract.wasm.v1");
const CONTRACT_NAME: ContractName = ContractName::new_unchecked("init_rwa_identity_registry");

const NATIONALITY_ATTRIBUTE_TAG: u8 = 5;

pub fn deploy_module(chain: &mut Chain, sender: &Account) -> ModuleDeploySuccess {
    let module = WasmModule::from_slice(MODULE_BYTES).unwrap();
    chain
        .module_deploy_v1(Signer::with_one_key(), sender.address, module)
        .expect("deploying module")
}

pub fn init(
    chain: &mut Chain,
    sender: &Account,
) -> Result<(ContractInitSuccess, ModuleReference, OwnedContractName), ContractInitError> {
    let module_ref = WasmModule::from_slice(MODULE_BYTES)
        .unwrap()
        .get_module_ref();
    let contract_name = CONTRACT_NAME.to_owned();
    let init = chain.contract_init(
        Signer::with_one_key(),
        sender.address,
        MAX_ENERGY,
        InitContractPayload {
            amount:    Amount::zero(),
            init_name: CONTRACT_NAME.to_owned(),
            mod_ref:   module_ref,
            param:     OwnedParameter::empty(),
        },
    )?;
    Ok((init, module_ref, contract_name))
}

pub fn registry_identity(
    chain: &mut Chain,
    sender: &Account,
    contract: &ContractAddress,
    params: &RegisterIdentityParams,
) -> Result<ContractInvokeSuccess, ContractInvokeError> {
    let call: ContractInvokeSuccess = chain.contract_update(
        Signer::with_one_key(),
        sender.address,
        sender.address.into(),
        MAX_ENERGY,
        UpdateContractPayload {
            address:      *contract,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("registerIdentity"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        },
    )?;
    Ok(call)
}

pub fn register_nationalities(
    chain: &mut Chain,
    sender: &Account,
    contract: &ContractAddress,
    nationalities: Vec<(Address, &str)>,
) -> Vec<Result<ContractInvokeSuccess, ContractInvokeError>> {
    nationalities
        .iter()
        .map(|(address, nationality)| {
            registry_identity(chain, sender, contract, &RegisterIdentityParams {
                address:  *address,
                identity: Identity {
                    attributes:  vec![IdentityAttribute {
                        tag:   NATIONALITY_ATTRIBUTE_TAG,
                        value: nationality.to_string(),
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
) -> Result<ContractInvokeSuccess, ContractInvokeError> {
    chain.contract_update(
        Signer::with_one_key(),
        sender.address,
        sender.address.into(),
        MAX_ENERGY,
        UpdateContractPayload {
            address:      *contract,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("addAgent"),
            ),
            message:      OwnedParameter::from_serial(agent_address).unwrap(),
        },
    )
}
