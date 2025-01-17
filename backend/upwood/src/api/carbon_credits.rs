use poem::web::Data;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::OpenApi;
use shared::db::security_sft_single::SftSingleTokenDetails;
use shared::db_shared::DbPool;

use super::*;

pub struct Api;

#[OpenApi]
impl Api {
    /// Retrieves the details of the carbon credit contract.
    ///
    /// This function is an admin-only endpoint that retrieves the details of the carbon credit contract.
    ///
    /// # Arguments
    /// * `db_pool` - A reference to the database connection pool.
    /// * `claims` - The bearer authorization claims of the authenticated user.
    /// * `carbon_credit_contract` - A reference to the carbon credit contract address.
    ///
    /// # Returns
    /// A JSON result containing the token details of the carbon credit contract.
    #[oai(
        path = "/admin/carbon_credits/contract",
        method = "get",
        tag = "ApiTags::CarbonCredits"
    )]
    pub async fn contract_get(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Data(contracts): Data<&SystemContractsConfig>,
    ) -> JsonResult<SftSingleTokenDetails> {
        ensure_is_admin(&claims)?;
        let token = SftSingleTokenDetails::find(
            contracts.carbon_credit_contract_index,
            &mut db_pool.get()?,
        )?
        .ok_or(Error::NotFound(PlainText("Token not found".to_string())))?;
        Ok(Json(token))
    }
}
