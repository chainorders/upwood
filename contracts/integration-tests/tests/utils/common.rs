use super::{
    compliance::{
        ComplianceContract, ComplianceInitParams, ComplianceModule, ComplianceModuleContract,
        ComplianceModuleInitParams, IComplianceModule,
    },
    consts::{
        COMPLIANCE_MODULE, IDENTITY_REGISTRY_MODULE, SECURITY_NFT_MODULE, SECURITY_SFT_MODULE,
    },
    identity_registry::{
        IIdentityRegistryModule, IdentityRegistryContract, IdentityRegistryModule,
    },
    security_nft::{ISecurityNftModule, SecurityNftContract, SecurityNftModule},
    security_sft::{ISecuritySftModule, SecuritySftContract, SecuritySftModule},
    test_contract_client::{ITestContract, ITestModule},
};
use concordium_smart_contract_testing::*;

pub fn init_identity_contracts(
    chain: &mut Chain,
    admin: &Account,
    nationalities: Vec<String>,
) -> (IdentityRegistryContract, ComplianceContract) {
    let ir_module = IdentityRegistryModule {
        module_path: IDENTITY_REGISTRY_MODULE.to_owned(),
    };
    ir_module.deploy(chain, admin).expect("Deploying identity registry module");
    let identity_registry = ir_module
        .rwa_identity_registry()
        .init(chain, admin, &())
        .map(|r| IdentityRegistryContract(r.contract_address))
        .expect("Initializing identity registry module");

    let compliance_module = ComplianceModule {
        module_path: COMPLIANCE_MODULE.to_owned(),
    };
    compliance_module.deploy(chain, admin).expect("Deploying compliance module");
    let compliance_module_contract = compliance_module
        .rwa_compliance_module_allowed_nationalities()
        .init(chain, admin, &ComplianceModuleInitParams {
            identity_registry: identity_registry.contract_address(),
            nationalities,
        })
        .map(|r| ComplianceModuleContract(r.contract_address))
        .expect("Initializing compliance module");
    let compliance_contract = compliance_module
        .rwa_compliance()
        .init(chain, admin, &ComplianceInitParams {
            modules: vec![compliance_module_contract.contract_address()],
        })
        .map(|r| ComplianceContract(r.contract_address))
        .expect("Initializing compliance contract");

    (identity_registry, compliance_contract)
}

pub fn init_security_token_contracts(
    chain: &mut Chain,
    admin: &Account,
    identity_registry: &IdentityRegistryContract,
    compliance_contract: &ComplianceContract,
    sponsors: Vec<ContractAddress>,
) -> Result<(SecurityNftContract, SecuritySftContract), ContractInvokeError> {
    let nft_contract = init_security_nft_contract(
        chain,
        admin,
        identity_registry,
        compliance_contract,
        sponsors.clone(),
    );

    let sft_contract =
        init_security_sft_contract(chain, admin, identity_registry, compliance_contract, sponsors);

    Ok((nft_contract, sft_contract))
}

pub fn init_security_sft_contract(
    chain: &mut Chain,
    admin: &Account,
    identity_registry: &IdentityRegistryContract,
    compliance_contract: &ComplianceContract,
    sponsors: Vec<ContractAddress>,
) -> SecuritySftContract {
    let sft_module = SecuritySftModule {
        module_path: SECURITY_SFT_MODULE.to_owned(),
    };
    sft_module.deploy(chain, admin).expect("Deploying security sft module");

    sft_module
        .rwa_security_sft()
        .init(chain, admin, &concordium_rwa_security_sft::types::InitParam {
            identity_registry: identity_registry.contract_address(),
            compliance: compliance_contract.contract_address(),
            sponsors,
        })
        .map(|r| SecuritySftContract(r.contract_address))
        .expect("Initializing security sft module")
}

pub fn init_security_nft_contract(
    chain: &mut Chain,
    admin: &Account,
    identity_registry: &IdentityRegistryContract,
    compliance_contract: &ComplianceContract,
    sponsors: Vec<ContractAddress>,
) -> SecurityNftContract {
    let nft_module = SecurityNftModule {
        module_path: SECURITY_NFT_MODULE.to_owned(),
    };
    nft_module.deploy(chain, admin).expect("Deploying security nft module");

    nft_module
        .rwa_security_nft()
        .init(chain, admin, &concordium_rwa_security_nft::types::InitParam {
            identity_registry: identity_registry.contract_address(),
            compliance: compliance_contract.contract_address(),
            sponsors,
        })
        .map(|r| SecurityNftContract(r.contract_address))
        .expect("Initializing security nft module")
}
