use concordium_base::smart_contracts::WasmModule;
use concordium_smart_contract_testing::*;

use super::MAX_ENERGY;
const MODULE_BYTES: &[u8] = include_bytes!("../../compliance/contract.wasm.v1");

pub fn deploy_module(chain: &mut Chain, sender: &Account) -> ModuleDeploySuccess {
    let module = WasmModule::from_slice(MODULE_BYTES).unwrap();
    chain
        .module_deploy_v1(Signer::with_one_key(), sender.address, module)
        .expect("deploying module")
}

pub fn init(
    chain: &mut Chain,
    sender: &Account,
    compliance_modules: Vec<ContractAddress>,
) -> Result<(ContractInitSuccess, ModuleReference, OwnedContractName), ContractInitError> {
    let module_ref = WasmModule::from_slice(MODULE_BYTES)
        .unwrap()
        .get_module_ref();
    let contract_name = OwnedContractName::new_unchecked("init_rwa_compliance".to_string());
    let init = chain.contract_init(
        Signer::with_one_key(),
        sender.address,
        MAX_ENERGY,
        InitContractPayload {
            amount:    Amount::zero(),
            init_name: OwnedContractName::new_unchecked("init_rwa_compliance".to_string()),
            mod_ref:   module_ref,
            param:     OwnedParameter::from_serial(
                &concordium_rwa_compliance::compliance::init::InitParams {
                    modules: compliance_modules,
                },
            )
            .unwrap(),
        },
    )?;

    Ok((init, module_ref, contract_name))
}

pub fn init_nationalities(
    chain: &mut Chain,
    sender: &Account,
    param: &concordium_rwa_compliance::compliance_modules::allowed_nationalities::init::InitParams,
) -> Result<(ContractInitSuccess, ModuleReference, OwnedContractName), ContractInitError> {
    let module_ref = WasmModule::from_slice(MODULE_BYTES)
        .unwrap()
        .get_module_ref();
    let contract_name = OwnedContractName::new_unchecked(
        "init_rwa_compliance_module_allowed_nationalities".to_string(),
    );
    let init = chain.contract_init(
        Signer::with_one_key(),
        sender.address,
        MAX_ENERGY,
        InitContractPayload {
            amount:    Amount::zero(),
            init_name: contract_name.clone(),
            mod_ref:   module_ref,
            param:     OwnedParameter::from_serial(param).unwrap(),
        },
    )?;

    Ok((init, module_ref, contract_name))
}
