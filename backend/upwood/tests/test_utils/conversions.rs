use concordium_smart_contract_testing::{
    ContractInitSuccess, ContractInvokeSuccess, ModuleReference, OwnedContractName,
};
use events_listener::listener::{ContractCall, ContractCallType, ContractCallTypeInit};
use events_listener::processors::cis2_utils::ContractAddressToDecimal;

pub fn to_contract_call_update(res: &ContractInvokeSuccess) -> Vec<ContractCall> {
    ContractCall::parse_effects(res.effective_trace_elements_cloned())
}

pub fn to_contract_call_init(
    res: &ContractInitSuccess,
    module_ref: ModuleReference,
    contract_name: OwnedContractName,
) -> ContractCall {
    ContractCall {
        call_type: ContractCallType::Init(ContractCallTypeInit {
            contract_name,
            module_ref,
            events: res.events.clone(),
        }),
        contract:  res.contract_address.to_decimal(),
    }
}
