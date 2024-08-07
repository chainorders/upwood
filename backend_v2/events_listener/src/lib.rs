use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

mod schema;
pub mod txn_listener;
pub mod txn_processor;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
pub fn run_migrations(database_url: &str) -> anyhow::Result<()> {
    PgConnection::establish(database_url)
        .expect("Error connecting to Postgres")
        .run_pending_migrations(MIGRATIONS)
        .expect("Error running migrations");
    Ok(())
}
