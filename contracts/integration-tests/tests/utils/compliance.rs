use super::consts::*;
use concordium_rwa_compliance::{
    compliance::init::InitParams as ComplianceInitParams,
    compliance_modules::allowed_nationalities::init::InitParams as ComplianceModuleInitParams,
};
use concordium_smart_contract_testing::*;
use concordium_std::ExpectReport;

pub fn compliance_deploy_and_init(
    chain: &mut Chain,
    ir_contract: ContractAddress,
    owner: AccountAddress,
    nationalities: Vec<String>,
) -> (ContractAddress, ContractAddress) {
    let compliance_module = chain
        .module_deploy_v1(Signer::with_one_key(), owner, module_load_v1(COMPLIANCE_MODULE).unwrap())
        .unwrap()
        .module_reference;
    let compliance_module_contract = chain
        .contract_init(
            Signer::with_one_key(),
            owner,
            Energy::from(30000),
            InitContractPayload {
                mod_ref: compliance_module,
                amount: Amount::zero(),
                init_name: OwnedContractName::new_unchecked(
                    COMPLIANCE_MODULE_CONTRACT_NAME.to_owned(),
                ),
                param: OwnedParameter::from_serial(&ComplianceModuleInitParams {
                    identity_registry: ir_contract,
                    nationalities,
                })
                .unwrap(),
            },
        )
        .expect_report("Compliance Module: Init")
        .contract_address;

    let compliance_contract = chain
        .contract_init(
            Signer::with_one_key(),
            owner,
            Energy::from(30000),
            InitContractPayload {
                mod_ref: compliance_module,
                amount: Amount::zero(),
                init_name: OwnedContractName::new_unchecked(COMPLIANCE_CONTRACT_NAME.to_owned()),
                param: OwnedParameter::from_serial(&ComplianceInitParams {
                    modules: vec![compliance_module_contract],
                })
                .unwrap(),
            },
        )
        .expect_report("Compliance: Init")
        .contract_address;

    (compliance_module_contract, compliance_contract)
}
