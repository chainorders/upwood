use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db_shared::DbConn;

#[derive(Object, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ForestProjectTokenUserYieldClaim {
    pub forest_project_id:        Uuid,
    pub token_id:                 Decimal,
    pub token_contract_address:   Decimal,
    pub holder_address:           String,
    pub token_balance:            Decimal,
    pub cognito_user_id:          String,
    pub yielder_contract_address: Decimal,
    pub max_token_id:             Decimal,
}

impl ForestProjectTokenUserYieldClaim {
    pub fn list(
        conn: &mut DbConn,
        user_id: &str,
        yielder_address: Decimal,
        page: i64,
        page_size: i64,
    ) -> QueryResult<Vec<Self>> {
        use crate::schema_manual::forest_project_token_user_yields::dsl::*;
        let res = forest_project_token_user_yields
            .filter(cognito_user_id.eq(user_id))
            .filter(yielder_contract_address.eq(yielder_address))
            .select((
                forest_project_id,
                token_id,
                token_contract_address,
                holder_address,
                token_balance,
                cognito_user_id,
                yielder_contract_address,
                max_token_id,
            ))
            .distinct_on((
                forest_project_id,
                token_id,
                token_contract_address,
                holder_address,
                token_balance,
                cognito_user_id,
                yielder_contract_address,
                max_token_id,
            ))
            .limit(page_size)
            .offset(page * page_size)
            .load::<(
                Uuid,
                Decimal,
                Decimal,
                String,
                Decimal,
                String,
                Decimal,
                Decimal,
            )>(conn)?
            .into_iter()
            .map(
                |(
                    forest_project_id_,
                    token_id_,
                    token_contract_address_,
                    holder_address_,
                    token_balance_,
                    cognito_user_id_,
                    yielder_contract_address_,
                    max_token_id_,
                )| ForestProjectTokenUserYieldClaim {
                    forest_project_id:        forest_project_id_,
                    token_id:                 token_id_,
                    token_contract_address:   token_contract_address_,
                    holder_address:           holder_address_,
                    token_balance:            token_balance_,
                    cognito_user_id:          cognito_user_id_,
                    yielder_contract_address: yielder_contract_address_,
                    max_token_id:             max_token_id_,
                },
            )
            .collect::<Vec<_>>();
        Ok(res)
    }
}
