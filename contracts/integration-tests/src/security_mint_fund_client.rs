#![allow(unused)]

use concordium_base::smart_contracts::WasmModule;
use concordium_smart_contract_testing::*;
use security_mint_fund::{
    CancelInvestParams, ClaimInvestParams, FundState, InitParam, TransferInvestParams,
};

use super::MAX_ENERGY;

const MODULE_BYTES: &[u8] = include_bytes!("../../security-mint-fund/contract.wasm.v1");
const CONTRACT_NAME: ContractName = ContractName::new_unchecked("init_security_mint_fund");

pub struct MintFundTestClient(pub ContractAddress);
impl MintFundTestClient {
    pub fn module() -> WasmModule { WasmModule::from_slice(MODULE_BYTES).unwrap() }

    pub fn init_payload(init_params: &InitParam) -> InitContractPayload {
        InitContractPayload {
            amount:    Amount::zero(),
            init_name: CONTRACT_NAME.to_owned(),
            mod_ref:   Self::module().get_module_ref(),
            param:     OwnedParameter::from_serial(init_params).unwrap(),
        }
    }

    pub fn transfer_invest_payload(&self, params: &TransferInvestParams) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.0,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("transferInvest"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }

    pub fn update_fund_state_payload(&self, params: &FundState) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.0,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("updateFundState"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }

    pub fn claim_investment_payload(&self, params: &ClaimInvestParams) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.0,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("claimInvestment"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }
}

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
) -> Result<ContractInitSuccess, ContractInitError> {
    let module = WasmModule::from_slice(MODULE_BYTES).unwrap();
    chain.contract_init(
        Signer::with_one_key(),
        sender.address,
        MAX_ENERGY,
        InitContractPayload {
            amount:    Amount::zero(),
            init_name: CONTRACT_NAME.to_owned(),
            mod_ref:   module.get_module_ref(),
            param:     OwnedParameter::from_serial(params).unwrap(),
        },
    )
}

pub fn transfer_invest(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    params: &TransferInvestParams,
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
                EntrypointName::new_unchecked("transferInvest"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        },
    )
}

pub fn cancel_investment(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    params: &CancelInvestParams,
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
                EntrypointName::new_unchecked("cancelInvestment"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        },
    )
}

pub fn update_fund_state(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    params: &FundState,
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
                EntrypointName::new_unchecked("updateFundState"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        },
    )
}

pub fn claim_investment(
    chain: &mut Chain,
    sender: &Account,
    contract: ContractAddress,
    params: &ClaimInvestParams,
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
                EntrypointName::new_unchecked("claimInvestment"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        },
    )
}
