use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db_app::forest_project::ForestProjectState;
use crate::db_shared::DbConn;
use crate::schema::{cis2_token_holders, forest_project_token_contracts, forest_projects};

#[derive(Object, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ForestProjectUserBalanceAgg {
    pub forest_project_id: Uuid,
    pub total_balance:     Decimal,
}

impl ForestProjectUserBalanceAgg {
    pub fn list_by_user_id_and_forest_project_ids(
        conn: &mut DbConn,
        account_address: &str,
        project_ids: &[Uuid],
    ) -> QueryResult<Vec<Self>> {
        let ret = forest_projects::table
            .inner_join(
                forest_project_token_contracts::table
                    .on(forest_projects::id.eq(forest_project_token_contracts::forest_project_id)),
            )
            .inner_join(
                cis2_token_holders::table.on(forest_project_token_contracts::contract_address
                    .eq(cis2_token_holders::cis2_address)),
            )
            .filter(cis2_token_holders::holder_address.eq(account_address))
            .filter(forest_projects::id.eq_any(project_ids))
            .group_by(forest_projects::id)
            .select((
                forest_projects::id,
                diesel::dsl::sum(cis2_token_holders::un_frozen_balance).nullable(),
            ))
            .load::<(Uuid, Option<Decimal>)>(conn)?
            .into_iter()
            .map(
                |(forest_project_id, total_balance)| ForestProjectUserBalanceAgg {
                    forest_project_id,
                    total_balance: total_balance.unwrap_or_default(),
                },
            )
            .collect::<Vec<_>>();
        Ok(ret)
    }

    pub fn list_by_user_id(
        conn: &mut DbConn,
        account_address: &str,
        forest_project_states: &[ForestProjectState],
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        let ret = forest_projects::table
            .inner_join(
                forest_project_token_contracts::table
                    .on(forest_projects::id.eq(forest_project_token_contracts::forest_project_id)),
            )
            .inner_join(
                cis2_token_holders::table.on(forest_project_token_contracts::contract_address
                    .eq(cis2_token_holders::cis2_address)),
            )
            .filter(cis2_token_holders::holder_address.eq(account_address))
            .filter(forest_projects::state.eq_any(forest_project_states))
            .group_by(forest_projects::id)
            .select((
                forest_projects::id,
                diesel::dsl::sum(cis2_token_holders::un_frozen_balance).nullable(),
            ))
            .offset(page * page_size)
            .limit(page_size)
            .load::<(Uuid, Option<Decimal>)>(conn)?
            .into_iter()
            .map(
                |(forest_project_id, total_balance)| ForestProjectUserBalanceAgg {
                    forest_project_id,
                    total_balance: total_balance.unwrap_or_default(),
                },
            )
            .collect::<Vec<_>>();

        let count = forest_projects::table
            .inner_join(
                forest_project_token_contracts::table
                    .on(forest_projects::id.eq(forest_project_token_contracts::forest_project_id)),
            )
            .inner_join(
                cis2_token_holders::table.on(forest_project_token_contracts::contract_address
                    .eq(cis2_token_holders::cis2_address)),
            )
            .filter(cis2_token_holders::holder_address.eq(account_address))
            .filter(forest_projects::state.eq_any(forest_project_states))
            .count()
            .get_result::<i64>(conn)?;
        let page_count = count as f64 / page_size as f64;
        let page_count = page_count.ceil() as i64;
        let page_count = if page_count == 0 { 1 } else { page_count };

        Ok((ret, page_count))
    }
}
