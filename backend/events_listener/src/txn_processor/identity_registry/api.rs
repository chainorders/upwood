use concordium_rust_sdk::types::{Address, ContractAddress};
use shared::api::ApiResult;
use shared::db::DbPool;
use poem::web::Data;
use poem_openapi::param::Path;
use poem_openapi::payload::Json;
use poem_openapi::OpenApi;

use super::db;

pub struct Api;

#[OpenApi]
impl Api {
    #[oai(
        path = "/identity_registry/:contract/identity/:address",
        method = "get"
    )]
    pub async fn is_registered(
        &self,
        Path(contract): Path<String>,
        Path(address): Path<String>,
        Data(pool): Data<&DbPool>,
    ) -> ApiResult<bool> {
        let cis2_address: ContractAddress = contract.parse()?;
        let address: Address = address.parse()?;
        let mut conn = pool.get()?;
        let res = db::find_identity(&mut conn, &cis2_address, &address)?;
        ApiResult::Ok(Json(res.is_some()))
    }
}
