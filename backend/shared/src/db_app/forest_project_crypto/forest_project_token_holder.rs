use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

use crate::db_app::forest_project_crypto::prelude::*;
use crate::db_shared::DbConn;

#[derive(Object, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ForestProjectTokenHolder {
    pub cis2_address:        Decimal,
    pub token_id:            Decimal,
    pub holder_address:      String,
    pub frozen_balance:      Decimal,
    pub un_frozen_balance:   Decimal,
    pub forest_project_id:   uuid::Uuid,
    pub forest_project_name: String,
    pub contract_type:       SecurityTokenContractType,
    pub email:               String,
}

impl ForestProjectTokenHolder {
    pub fn list(
        conn: &mut DbConn,
        cis2_address: Option<Decimal>,
        token_id: Option<Decimal>,
        holder_address: Option<String>,
        forest_project_id: Option<uuid::Uuid>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema::{
            cis2_token_holders, forest_project_token_contracts, forest_projects, users,
        };

        let query = cis2_token_holders::table
            .inner_join(
                forest_project_token_contracts::table.on(cis2_token_holders::cis2_address
                    .eq(forest_project_token_contracts::contract_address)),
            )
            .inner_join(
                forest_projects::table
                    .on(forest_project_token_contracts::forest_project_id.eq(forest_projects::id)),
            )
            .inner_join(
                users::table.on(cis2_token_holders::holder_address.eq(users::account_address)),
            );
        let mut count_query = query.into_boxed();
        let mut query = query.into_boxed();

        query = query.filter(
            cis2_token_holders::frozen_balance
                .gt(Decimal::ZERO)
                .or(cis2_token_holders::un_frozen_balance.gt(Decimal::ZERO)),
        );
        count_query = count_query.filter(
            cis2_token_holders::frozen_balance
                .gt(Decimal::ZERO)
                .or(cis2_token_holders::un_frozen_balance.gt(Decimal::ZERO)),
        );

        if let Some(cis2_address) = cis2_address {
            query = query.filter(cis2_token_holders::cis2_address.eq(cis2_address));
            count_query = count_query.filter(cis2_token_holders::cis2_address.eq(cis2_address));
        }
        if let Some(token_id) = token_id {
            query = query.filter(cis2_token_holders::token_id.eq(token_id));
            count_query = count_query.filter(cis2_token_holders::token_id.eq(token_id));
        }
        if let Some(holder_address) = holder_address {
            query = query.filter(cis2_token_holders::holder_address.eq(holder_address.clone()));
            count_query = count_query.filter(cis2_token_holders::holder_address.eq(holder_address));
        }
        if let Some(forest_project_id) = forest_project_id {
            query = query.filter(forest_projects::id.eq(forest_project_id));
            count_query = count_query.filter(forest_projects::id.eq(forest_project_id));
        }

        let results = query
            .select((
                cis2_token_holders::cis2_address,
                cis2_token_holders::token_id,
                cis2_token_holders::holder_address,
                cis2_token_holders::frozen_balance,
                cis2_token_holders::un_frozen_balance,
                forest_projects::id,
                forest_projects::name,
                forest_project_token_contracts::contract_type,
                users::email,
            ))
            .limit(page_size)
            .offset(page * page_size)
            .load(conn)?
            .into_iter()
            .map(
                |(
                    cis2_address,
                    token_id,
                    holder_address,
                    frozen_balance,
                    un_frozen_balance,
                    forest_project_id,
                    forest_project_name,
                    contract_type,
                    email,
                )| {
                    ForestProjectTokenHolder {
                        cis2_address,
                        token_id,
                        holder_address,
                        frozen_balance,
                        un_frozen_balance,
                        forest_project_id,
                        forest_project_name,
                        contract_type,
                        email,
                    }
                },
            )
            .collect::<Vec<Self>>();
        let count = count_query.count().get_result::<i64>(conn)?;
        let page_count = (count as f64 / page_size as f64).ceil() as i64;

        Ok((results, page_count))
    }
}
