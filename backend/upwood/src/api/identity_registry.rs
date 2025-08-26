use poem::web::Data;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::OpenApi;
use shared::db::txn_listener::ListenerContract;
use shared::db_shared::DbPool;

use super::*;

pub struct Api;

#[OpenApi]
impl Api {
    /// Retrieves the details of the identity registry contract.
    ///
    /// This function ensures the user is an admin, retrieves the contract details from the database, and returns the contract information as a JSON response.
    ///
    /// # Arguments
    /// * `db_pool` - A reference to the database connection pool.
    /// * `claims` - The bearer authorization claims of the authenticated user.
    /// * `identity_registry` - A reference to the identity registry contract address.
    ///
    /// # Returns
    /// A JSON result containing the `ListenerContract` details.
    #[oai(
        path = "/admin/identity_registry/contract",
        method = "get",
        tag = "ApiTags::IdentityRegistry"
    )]
    pub async fn details(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Data(contracts): Data<&SystemContractsConfig>,
    ) -> JsonResult<ListenerContract> {
        ensure_is_admin(&claims)?;
        let mut db_conn = db_pool.get()?;
        let contract =
            ListenerContract::find(&mut db_conn, contracts.identity_registry_contract_index)?
                .ok_or(Error::NotFound(PlainText("Contract not found".to_string())))?;
        Ok(Json(contract))
    }
}
