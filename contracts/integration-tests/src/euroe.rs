#![allow(unused)]

use concordium_base::smart_contracts::WasmModule;
use concordium_cis2::{
    BalanceOfQueryResponse, TokenAmountU64, TokenIdUnit, UpdateOperator, UpdateOperatorParams,
};
use concordium_smart_contract_testing::*;
use concordium_std::{Deserial, SchemaType, Serial, Serialize};

use super::{cis2, MAX_ENERGY};
use crate::cis2_security::{Cis2Payloads, Cis2TestClient};
use crate::contract_base::{ContractPayloads, ContractTestClient};
pub const MODULE_BYTES: &[u8] = include_bytes!("../../euroe/dist/module.wasm.v1");
pub const CONTRACT_NAME: ContractName = ContractName::new_unchecked("init_euroe_stablecoin");

#[derive(Serial, Deserial, SchemaType)]
pub struct MintParams {
    pub owner:  Address,
    pub amount: TokenAmountU64,
}

#[derive(Serialize, SchemaType)]
pub struct RoleTypes {
    pub mintrole:  Address,
    pub burnrole:  Address,
    pub blockrole: Address,
    pub pauserole: Address,
    pub adminrole: Address,
}

#[derive(Clone, Copy)]
pub struct EuroETestClient(pub ContractAddress);
impl ContractPayloads<()> for EuroETestClient {
    fn module() -> WasmModule { WasmModule::from_slice(MODULE_BYTES).unwrap() }

    fn contract_name() -> OwnedContractName { CONTRACT_NAME.to_owned() }

    fn contract_address(&self) -> ContractAddress { self.0 }
}
impl Cis2Payloads<(), TokenIdUnit, TokenAmountU64> for EuroETestClient {}
impl ContractTestClient<()> for EuroETestClient {
    fn new(contract_address: ContractAddress) -> Self { Self(contract_address) }
}
impl Cis2TestClient<(), TokenIdUnit, TokenAmountU64> for EuroETestClient {}

impl EuroETestClient {
    pub fn module() -> WasmModule { WasmModule::from_slice(MODULE_BYTES).unwrap() }

    pub fn grant_role_payload(&self, params: &RoleTypes) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.0,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("grantRole"),
            ),
            message:      OwnedParameter::from_serial(&params).unwrap(),
        }
    }

    pub fn mint_payload(&self, params: &MintParams) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.0,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("mint"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }

    pub fn grant_role(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &RoleTypes,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.grant_role_payload(params),
        )
    }

    pub fn mint(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &MintParams,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.mint_payload(params),
        )
    }
}

pub fn deploy_module(chain: &mut Chain, sender: &Account) -> ModuleDeploySuccess {
    WasmModule::from_slice(MODULE_BYTES).unwrap();
    chain
        .module_deploy_v1(
            Signer::with_one_key(),
            sender.address,
            WasmModule::from_slice(MODULE_BYTES).unwrap(),
        )
        .expect("deploying module")
}

pub fn init(
    chain: &mut Chain,
    sender: &Account,
) -> std::result::Result<(ContractInitSuccess, ModuleReference, OwnedContractName), ContractInitError>
{
    let module_ref = WasmModule::from_slice(MODULE_BYTES)
        .unwrap()
        .get_module_ref();
    let contract_name = CONTRACT_NAME.to_owned();
    let res = chain.contract_init(
        Signer::with_one_key(),
        sender.address,
        MAX_ENERGY,
        InitContractPayload {
            amount:    Amount::zero(),
            init_name: contract_name.clone(),
            mod_ref:   module_ref,
            param:     OwnedParameter::empty(),
        },
    )?;

    Ok((res, module_ref, contract_name))
}
