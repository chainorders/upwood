#![allow(unused)]

use concordium_base::smart_contracts::WasmModule;
use concordium_smart_contract_testing::*;
use security_mint_fund::types::{
    AddFundParams, ClaimInvestmentParams, FundId, FundState, InitParam, TransferInvestParams,
    UpdateFundStateParams,
};

use super::MAX_ENERGY;
use crate::contract_base::{ContractPayloads, ContractTestClient};

const MODULE_BYTES: &[u8] = include_bytes!("../../security-mint-fund/contract.wasm.v1");
const CONTRACT_NAME: ContractName = ContractName::new_unchecked("init_security_mint_fund");

pub struct MintFundTestClient(pub ContractAddress);

impl ContractPayloads<InitParam> for MintFundTestClient {
    fn module() -> WasmModule { WasmModule::from_slice(MODULE_BYTES).unwrap() }

    fn contract_name() -> OwnedContractName { CONTRACT_NAME.to_owned() }

    fn contract_address(&self) -> ContractAddress { self.0 }
}

impl ContractTestClient<InitParam> for MintFundTestClient {
    fn new(contract_address: ContractAddress) -> Self { Self(contract_address) }
}

impl MintFundTestClient {
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

    pub fn update_fund_state_payload(
        &self,
        params: &UpdateFundStateParams,
    ) -> UpdateContractPayload {
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

    pub fn claim_investment_payload(
        &self,
        params: &ClaimInvestmentParams,
    ) -> UpdateContractPayload {
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

    pub fn add_fund_payload(&self, params: &AddFundParams) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.0,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("addFund"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }

    pub fn remove_fund_payload(&self, fund_id: FundId) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.0,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                CONTRACT_NAME,
                EntrypointName::new_unchecked("removeFund"),
            ),
            message:      OwnedParameter::from_serial(&fund_id).unwrap(),
        }
    }
}

impl MintFundTestClient {
    pub fn transfer_invest(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &TransferInvestParams,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.transfer_invest_payload(params),
        )
    }

    pub fn update_fund_state(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &UpdateFundStateParams,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.update_fund_state_payload(params),
        )
    }

    pub fn claim_investment(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &ClaimInvestmentParams,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.claim_investment_payload(params),
        )
    }

    pub fn add_fund(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &AddFundParams,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.add_fund_payload(params),
        )
    }

    pub fn remove_fund(
        &self,
        chain: &mut Chain,
        sender: &Account,
        fund_id: FundId,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.remove_fund_payload(fund_id),
        )
    }
}

pub fn deploy_module(chain: &mut Chain, sender: &Account) -> ModuleDeploySuccess {
    let module = WasmModule::from_slice(MODULE_BYTES).unwrap();
    chain
        .module_deploy_v1(Signer::with_one_key(), sender.address, module)
        .expect("deploying module")
}
