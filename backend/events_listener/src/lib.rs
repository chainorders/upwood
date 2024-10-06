mod schema;
pub mod txn_listener;
pub mod txn_processor;

pub mod db_setup {
    use diesel::{Connection, PgConnection};
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use tracing::info;

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();
    pub fn run_migrations(database_url: &str) {
        info!("Running migrations on database: {}", database_url);
        let mut conn = PgConnection::establish(database_url).expect("Error connecting to Postgres");
        let applied_migrations = conn
            .run_pending_migrations(MIGRATIONS)
            .expect("Error running migrations");
        applied_migrations
            .iter()
            .for_each(|m| info!("Applied migration: {}", m));
    }
}
