use events_listener::txn_listener::db::ListenerContract;
use events_listener::txn_processor::cis2_utils::ContractAddressToDecimal;
use poem::web::Data;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::OpenApi;
use shared::db::DbPool;

use super::*;

pub struct AdminApi;

#[OpenApi]
impl AdminApi {
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
        Data(identity_registry): Data<&IdentityRegistryContractAddress>,
    ) -> JsonResult<ListenerContract> {
        ensure_is_admin(&claims)?;
        let mut db_conn = db_pool.get()?;
        let contract = ListenerContract::find(&mut db_conn, identity_registry.0.to_decimal())?
            .ok_or(Error::NotFound(PlainText("Contract not found".to_string())))?;
        Ok(Json(contract))
    }
}
