use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::security_token_contract_type::SecurityTokenContractType;
use crate::db;
use crate::db_shared::DbConn;

#[derive(Object, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ForestProjectMarketTrader {
    pub trader:              db::security_p2p_trading::Trader,
    pub market_type:         SecurityTokenContractType,
    pub forest_project_id:   Uuid,
    pub forest_project_name: String,
    pub cognito_user_id:     String,
    pub email:               String,
}

impl ForestProjectMarketTrader {
    #[allow(clippy::too_many_arguments)]
    pub fn list(
        conn: &mut DbConn,
        market_contract_addr: Decimal,
        project_id: Option<Uuid>,
        currency: Option<(Decimal, Decimal)>,
        token_id_filter: Option<Decimal>,
        token_contract_addr: Option<Decimal>,
        page: i64,
        page_size: i64,
    ) -> QueryResult<(Vec<Self>, i64)> {
        use crate::schema::{
            forest_project_token_contracts, forest_projects, security_p2p_trading_traders, users,
        };

        let query = security_p2p_trading_traders::table
            .inner_join(
                forest_project_token_contracts::table
                    .on(security_p2p_trading_traders::token_contract_address
                        .eq(forest_project_token_contracts::contract_address)),
            )
            .inner_join(
                forest_projects::table
                    .on(forest_project_token_contracts::forest_project_id.eq(forest_projects::id)),
            )
            .inner_join(
                users::table.on(security_p2p_trading_traders::trader.eq(users::account_address)),
            );

        let mut count_query = query.into_boxed();
        let mut query = query.into_boxed();

        query =
            query.filter(security_p2p_trading_traders::contract_address.eq(market_contract_addr));
        count_query = count_query
            .filter(security_p2p_trading_traders::contract_address.eq(market_contract_addr));

        if let Some(project_id) = project_id {
            query = query.filter(forest_projects::id.eq(project_id));
            count_query = count_query.filter(forest_projects::id.eq(project_id));
        }

        if let Some((currency_token_id, currency_contract_address)) = currency {
            query = query
                .filter(security_p2p_trading_traders::currency_token_id.eq(currency_token_id))
                .filter(
                    security_p2p_trading_traders::currency_token_contract_address
                        .eq(currency_contract_address),
                );
            count_query = count_query
                .filter(
                    security_p2p_trading_traders::currency_token_contract_address
                        .eq(currency_contract_address),
                )
                .filter(security_p2p_trading_traders::currency_token_id.eq(currency_token_id));
        }

        if let Some(token_id) = token_id_filter {
            query = query.filter(security_p2p_trading_traders::token_id.eq(token_id));
            count_query = count_query.filter(security_p2p_trading_traders::token_id.eq(token_id));
        }

        if let Some(token_contract) = token_contract_addr {
            query = query
                .filter(security_p2p_trading_traders::token_contract_address.eq(token_contract));
            count_query = count_query
                .filter(security_p2p_trading_traders::token_contract_address.eq(token_contract));
        }

        let total_count = count_query.count().get_result::<i64>(conn)?;
        let page_count = (total_count as f64 / page_size as f64).ceil() as i64;

        let records = query
            .select((
                security_p2p_trading_traders::all_columns,
                forest_projects::id,
                forest_projects::name,
                forest_project_token_contracts::contract_type,
                users::cognito_user_id,
                users::email,
            ))
            .limit(page_size)
            .offset(page * page_size)
            .load::<(
                db::security_p2p_trading::Trader,
                Uuid,
                String,
                SecurityTokenContractType,
                String,
                String,
            )>(conn)?
            .into_iter()
            .map(
                |(
                    trader,
                    forest_project_id,
                    forest_project_name,
                    contract_type,
                    user_id,
                    email,
                )| Self {
                    trader,
                    market_type: contract_type,
                    forest_project_id,
                    forest_project_name,
                    cognito_user_id: user_id,
                    email,
                },
            )
            .collect::<Vec<Self>>();

        Ok((records, page_count))
    }
}
