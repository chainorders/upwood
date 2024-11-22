use concordium_cis2::UpdateOperatorParams;
use concordium_smart_contract_testing::{
    Amount, ContractAddress, EntrypointName, OwnedContractName, OwnedParameter, OwnedReceiveName,
    UpdateContractPayload,
};

pub struct Cis2TestClient {
    pub address:       ContractAddress,
    pub contract_name: OwnedContractName,
}

impl Cis2TestClient {
    pub fn update_operator_payload(&self, params: &UpdateOperatorParams) -> UpdateContractPayload {
        UpdateContractPayload {
            address:      self.address,
            amount:       Amount::zero(),
            receive_name: OwnedReceiveName::construct_unchecked(
                self.contract_name.as_contract_name(),
                EntrypointName::new_unchecked("updateOperator"),
            ),
            message:      OwnedParameter::from_serial(params).unwrap(),
        }
    }
}
