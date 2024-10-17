use concordium_rust_sdk::types::Address;
use shared::db::{DbConn, DbResult};

use super::db;

#[derive(Debug, Clone)]
pub struct IdentityRegistry(pub concordium_rust_sdk::types::ContractAddress);
impl IdentityRegistry {
    pub fn new(address: concordium_rust_sdk::types::ContractAddress) -> Self { Self(address) }

    pub fn is_registered(&self, conn: &mut DbConn, address: &Address) -> DbResult<bool> {
        let is_registered = db::find_identity(conn, &self.0, address)?.is_some();
        Ok(is_registered)
    }
}
