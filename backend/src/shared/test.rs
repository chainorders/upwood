use concordium_cis2::{TokenIdU64, TokenIdU8, TokenIdVec};
use concordium_rust_sdk::base::{contracts_common::Serial, smart_contracts::ContractEvent};
use diesel::{Connection, PgConnection, RunQueryDsl};

pub struct TestDbContext {
    pub base_url: String,
    pub db_name:  String,
}

impl TestDbContext {
    pub fn new(base_url: &str, db_name: &str) -> Self {
        // First, connect to postgres db to be able to create our test
        // database.
        let postgres_url = format!("{}/postgres", base_url);
        let mut conn =
            PgConnection::establish(&postgres_url).expect("Cannot connect to postgres database.");

        // Create a new database for the test
        let query = diesel::sql_query(format!("CREATE DATABASE {}", db_name).as_str());
        query
            .execute(&mut conn)
            .unwrap_or_else(|_| panic!("Could not create database {}", db_name));

        Self {
            base_url: base_url.to_string(),
            db_name:  db_name.to_string(),
        }
    }
}

impl Drop for TestDbContext {
    fn drop(&mut self) {
        let postgres_url = format!("{}/postgres", self.base_url);
        let mut conn =
            PgConnection::establish(&postgres_url).expect("Cannot connect to postgres database.");

        let disconnect_users = format!(
            "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}';",
            self.db_name
        );

        diesel::sql_query(disconnect_users.as_str()).execute(&mut conn).unwrap();

        let query = diesel::sql_query(format!("DROP DATABASE {}", self.db_name).as_str());
        query.execute(&mut conn).unwrap_or_else(|_| panic!("Couldn't drop database {}", self.db_name));
    }
}

pub fn to_contract_event<Event>(e: &Event) -> ContractEvent
where
    Event: Serial, {
    let mut out: Vec<u8> = Vec::new();
    e.serial(&mut out).expect("Error serializing event");
    out.into()
}

pub fn to_token_id_vec_u8(token_id: TokenIdU8) -> TokenIdVec {
    let mut bytes: Vec<u8> = vec![];
    token_id.serial(&mut bytes).expect("Error converting tokenid u8 to token id vec");
    TokenIdVec(bytes)
}

pub fn to_token_id_vec_u64(token_id: TokenIdU64) -> TokenIdVec {
    let mut bytes: Vec<u8> = vec![];
    token_id.serial(&mut bytes).expect("Error converting tokenid u64 to token id vec");
    TokenIdVec(bytes)
}
