use events_listener::txn_processor::cis2_security::security_sft_single;
use events_listener::txn_processor::cis2_utils::ContractAddressToDecimal;
use poem::web::Data;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::OpenApi;
use shared::db_shared::DbPool;

use super::*;

pub struct AdminApi;

#[OpenApi]
impl AdminApi {
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
        Data(carbon_credit_contract): Data<&CarbonCreditContractAddress>,
    ) -> JsonResult<security_sft_single::TokenDetails> {
        ensure_is_admin(&claims)?;
        let token = security_sft_single::TokenDetails::find(
            carbon_credit_contract.0.to_decimal(),
            &mut db_pool.get()?,
        )?
        .ok_or(Error::NotFound(PlainText("Token not found".to_string())))?;
        Ok(Json(token))
    }
}
