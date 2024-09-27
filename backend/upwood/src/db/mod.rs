pub mod users;

use ::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};
use diesel::{Connection, PgConnection};
use tracing::info;

pub type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<PgConnection>>;
pub type DbConn = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<PgConnection>>;
pub type DbResult<T> = Result<T, diesel::result::Error>;

pub fn setup_database(database_url: &str, db_pool_max_size: u32) -> DbPool {
    run_migrations(database_url);
    Pool::builder()
        .max_size(db_pool_max_size)
        .build(ConnectionManager::new(database_url))
        .unwrap()
}

const MIGRATIONS: diesel_migrations::EmbeddedMigrations = diesel_migrations::embed_migrations!();
fn run_migrations(database_url: &str) {
    use diesel_migrations::MigrationHarness;

    info!("Running migrations on database: {}", database_url);
    let mut conn = PgConnection::establish(database_url).expect("Error connecting to Postgres");
    let applied_migrations = conn
        .run_pending_migrations(MIGRATIONS)
        .expect("Error running migrations");
    applied_migrations
        .iter()
        .for_each(|m| info!("Applied migration: {}", m));
}
