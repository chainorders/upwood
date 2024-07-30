use bigdecimal::BigDecimal;
use concordium_rust_sdk::cis2;
use diesel::PgConnection;
use num_bigint::BigInt;

pub type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;
pub type DbConn = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>;
pub type DbResult<T> = Result<T, diesel::result::Error>;

pub fn token_amount_to_sql(amount: &cis2::TokenAmount) -> BigDecimal {
    BigInt::from_bytes_le(num_bigint::Sign::NoSign, &amount.0.to_bytes_le()).into()
}
