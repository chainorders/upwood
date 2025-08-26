use concordium_base::smart_contracts::WasmModule;
use concordium_smart_contract_testing::*;

use super::MAX_ENERGY;
use crate::contract_base::{ContractPayloads, ContractTestClient};
const MODULE_BYTES: &[u8] = include_bytes!("../../compliance/contract.wasm.v1");
pub const CONTRACT_NAME_COMPLIANCE: &str = "init_rwa_compliance";
pub const CONTRACT_NAME_NATIONALITIES: &str = "init_rwa_compliance_module_allowed_nationalities";
pub fn deploy_module(chain: &mut Chain, sender: &Account) -> ModuleDeploySuccess {
    let module = WasmModule::from_slice(MODULE_BYTES).unwrap();
    chain
        .module_deploy_v1(Signer::with_one_key(), sender.address, module)
        .expect("deploying module")
}

pub struct ComplianceTestClient(pub ContractAddress);
impl ContractPayloads<concordium_rwa_compliance::compliance::types::InitParams>
    for ComplianceTestClient
{
    fn module() -> WasmModule { WasmModule::from_slice(MODULE_BYTES).unwrap() }

    fn contract_name() -> OwnedContractName {
        OwnedContractName::new_unchecked(CONTRACT_NAME_COMPLIANCE.to_string())
    }

    fn contract_address(&self) -> ContractAddress { self.0 }
}
impl ContractTestClient<concordium_rwa_compliance::compliance::types::InitParams>
    for ComplianceTestClient
{
    fn new(contract_address: ContractAddress) -> Self { Self(contract_address) }
}

pub fn init(
    chain: &mut Chain,
    sender: &Account,
    compliance_modules: Vec<ContractAddress>,
) -> Result<(ContractInitSuccess, ModuleReference, OwnedContractName), ContractInitError> {
    let module_ref = WasmModule::from_slice(MODULE_BYTES)
        .unwrap()
        .get_module_ref();
    let contract_name = OwnedContractName::new_unchecked(CONTRACT_NAME_COMPLIANCE.to_string());
    let init = chain.contract_init(
        Signer::with_one_key(),
        sender.address,
        MAX_ENERGY,
        InitContractPayload {
            amount:    Amount::zero(),
            init_name: contract_name.clone(),
            mod_ref:   module_ref,
            param:     OwnedParameter::from_serial(
                &concordium_rwa_compliance::compliance::types::InitParams {
                    modules: compliance_modules,
                },
            )
            .unwrap(),
        },
    )?;

    Ok((init, module_ref, contract_name))
}

pub struct NationalitiesModuleTestClient(pub ContractAddress);
impl
    ContractPayloads<
        concordium_rwa_compliance::compliance_modules::allowed_nationalities::types::InitParams,
    > for NationalitiesModuleTestClient
{
    fn module() -> WasmModule { WasmModule::from_slice(MODULE_BYTES).unwrap() }

    fn contract_name() -> OwnedContractName {
        OwnedContractName::new_unchecked(CONTRACT_NAME_NATIONALITIES.to_string())
    }

    fn contract_address(&self) -> ContractAddress { self.0 }
}
impl
    ContractTestClient<
        concordium_rwa_compliance::compliance_modules::allowed_nationalities::types::InitParams,
    > for NationalitiesModuleTestClient
{
    fn new(contract_address: ContractAddress) -> Self { Self(contract_address) }
}

pub fn init_nationalities(
    chain: &mut Chain,
    sender: &Account,
    param: &concordium_rwa_compliance::compliance_modules::allowed_nationalities::types::InitParams,
) -> Result<(ContractInitSuccess, ModuleReference, OwnedContractName), ContractInitError> {
    let module_ref = WasmModule::from_slice(MODULE_BYTES)
        .unwrap()
        .get_module_ref();
    let contract_name = OwnedContractName::new_unchecked(CONTRACT_NAME_NATIONALITIES.to_string());

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
