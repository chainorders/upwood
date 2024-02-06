use concordium_rust_sdk::types::ContractAddress;

use crate::txn_processor::db::{Collection, DbAddress, DbContractAddress, IDb};

pub trait IContractDb: IDb {
    fn identities(&self, contract: &ContractAddress) -> Collection<DbAddress> {
        self.database(contract).collection::<DbAddress>("identities").into()
    }

    fn issuers(&self, contract: &ContractAddress) -> Collection<DbContractAddress> {
        self.database(contract).collection::<DbContractAddress>("issuers").into()
    }

    fn agents(&self, contract: &ContractAddress) -> Collection<DbAddress> {
        self.database(contract).collection::<DbAddress>("agents").into()
    }
}
