use super::test_contract_client::*;
use concordium_rwa_sponsor::{
    types::{PermitMessage, PermitParam},
    utils::NonceParam,
};
use concordium_std::{ContractAddress, OwnedContractName};

pub const MODULE_PATH: &str = "../sponsor/contract.wasm.v1";
pub const CONTRACT_NAME: &str = "init_rwa_sponsor";

pub trait ISponsorModule: ITestModule {
    fn rwa_sponsor(&self) -> GenericInit<(), ()> {
        GenericInit::<(), ()>::new(self.module_ref(), CONTRACT_NAME)
    }
}

impl ITestModule for SponsorModule {
    fn module_path(&self) -> String { self.module_path.to_owned() }
}

pub trait ISponsorContract: ITestContract {
    fn permit(&self) -> GenericReceive<PermitParam, (), ()> {
        GenericReceive::<PermitParam, (), ()>::new(
            self.contract_address(),
            Self::contract_name(),
            "permit",
            self.max_energy(),
        )
    }

    fn nonce(&self) -> GenericReceive<NonceParam, u64, ()> {
        GenericReceive::<NonceParam, u64, ()>::new(
            self.contract_address(),
            Self::contract_name(),
            "nonce",
            self.max_energy(),
        )
    }

    fn bytes_to_sign(&self) -> GenericReceive<PermitMessage, [u8; 32], ()> {
        GenericReceive::<PermitMessage, [u8; 32], ()>::new(
            self.contract_address(),
            Self::contract_name(),
            "bytesToSign",
            self.max_energy(),
        )
    }
}

pub struct SponsorModule {
    pub module_path: String,
}
impl ISponsorModule for SponsorModule {}

pub struct SponsorContract(pub ContractAddress);
impl ITestContract for SponsorContract {
    fn contract_name() -> OwnedContractName {
        OwnedContractName::new_unchecked(CONTRACT_NAME.to_owned())
    }

    fn contract_address(&self) -> ContractAddress { self.0 }
}
impl ISponsorContract for SponsorContract {}
