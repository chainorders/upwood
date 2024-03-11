use super::test_contract_client::*;
pub use concordium_rwa_compliance::{
    compliance::init::InitParams as ComplianceInitParams,
    compliance_modules::allowed_nationalities::init::InitParams as ComplianceModuleInitParams,
};
use concordium_smart_contract_testing::*;
pub const COMPLIANCE_MODULE_CONTRACT_NAME: &str =
    "init_rwa_compliance_module_allowed_nationalities";
pub const COMPLIANCE_CONTRACT_NAME: &str = "init_rwa_compliance";

pub trait IComplianceModule: ITestModule {
    fn rwa_compliance(&self) -> GenericInit<ComplianceInitParams> {
        GenericInit::<ComplianceInitParams, ()>::new(self.module_ref(), COMPLIANCE_CONTRACT_NAME)
    }

    fn rwa_compliance_module_allowed_nationalities(
        &self,
    ) -> GenericInit<ComplianceModuleInitParams> {
        GenericInit::<ComplianceModuleInitParams, ()>::new(
            self.module_ref(),
            COMPLIANCE_MODULE_CONTRACT_NAME,
        )
    }
}
pub trait IComplianceContract: ITestContract {}
pub trait IComplianceModuleContract: ITestContract {}

pub struct ComplianceModule {
    pub module_path: String,
}

impl ITestModule for ComplianceModule {
    fn module_path(&self) -> String { self.module_path.to_owned() }
}

impl IComplianceModule for ComplianceModule {}

pub struct ComplianceContract(pub ContractAddress);

impl ITestContract for ComplianceContract {
    fn contract_name() -> OwnedContractName {
        OwnedContractName::new_unchecked(COMPLIANCE_CONTRACT_NAME.to_owned())
    }

    fn contract_address(&self) -> ContractAddress { self.0 }
}

impl IComplianceContract for ComplianceContract {}

pub struct ComplianceModuleContract(pub ContractAddress);

impl ITestContract for ComplianceModuleContract {
    fn contract_name() -> OwnedContractName {
        OwnedContractName::new_unchecked(COMPLIANCE_MODULE_CONTRACT_NAME.to_owned())
    }

    fn contract_address(&self) -> ContractAddress { self.0 }
}

impl IComplianceModuleContract for ComplianceModuleContract {}
