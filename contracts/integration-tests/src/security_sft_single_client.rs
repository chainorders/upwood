#![allow(unused)]

use concordium_base::smart_contracts::WasmModule;
use concordium_cis2::{TransferParams, UpdateOperator, UpdateOperatorParams};
use concordium_smart_contract_testing::*;
use security_sft_single::types::*;

use super::{cis2, cis2_security, MAX_ENERGY};
use crate::cis2_security::{
    Cis2Payloads, Cis2SecurityPayloads, Cis2SecurityTestClient, Cis2TestClient,
};
use crate::contract_base::{ContractPayloads, ContractTestClient};

pub const MODULE_BYTES: &[u8] = include_bytes!("../../security-sft-single/contract.wasm.v1");
pub const CONTRACT_NAME: ContractName = ContractName::new_unchecked("init_security_sft_single");

#[derive(Clone, Copy)]
pub struct SftSingleTestClient(pub ContractAddress);
impl ContractPayloads<InitParam> for SftSingleTestClient {
    fn module() -> WasmModule { WasmModule::from_slice(MODULE_BYTES).unwrap() }

    fn contract_name() -> OwnedContractName { CONTRACT_NAME.to_owned() }

    fn contract_address(&self) -> ContractAddress { self.0 }
}
impl Cis2Payloads<InitParam, TokenId, TokenAmount> for SftSingleTestClient {}
impl ContractTestClient<InitParam> for SftSingleTestClient {
    fn new(contract_address: ContractAddress) -> Self { Self(contract_address) }
}
impl Cis2TestClient<InitParam, TokenId, TokenAmount> for SftSingleTestClient {}
impl Cis2SecurityPayloads<InitParam, AgentRole, TokenId, TokenAmount> for SftSingleTestClient {}
impl Cis2SecurityTestClient<InitParam, AgentRole, TokenId, TokenAmount> for SftSingleTestClient {}

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
        SftSingleTestClient::init_payload(params),
    )?;
    Ok((
        res,
        SftSingleTestClient::module().get_module_ref(),
        SftSingleTestClient::contract_name(),
    ))
}
