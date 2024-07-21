pub mod migrations {
    use clap::Parser;
    use diesel::{Connection, PgConnection};
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use log::{debug, info};
    //TODO: Make the migrations path relative to the current directory somehow
    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/txn_listener/migrations");

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
}
pub mod listener_config {
    use crate::txn_listener::schema::{self, listener_config::dsl::*};
    use bigdecimal::BigDecimal;
    use concordium_rust_sdk::{types::AbsoluteBlockHeight, v2::FinalizedBlockInfo};
    use diesel::{
        dsl::*,
        prelude::*,
        r2d2::{ConnectionManager, PooledConnection},
    };
    use num_traits::ToPrimitive;

    type Conn = PooledConnection<ConnectionManager<PgConnection>>;

    #[derive(Selectable, Queryable, Identifiable)]
    #[diesel(table_name = schema::listener_config)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct ListenerConfig {
        pub id:                i32,
        pub last_block_height: BigDecimal,
        pub last_block_hash:   Vec<u8>,
    }

    #[derive(Insertable)]
    #[diesel(table_name = schema::listener_config)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct ListenerConfigInsert {
        pub last_block_height: BigDecimal,
        pub last_block_hash:   Vec<u8>,
    }

    /// Retrieves the last processed block from the database.
    pub async fn get_last_processed_block(
        conn: &mut Conn,
    ) -> anyhow::Result<Option<AbsoluteBlockHeight>> {
        let config = listener_config
            .order(last_block_height.desc())
            .limit(1)
            .select(last_block_height)
            .first(conn)
            .optional()?
            .map(|block_height: BigDecimal| AbsoluteBlockHeight {
                height: block_height.to_u64().expect("Block height should convert to u64"),
            });

        Ok(config)
    }

    /// Updates the last processed block in the database.
    pub async fn update_last_processed_block(
        conn: &mut Conn,
        block: &FinalizedBlockInfo,
    ) -> anyhow::Result<i32> {
        let created_id: i32 = insert_into(listener_config)
            .values(ListenerConfigInsert {
                last_block_hash:   block.block_hash.bytes.to_vec(),
                last_block_height: block.height.height.into(),
            })
            .returning(id)
            .get_result(conn)?;

        Ok(created_id)
    }
}

pub mod listener_contracts {
    use bigdecimal::BigDecimal;
    use concordium_rust_sdk::{
        base::{hashes::ModuleReference, smart_contracts::OwnedContractName},
        types::ContractAddress,
    };
    use diesel::{
        dsl::*,
        prelude::*,
        r2d2::{ConnectionManager, PooledConnection},
    };

    use crate::txn_listener::schema::{self, listener_contracts::dsl::*};
    type Conn = PooledConnection<ConnectionManager<PgConnection>>;

    #[derive(Selectable, Queryable, Identifiable, Insertable)]
    #[diesel(primary_key(index))]
    #[diesel(table_name = schema::listener_contracts)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct ListenerContract {
        pub module_ref:    Vec<u8>,
        pub contract_name: String,
        pub index:         BigDecimal,
        pub sub_index:     BigDecimal,
    }

    /// Adds a contract to the database.
    pub async fn add_contract(
        conn: &mut Conn,
        address: &concordium_rust_sdk::types::ContractAddress,
        origin_ref: &ModuleReference,
        init_name: &OwnedContractName,
    ) -> anyhow::Result<()> {
        insert_into(listener_contracts)
            .values(ListenerContract {
                index:         address.index.into(),
                sub_index:     address.subindex.into(),
                contract_name: init_name.to_string(),
                module_ref:    origin_ref.bytes.to_vec(),
            })
            .execute(conn)?;

        Ok(())
    }

    /// Finds a contract in the database based on its address.
    pub async fn find_contract(
        conn: &mut Conn,
        contract_address: &ContractAddress,
    ) -> anyhow::Result<Option<(ModuleReference, OwnedContractName)>> {
        let contract = listener_contracts
            .filter(index.eq::<BigDecimal>(contract_address.index.into()))
            .select((module_ref, contract_name))
            .get_result(conn)
            .optional()?
            .map(|c: (Vec<u8>, String)| {
                (to_module_ref(c.0), OwnedContractName::new_unchecked(c.1))
            });

        Ok(contract)
    }

    fn to_module_ref(vec: Vec<u8>) -> ModuleReference {
        ModuleReference::new(vec.as_slice().try_into().expect("Should convert vec to module ref"))
    }
}
