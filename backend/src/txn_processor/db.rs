use concordium_rust_sdk::types::{smart_contracts::OwnedContractName, ContractAddress};

pub trait IDb {
    fn client(&self) -> &mongodb::Client;
    fn contract_name(&self) -> &OwnedContractName;
    fn database(&self, contract: &ContractAddress) -> mongodb::Database {
        self.client().database(&format!(
            "{}-{}-{}",
            self.contract_name(),
            contract.index,
            contract.subindex
        ))
    }
}

#[derive(Clone)]
pub struct ContractDb {
    pub client:        mongodb::Client,
    pub contract_name: OwnedContractName,
}
impl IDb for ContractDb {
    fn client(&self) -> &mongodb::Client { &self.client }

    fn contract_name(&self) -> &OwnedContractName { &self.contract_name }
}
