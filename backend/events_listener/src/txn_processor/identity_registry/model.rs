use std::collections::BTreeMap;

use concordium_rust_sdk::types::Address;
use shared::db::{DbConn, DbResult};

use super::db;

#[derive(Debug, Clone)]
pub struct IdentityRegistry(pub concordium_rust_sdk::types::ContractAddress);
impl IdentityRegistry {
    pub fn new(address: concordium_rust_sdk::types::ContractAddress) -> Self { Self(address) }

    pub fn identity_exists(&self, conn: &mut DbConn, address: &Address) -> DbResult<bool> {
        db::identity_exists(conn, &self.0, address)
    }

    pub fn identity_exists_batch(
        &self,
        conn: &mut DbConn,
        addresses: &[Address],
    ) -> DbResult<BTreeMap<Address, bool>> {
        let existing = db::identity_exists_batch(conn, &self.0, addresses)?;
        let res = addresses
            .iter()
            .map(|a| (*a, existing.contains(a)))
            .collect::<BTreeMap<_, _>>();
        Ok(res)
    }
}
