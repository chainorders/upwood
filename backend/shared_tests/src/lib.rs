use concordium_rust_sdk::base::contracts_common::Serial;
use concordium_rust_sdk::base::smart_contracts::ContractEvent;
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, ImageExt};
use testcontainers_modules::postgres::{self, Postgres};

pub async fn create_new_database_container() -> (String, ContainerAsync<Postgres>) {
    let pg_container = postgres::Postgres::default()
        .with_tag("14-alpine")
        .start()
        .await
        .unwrap();
    let database_url = format!(
        "postgres://postgres:postgres@127.0.0.1:{}/postgres",
        pg_container.get_host_port_ipv4(5432).await.unwrap()
    );

    (database_url, pg_container)
}

pub fn to_contract_event<Event>(e: &Event) -> ContractEvent
where Event: Serial {
    let mut out: Vec<u8> = Vec::new();
    e.serial(&mut out).expect("Error serializing event");
    out.into()
}
