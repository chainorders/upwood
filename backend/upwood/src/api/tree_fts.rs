use events_listener::txn_processor::cis2_security::security_sft_single;
use events_listener::txn_processor::cis2_utils::ContractAddressToDecimal;
use poem::web::Data;
use poem_openapi::payload::{Json, PlainText};
use poem_openapi::OpenApi;
use shared::db::DbPool;

use super::*;

pub struct AdminApi;

#[OpenApi]
impl AdminApi {
    /// Retrieves the details of the token associated with the TreeFT contract.
    ///
    /// This function is an administrative endpoint that requires the caller to be an admin.
    /// It retrieves the token details from the database using the provided `DbPool` and `TreeFTContractAddress`.
    ///
    /// # Arguments
    /// - `db_pool`: A reference to the database connection pool.
    /// - `claims`: The bearer authorization claims of the caller.
    /// - `carbon_credit_contract`: A reference to the TreeFT contract address.
    ///
    /// # Returns
    /// A `JsonResult` containing the `TokenDetails` of the token associated with the TreeFT contract.
    #[oai(
        path = "/admin/tree_fts/contract",
        method = "get",
        tag = "ApiTags::TreeFT"
    )]
    pub async fn contract_get(
        &self,
        Data(db_pool): Data<&DbPool>,
        BearerAuthorization(claims): BearerAuthorization,
        Data(tree_ft_contract): Data<&TreeFTContractAddress>,
    ) -> JsonResult<security_sft_single::TokenDetails> {
        ensure_is_admin(&claims)?;
        let token = security_sft_single::TokenDetails::find(
            tree_ft_contract.0.to_decimal(),
            &mut db_pool.get()?,
        )?
        .ok_or(Error::NotFound(PlainText("Token not found".to_string())))?;
        Ok(Json(token))
    }
}
