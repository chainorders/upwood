use concordium_rust_sdk::base::contracts_common::Serial;
use concordium_rust_sdk::base::smart_contracts::ContractEvent;
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, ImageExt};
use testcontainers_modules::postgres::{self, Postgres};

#[derive(Clone)]
pub struct PostgresTestConfig {
    pub postgres_user:     String,
    pub postgres_password: String,
    pub postgres_host:     String,
    pub postgres_port:     u16,
    pub postgres_db:       String,
}

impl PostgresTestConfig {
    pub fn db_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_db
        )
    }
}

pub async fn create_new_database_container() -> (PostgresTestConfig, ContainerAsync<Postgres>) {
    let pg_container = postgres::Postgres::default()
        .with_tag("14-alpine")
        .start()
        .await
        .unwrap();

    let config: PostgresTestConfig = PostgresTestConfig {
        postgres_db:       "postgres".to_string(),
        postgres_host:     "localhost".to_string(),
        postgres_password: "postgres".to_string(),
        postgres_port:     pg_container.get_host_port_ipv4(5432).await.unwrap(),
        postgres_user:     "postgres".to_string(),
    };

    (config, pg_container)
}

pub fn to_contract_event<Event>(e: &Event) -> ContractEvent
where Event: Serial {
    let mut out: Vec<u8> = Vec::new();
    e.serial(&mut out).expect("Error serializing event");
    out.into()
}
