use bigdecimal::BigDecimal;
use concordium_cis2::TokenAmountU64;
use concordium_rust_sdk::cis2;
use diesel::PgConnection;
use num_bigint::BigInt;

pub type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;
pub type DbConn = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>;
pub type DbResult<T> = Result<T, diesel::result::Error>;

pub fn token_amount_to_sql(amount: &cis2::TokenAmount) -> BigDecimal {
    BigDecimal::new(
        BigInt::new(num_bigint::Sign::Plus, amount.0.to_u32_digits()),
        0,
    )
}

pub fn token_amount_u64_to_sql(amount: &TokenAmountU64) -> BigDecimal {
    BigDecimal::new(BigInt::from(amount.0), 0)
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use concordium_rust_sdk::cis2;
    use num_bigint::BigUint;
    use num_traits::FromPrimitive;

    use super::token_amount_to_sql;

    #[test]
    fn token_amount_conversions() {
        let amount = 1000u64;
        let token_amount = cis2::TokenAmount(BigUint::from_u64(amount).unwrap());
        let token_amount = token_amount_to_sql(&token_amount);
        assert_eq!(token_amount, BigDecimal::from_u64(amount).unwrap());
    }
}
