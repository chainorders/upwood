use super::test_contract_client::*;
use concordium_rwa_identity_registry::{event::Event, identities::RegisterIdentityParams};
use concordium_smart_contract_testing::*;

pub const NATIONALITY_ATTRIBUTE_TAG: u8 = 5;
pub const CONTRACT_NAME: &str = "init_rwa_identity_registry";

pub trait IIdentityRegistryModule: ITestModule {
    fn rwa_identity_registry(&self) -> GenericInit<(), Event> {
        GenericInit::<(), Event>::new(self.module_ref(), CONTRACT_NAME)
    }
}

pub trait IIdentityRegistryContract: ITestContract {
    fn register_identity(&self) -> GenericReceive<RegisterIdentityParams, (), Event> {
        GenericReceive::<RegisterIdentityParams, (), Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "registerIdentity",
            self.max_energy(),
        )
    }

    fn add_agent(&self) -> GenericReceive<Address, (), Event> {
        GenericReceive::<Address, (), Event>::new(
            self.contract_address(),
            Self::contract_name(),
            "addAgent",
            self.max_energy(),
        )
    }
}

/// methods which have been created for test purposes. They don't actually exist
/// in the contract
pub trait IIdentityRegistryContractExt: IIdentityRegistryContract {}

pub struct IdentityRegistryModule {
    pub module_path: String,
}
impl ITestModule for IdentityRegistryModule {
    fn module_path(&self) -> String { self.module_path.to_owned() }
}
impl IIdentityRegistryModule for IdentityRegistryModule {}

#[derive(Clone)]
pub struct IdentityRegistryContract(pub ContractAddress);
impl ITestContract for IdentityRegistryContract {
    fn contract_address(&self) -> ContractAddress { self.0 }

    fn contract_name() -> OwnedContractName {
        OwnedContractName::new_unchecked(CONTRACT_NAME.to_owned())
    }
}
impl IIdentityRegistryContract for IdentityRegistryContract {}
impl IIdentityRegistryContractExt for IdentityRegistryContract {}
