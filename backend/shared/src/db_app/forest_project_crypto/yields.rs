use diesel::dsl::sum;
use diesel::prelude::*;
use poem_openapi::Object;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db_shared::DbConn;

#[derive(Object, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct UserYieldsAggregate {
    pub yield_token_id:         Decimal,
    pub yield_contract_address: Decimal,
    pub yield_amount:           Decimal,
}

impl UserYieldsAggregate {
    pub fn list(
        conn: &mut DbConn,
        yielder_contract_address_: Decimal,
        account: &str,
    ) -> QueryResult<Vec<Self>> {
        use crate::schema_manual::holder_yields::dsl::*;

        let ret = holder_yields
            .filter(yielder_contract_address.eq(yielder_contract_address_))
            .filter(holder_address.eq(account))
            .group_by((yield_token_id, yield_contract_address))
            .select((
                yield_token_id,
                yield_contract_address,
                sum(yield_value).assume_not_null(),
            ))
            .load::<(Decimal, Decimal, Decimal)>(conn)?
            .into_iter()
            .map(
                |(yield_token_id_, yield_contract_address_, yield_amount)| UserYieldsAggregate {
                    yield_token_id: yield_token_id_,
                    yield_contract_address: yield_contract_address_,
                    yield_amount,
                },
            )
            .collect::<Vec<Self>>();
        Ok(ret)
    }
}

#[derive(Object, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct YieldClaim {
    pub token_contract_address: Decimal,
    pub token_id:               Decimal,
    pub token_balance:          Decimal,
    pub max_token_id:           Decimal,
}

impl YieldClaim {
    pub fn list(
        conn: &mut DbConn,
        yielder_contract_address_: Decimal,
        account: &str,
    ) -> QueryResult<Vec<Self>> {
        use crate::schema_manual::holder_yields::dsl::*;

        let ret = holder_yields
            .filter(yielder_contract_address.eq(yielder_contract_address_))
            .filter(holder_address.eq(account))
            .order_by((cis2_address.asc(), token_id.asc(), token_ver_to.desc()))
            .distinct_on((cis2_address, token_id))
            .load::<HolderYield>(conn)?
            .into_iter()
            .map(|holder_yield| YieldClaim {
                token_contract_address: holder_yield.cis2_address,
                token_id:               holder_yield.token_id,
                token_balance:          holder_yield.un_frozen_balance,
                max_token_id:           holder_yield.token_ver_to,
            })
            .collect::<Vec<Self>>();
        Ok(ret)
    }
}

#[derive(Object, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ForestProjectTokenContractUserYields {
    pub forest_project_id:      Uuid,
    pub token_contract_address: Decimal,
    pub yield_token_id:         Decimal,
    pub yield_contract_address: Decimal,
    pub yield_amount:           Decimal,
}

impl ForestProjectTokenContractUserYields {
    pub fn list_by_forest_project_ids(
        conn: &mut DbConn,
        yielder_contract_address_: Decimal,
        account: &str,
        forest_project_ids: &[Uuid],
    ) -> QueryResult<Vec<Self>> {
        use crate::schema_manual::holder_yields::dsl::*;

        let ret = holder_yields
            .filter(yielder_contract_address.eq(yielder_contract_address_))
            .filter(holder_address.eq(account))
            .filter(forest_project_id.eq_any(forest_project_ids))
            .group_by((
                forest_project_id,
                cis2_address,
                yield_token_id,
                yield_contract_address,
            ))
            .select((
                forest_project_id,
                cis2_address,
                yield_token_id,
                yield_contract_address,
                sum(yield_value).assume_not_null(),
            ))
            .order_by((
                forest_project_id.asc(),
                cis2_address.asc(),
                yield_token_id.asc(),
                yield_contract_address.asc(),
            ))
            .load::<(Uuid, Decimal, Decimal, Decimal, Decimal)>(conn)?
            .into_iter()
            .map(
                |(
                    forest_project_id_,
                    token_contract_address_,
                    yield_token_id_,
                    yield_contract_address_,
                    yield_amount,
                )| {
                    ForestProjectTokenContractUserYields {
                        forest_project_id: forest_project_id_,
                        token_contract_address: token_contract_address_,
                        yield_token_id: yield_token_id_,
                        yield_contract_address: yield_contract_address_,
                        yield_amount,
                    }
                },
            )
            .collect::<Vec<Self>>();

        Ok(ret)
    }
}

#[derive(
    Object, Selectable, Queryable, Identifiable, Debug, PartialEq, Serialize, Deserialize, Clone,
)]
#[diesel(table_name = crate::schema_manual::holder_yields)]
#[diesel(primary_key(
    yielder_contract_address,
    holder_address,
    token_id,
    yield_contract_address,
    yield_token_id
))]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct HolderYield {
    pub yielder_contract_address: Decimal,
    pub holder_address:           String,
    pub un_frozen_balance:        Decimal,
    pub forest_project_id:        Uuid,
    pub cis2_address:             Decimal,
    pub token_id:                 Decimal,
    pub token_ver_to:             Decimal,
    pub previous_yield_token_id:  Option<Decimal>,
    pub yield_contract_address:   Decimal,
    pub yield_token_id:           Decimal,
    pub yield_rate_numerator:     Decimal,
    pub yield_rate_denominator:   Decimal,
    pub yield_type:               String,
    pub yield_period:             Decimal,
    pub yield_value:              Decimal,
}
