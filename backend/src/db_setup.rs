use clap::Parser;
use diesel::{Connection, PgConnection};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::{debug, info};
//TODO: Make the migrations path relative to the current directory somehow
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Parser, Debug, Clone)]
pub struct MigrationsConfig {
    #[clap(env)]
    pub database_url: String,
}

pub fn run_migrations(config: &MigrationsConfig) -> anyhow::Result<()> {
    let mut conn = PgConnection::establish(&config.database_url)?;
    info!("Transaction Listener Database Migrating...");
    debug!("Running migrations on url: {}", &config.database_url);
    conn.applied_migrations()
        .expect("Error checking for applied migrations")
        .iter()
        .for_each(|m| debug!("Applied Migration: {:?}", m));
    conn.pending_migrations(MIGRATIONS)
        .expect("Error checking for pending migrations")
        .iter()
        .for_each(|m| debug!("Pending Migration: {:?}", m.name().version()));
    conn.run_pending_migrations(MIGRATIONS).expect("Error running migrations");
    info!("Migrations complete");
    Ok(())
}
