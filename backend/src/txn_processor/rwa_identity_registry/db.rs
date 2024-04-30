use crate::shared::db::{Collection, DbAddress, DbContractAddress};
use mongodb::Database;

pub struct RwaIdentityRegistryDb {
    pub identities: Collection<DbAddress>,
    pub issuers:    Collection<DbContractAddress>,
    pub agents:     Collection<DbAddress>,
}

impl RwaIdentityRegistryDb {
    pub fn init(db: Database) -> Self {
        let identities = db.collection::<DbAddress>("identities").into();
        let issuers = db.collection::<DbContractAddress>("issuers").into();
        let agents = db.collection::<DbAddress>("agents").into();

        Self {
            identities,
            issuers,
            agents,
        }
    }
}
