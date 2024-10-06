use concordium_base::smart_contracts::WasmModule;
use concordium_smart_contract_testing::*;

use super::MAX_ENERGY;
const MODULE_BYTES: &[u8] = include_bytes!("../../compliance/contract.wasm.v1");

pub fn deploy_module(chain: &mut Chain, sender: &Account) -> ModuleDeploySuccess {
    let module = WasmModule::from_slice(MODULE_BYTES).unwrap();
    chain
        .module_deploy_v1(Signer::with_one_key(), sender.address, module)
        .expect("deploying module")
}

pub fn init(
    chain: &mut Chain,
    sender: &Account,
    compliance_modules: Vec<ContractAddress>,
) -> ContractInitSuccess {
    let module = WasmModule::from_slice(MODULE_BYTES).unwrap();
    chain
        .contract_init(
            Signer::with_one_key(),
            sender.address,
            MAX_ENERGY,
            InitContractPayload {
                amount:    Amount::zero(),
                init_name: OwnedContractName::new_unchecked("init_rwa_compliance".to_string()),
                mod_ref:   module.get_module_ref(),
                param:     OwnedParameter::from_serial(
                    &concordium_rwa_compliance::compliance::init::InitParams {
                        modules: compliance_modules,
                    },
                )
                .unwrap(),
            },
        )
        .expect("init")
}

pub fn init_all(
    chain: &mut Chain,
    sender: &Account,
    identity_registry: ContractAddress,
    nationalities: &[&str],
) -> ContractInitSuccess {
    let compliance_module = nationalities_module::init(
        chain,
        sender,
        &concordium_rwa_compliance::compliance_modules::allowed_nationalities::init::InitParams {
            nationalities: nationalities.iter().map(|n| n.to_string()).collect(),
            identity_registry,
        },
    )
    .contract_address;
    init(chain, sender, vec![compliance_module])
}

pub mod nationalities_module {
    use concordium_base::smart_contracts::WasmModule;
    use concordium_rwa_compliance::compliance_modules::allowed_nationalities::init::InitParams;
    use concordium_smart_contract_testing::*;

    use super::MODULE_BYTES;
    use crate::MAX_ENERGY;

    pub fn init(chain: &mut Chain, sender: &Account, param: &InitParams) -> ContractInitSuccess {
        let module = WasmModule::from_slice(MODULE_BYTES).unwrap();
        chain
            .contract_init(
                Signer::with_one_key(),
                sender.address,
                MAX_ENERGY,
                InitContractPayload {
                    amount:    Amount::zero(),
                    init_name: OwnedContractName::new_unchecked(
                        "init_rwa_compliance_module_allowed_nationalities".to_string(),
                    ),
                    mod_ref:   module.get_module_ref(),
                    param:     OwnedParameter::from_serial(param).unwrap(),
                },
            )
            .expect("init")
    }
}
