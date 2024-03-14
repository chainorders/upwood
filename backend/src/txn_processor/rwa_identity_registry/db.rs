use crate::{
    shared::db::{Collection, DbAddress, DbContractAddress},
    txn_processor::db::IDb,
};
use concordium_rust_sdk::types::ContractAddress;

/// `IRwaIdentityRegistryDb` is a trait that defines the necessary methods for
/// interacting with the RWA Identity Registry database. It extends the `IDb`
/// trait, which provides basic database functionality.
pub trait IRwaIdentityRegistryDb: IDb {
    /// Returns a MongoDB collection of `DbAddress` documents for a given
    /// contract.
    ///
    /// # Arguments
    ///
    /// * `contract` - A reference to the contract's address.
    ///
    /// # Returns
    ///
    /// * A MongoDB collection of `DbAddress` documents.
    fn identities(&self, contract: &ContractAddress) -> Collection<DbAddress> {
        self.database(contract).collection::<DbAddress>("identities").into()
    }

    /// Returns a MongoDB collection of `DbContractAddress` documents for a
    /// given contract.
    ///
    /// # Arguments
    ///
    /// * `contract` - A reference to the contract's address.
    ///
    /// # Returns
    ///
    /// * A MongoDB collection of `DbContractAddress` documents.
    fn issuers(&self, contract: &ContractAddress) -> Collection<DbContractAddress> {
        self.database(contract).collection::<DbContractAddress>("issuers").into()
    }

    /// Returns a MongoDB collection of `DbAddress` documents for a given
    /// contract.
    ///
    /// # Arguments
    ///
    /// * `contract` - A reference to the contract's address.
    ///
    /// # Returns
    ///
    /// * A MongoDB collection of `DbAddress` documents.
    fn agents(&self, contract: &ContractAddress) -> Collection<DbAddress> {
        self.database(contract).collection::<DbAddress>("agents").into()
    }
}
