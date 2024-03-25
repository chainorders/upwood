use concordium_rust_sdk::types::{smart_contracts::OwnedContractName, ContractAddress};

/// The `IDb` trait represents a database interface.
pub trait IDb {
    /// Returns a reference to the MongoDB client.
    fn client(&self) -> &mongodb::Client;

    /// Returns a reference to the owned contract name.
    fn contract_name(&self) -> &OwnedContractName;

    /// Returns a MongoDB database for the given contract address.
    ///
    /// # Arguments
    ///
    /// * `contract` - The contract address.
    ///
    /// # Returns
    ///
    /// A MongoDB database.
    fn database(&self, contract: &ContractAddress) -> mongodb::Database {
        self.client().database(&format!(
            "{}-{}-{}",
            self.contract_name(),
            contract.index,
            contract.subindex
        ))
    }
}

/// Represents a database connection for a contract.
#[derive(Clone)]
pub struct ContractDb {
    pub client:        mongodb::Client,
    pub contract_name: OwnedContractName,
}

/// Implementation of the `IDb` trait for the `ContractDb` struct.
impl IDb for ContractDb {
    /// Returns a reference to the MongoDB client.
    fn client(&self) -> &mongodb::Client { &self.client }

    /// Returns a reference to the owned contract name.
    fn contract_name(&self) -> &OwnedContractName { &self.contract_name }
}
