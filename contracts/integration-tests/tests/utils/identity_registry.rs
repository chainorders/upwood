use concordium_rwa_identity_registry::{identities::RegisterIdentityParams, types::Identity};
use concordium_rwa_utils::common_types::IdentityAttribute;
use concordium_smart_contract_testing::*;
use concordium_std::ExpectReport;

use super::consts::*;

pub fn identity_registry_deploy_and_init(
    chain: &mut Chain,
    owner: AccountAddress,
) -> ContractAddress {
    let ir_module = chain
        .module_deploy_v1(
            Signer::with_one_key(),
            owner,
            module_load_v1(IDENTITY_REGISTRY_MODULE).unwrap(),
        )
        .expect_report("Identity Registry: Deploy")
        .module_reference;

    chain
        .contract_init(
            Signer::with_one_key(),
            owner,
            Energy::from(30000),
            InitContractPayload {
                mod_ref: ir_module,
                amount: Amount::zero(),
                init_name: OwnedContractName::new_unchecked(IR_CONTRACT_NAME.to_owned()),
                param: OwnedParameter::empty(),
            },
        )
        .expect_report("Identity Registry: Init")
        .contract_address
}

pub fn add_identities(
    chain: &mut Chain,
    ir_contract: ContractAddress,
    ir_agent: AccountAddress,
    address_identity: Vec<(Address, String)>,
) -> Result<(), ContractInvokeError> {
    for identity in address_identity {
        add_identity_nationality(chain, ir_contract, ir_agent, identity.0, &identity.1)?;
    }

    Ok(())
}

fn add_identity_nationality(
    chain: &mut Chain,
    ir_contract: ContractAddress,
    ir_agent: AccountAddress,
    identity_address: Address,
    identity_nationality: &str,
) -> Result<ContractInvokeSuccess, ContractInvokeError> {
    chain.contract_update(
        Signer::with_one_key(),
        ir_agent,
        Address::Account(ir_agent),
        Energy::from(10000),
        UpdateContractPayload {
            amount: Amount::zero(),
            receive_name: OwnedReceiveName::new_unchecked(
                "rwa_identity_registry.registerIdentity".to_string(),
            ),
            address: ir_contract,
            message: OwnedParameter::from_serial(&RegisterIdentityParams {
                identity: Identity {
                    attributes: vec![IdentityAttribute {
                        tag: NATIONALITY_ATTRIBUTE_TAG,
                        value: identity_nationality.to_owned(),
                    }],
                    credentials: vec![],
                },
                address: identity_address,
            })
            .unwrap(),
        },
    )
}
