use crate::{
    shared::db::{Collection, DbAddress, DbContractAddress},
    txn_processor::db::IDb,
};
use concordium_rust_sdk::types::ContractAddress;

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
