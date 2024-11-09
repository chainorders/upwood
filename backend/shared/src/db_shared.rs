use diesel::PgConnection;

pub type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;
pub type DbConn = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>;
pub type DbResult<T> = Result<T, diesel::result::Error>;
