#![allow(unused)]

use concordium_base::smart_contracts::WasmModule;
use concordium_protocols::concordium_cis2_security::AgentWithRoles;
use concordium_smart_contract_testing::*;
use concordium_std::ContractName;
use security_p2p_trading::{
    AgentRole, CancelSellParams, Deposit, ForceCancelSellParams, InitParam, SellPositionOfParams,
    TransferExchangeParams, TransferSellParams,
};

use super::MAX_ENERGY;
use crate::contract_base::{ContractPayloads, ContractTestClient};

const MODULE_BYTES: &[u8] = include_bytes!("../../security-p2p-trading/contract.wasm.v1");
const CONTRACT_NAME: ContractName = ContractName::new_unchecked("init_security_p2p_trading");

pub struct P2PTradeTestClient(pub ContractAddress);
impl ContractTestClient<InitParam> for P2PTradeTestClient {
    fn new(contract_address: ContractAddress) -> Self { Self(contract_address) }
}
impl ContractPayloads<InitParam> for P2PTradeTestClient {
    fn module() -> WasmModule { WasmModule::from_slice(MODULE_BYTES).unwrap() }

    fn contract_name() -> OwnedContractName { CONTRACT_NAME.to_owned() }

    fn contract_address(&self) -> ContractAddress { self.0 }
}
impl P2PTradingClientPayloads for P2PTradeTestClient {}

pub fn deploy_module(chain: &mut Chain, sender: &Account) -> ModuleDeploySuccess {
    let module = WasmModule::from_slice(MODULE_BYTES).unwrap();
    chain
        .module_deploy_v1(Signer::with_one_key(), sender.address, module)
        .expect("deploying module")
}

pub trait P2PTradingClientPayloads: ContractPayloads<InitParam> {
    fn transfer_sell_payload(&self, params: &TransferSellParams) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("transferSell"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }

    fn cancel_sell_payload(&self, params: &CancelSellParams) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("cancelSell"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }

    fn transfer_exchange_payload(&self, params: &TransferExchangeParams) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("transferExchange"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }

    fn add_agent_payload(&self, params: &AgentWithRoles<AgentRole>) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("addAgent"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }

    fn remove_agent_payload(&self, params: &Address) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("removeAgent"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }

    fn add_market_payload(&self, params: &ContractAddress) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("addMarket"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }

    fn remove_market_payload(&self, params: &ContractAddress) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("removeMarket"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }

    fn market_in_use_payload(&self, params: &ContractAddress) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("marketInUse"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }

    fn sell_position_of_payload(&self, params: &SellPositionOfParams) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.contract_address(),
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                Self::contract_name().as_contract_name(),
                EntrypointName::new_unchecked("sellPositionOf"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }
}

pub trait P2PTradingClientResponses {
    fn parse_sell_position_of(&self) -> Deposit;
}
impl P2PTradingClientResponses for ContractInvokeSuccess {
    fn parse_sell_position_of(&self) -> Deposit {
        self.parse_return_value().expect("parsing deposit")
    }
}

impl P2PTradeTestClient {
    pub fn transfer_sell(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &TransferSellParams,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.transfer_sell_payload(params),
        )
    }

    pub fn cancel_sell(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &CancelSellParams,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.cancel_sell_payload(params),
        )
    }

    pub fn transfer_exchange(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &TransferExchangeParams,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.transfer_exchange_payload(params),
        )
    }

    pub fn add_agent(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &AgentWithRoles<AgentRole>,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.add_agent_payload(params),
        )
    }

    pub fn remove_agent(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &Address,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.remove_agent_payload(params),
        )
    }

    pub fn add_market(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &ContractAddress,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.add_market_payload(params),
        )
    }

    pub fn remove_market(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &ContractAddress,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.remove_market_payload(params),
        )
    }

    pub fn market_in_use(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &ContractAddress,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.market_in_use_payload(params),
        )
    }

    pub fn sell_position_of(
        &self,
        chain: &mut Chain,
        sender: &Account,
        params: &SellPositionOfParams,
    ) -> Result<ContractInvokeSuccess, ContractInvokeError> {
        chain.contract_update(
            Signer::with_one_key(),
            sender.address,
            sender.address.into(),
            MAX_ENERGY,
            self.sell_position_of_payload(params),
        )
    }
}
