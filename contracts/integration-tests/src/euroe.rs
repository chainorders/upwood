#![allow(unused)]

use concordium_base::smart_contracts::WasmModule;
use concordium_cis2::{
    BalanceOfQueryResponse, TokenAmountU64, TokenIdUnit, UpdateOperator, UpdateOperatorParams,
};
use concordium_smart_contract_testing::*;
use concordium_std::{Deserial, SchemaType, Serial, Serialize};

use super::{cis2, MAX_ENERGY};
use crate::cis2_test_client::Cis2TestClient;
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

pub struct EuroETestClient(pub ContractAddress);
impl EuroETestClient {
    pub fn module() -> WasmModule { WasmModule::from_slice(MODULE_BYTES).unwrap() }

    pub fn init_payload() -> InitContractPayload {
        InitContractPayload {
            amount:    Amount::zero(),
            init_name: CONTRACT_NAME.to_owned(),
            mod_ref:   Self::module().get_module_ref(),
            param:     OwnedParameter::empty(),
        }
    }

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

    pub fn cis2(&self) -> Cis2TestClient {
        Cis2TestClient {
            address:       self.0,
            contract_name: CONTRACT_NAME.to_owned(),
        }
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

pub fn grant_role(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    params: &RoleTypes,
) -> Result<ContractInvokeSuccess, ContractInvokeError> {
    chain.contract_update(
        Signer::with_one_key(),
        sender.address,
        sender.address.into(),
        MAX_ENERGY,
        EuroETestClient(contract).grant_role_payload(params),
    )
}

pub fn mint(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    params: &MintParams,
) -> Result<ContractInvokeSuccess, ContractInvokeError> {
    chain.contract_update(
        Signer::with_one_key(),
        sender.address,
        sender.address.into(),
        MAX_ENERGY,
        UpdateContractPayload {
            address:      contract,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("mint"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        },
    )
}

pub fn transfer_single(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: concordium_cis2::Transfer<TokenIdUnit, TokenAmountU64>,
) -> Result<ContractInvokeSuccess, ContractInvokeError> {
    cis2::transfer_single(chain, sender, contract, CONTRACT_NAME, payload)
}

pub fn balance_of(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &concordium_cis2::BalanceOfQueryParams<TokenIdUnit>,
) -> Result<BalanceOfQueryResponse<TokenAmountU64>, ContractInvokeError> {
    cis2::balance_of(chain, sender, contract, CONTRACT_NAME, payload)
}

pub fn balance_of_single(
    chain: &mut Chain,
    invoker: &Account,
    contract: ContractAddress,
    address: Address,
) -> TokenAmountU64 {
    cis2::balance_of_single(
        chain,
        invoker,
        contract,
        TokenIdUnit(),
        address,
        CONTRACT_NAME,
    )
    .expect("euro balance of single")
}

pub fn update_operator(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: &UpdateOperatorParams,
) -> ContractInvokeSuccess {
    cis2::update_operator(chain, sender, contract, CONTRACT_NAME, payload).expect("update operator")
}

pub fn update_operator_single(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    payload: UpdateOperator,
) -> ContractInvokeSuccess {
    update_operator(
        chain,
        sender,
        contract,
        &UpdateOperatorParams(vec![payload]),
    )
}
