use concordium_rust_sdk::cis2;
use diesel::PgConnection;
use num_traits::ToPrimitive;
use rust_decimal::Decimal;

pub type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;
pub type DbConn = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>;
pub type DbResult<T> = Result<T, diesel::result::Error>;

pub fn token_amount_to_sql(amount: &cis2::TokenAmount) -> Decimal {
    Decimal::from_str_radix(amount.0.to_str_radix(10).as_str(), 10)
        .expect("Failed to convert to Decimal")
}

pub fn to_u64(value: Decimal) -> u64 { value.to_u64().expect("Failed to convert to u64") }

#[cfg(test)]
mod tests {
    use concordium_rust_sdk::cis2;
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;
    use rust_decimal::Decimal;

    use super::token_amount_to_sql;

    #[test]
    fn token_amount_conversions() {
        let amount = 1000u64;
        let token_amount = cis2::TokenAmount(BigUint::from_u64(amount).unwrap());
        let token_amount = token_amount_to_sql(&token_amount);
        assert_eq!(token_amount, Decimal::from_u64(amount).unwrap());
    }
}
