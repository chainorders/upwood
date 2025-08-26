use concordium_base::smart_contracts::WasmModule;
use concordium_smart_contract_testing::{
    Account, Chain, ContractInitError, ContractInvokeError, InitContractPayload, Signer,
};
use concordium_std::{Amount, ContractAddress, OwnedContractName, OwnedParameter, Serial};

use crate::MAX_ENERGY;

pub trait ContractPayloads<I: Serial> {
    fn module() -> WasmModule;
    fn contract_name() -> OwnedContractName;
    fn contract_address(&self) -> ContractAddress;
    fn init_payload(init_params: &I) -> InitContractPayload {
        InitContractPayload {
            amount:    Amount::zero(),
            init_name: Self::contract_name(),
            mod_ref:   Self::module().get_module_ref(),
            param:     OwnedParameter::from_serial(init_params).unwrap(),
        }
    }
}

pub trait ContractTestClient<I: Serial>: ContractPayloads<I>
where Self: Sized {
    fn new(contract_address: ContractAddress) -> Self;

    fn init(
        chain: &mut Chain,
        sender: &Account,
        init_params: &I,
    ) -> Result<Self, ContractInitError> {
        chain
            .contract_init(
                Signer::with_one_key(),
                sender.address,
                MAX_ENERGY,
                Self::init_payload(init_params),
            )
            .map(|r| Self::new(r.contract_address))
    }
}

#[derive(Debug)]
pub enum ContractInvokeErrorOrParseError {
    ContractInvokeError(ContractInvokeError),
    ParseError,
}
