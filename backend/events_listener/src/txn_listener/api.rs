use bigdecimal::ToPrimitive;
use chrono::{DateTime, Utc};
use poem::web::Data;
use poem_openapi::payload::Json;
use poem_openapi::{Object, OpenApi};
use shared::api::ApiResult;
use shared::db::DbPool;

use super::db;

#[derive(Clone, Copy)]
pub struct Api;

#[OpenApi]
impl Api {
    #[oai(path = "/listener/block/last_processed", method = "get")]
    pub async fn last_processed_block(&self, Data(db_pool): Data<&DbPool>) -> ApiResult<Block> {
        let mut conn = db_pool.get()?;
        let block: Block = db::get_last_processed_block(&mut conn)?
            .ok_or(shared::api::Error::NotFound)?
            .into();
        Ok(Json(block))
    }
}

#[derive(Object)]
pub struct Block {
    pub block_hash_hex:  String,
    pub block_height:    u64,
    pub block_slot_time: DateTime<Utc>,
}

impl From<db::ListenerConfig> for Block {
    fn from(config: db::ListenerConfig) -> Self {
        Self {
            block_hash_hex:  hex::encode(&config.last_block_hash),
            block_height:    config.last_block_height.to_u64().unwrap(),
            block_slot_time: config.last_block_slot_time.and_utc(),
        }
    }
}
