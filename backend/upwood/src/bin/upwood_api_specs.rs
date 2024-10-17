use std::fs;
use std::io::Write;
use std::path::PathBuf;

use serde::Deserialize;
use tracing::info;

#[derive(Deserialize, Debug, Clone)]
/// Configuration struct for OpenAPI.
pub struct Config {
    pub output: PathBuf,
}

#[tokio::main]
async fn main() {
    let config: Config = config::Config::builder()
        .add_source(config::Environment::default())
        .build()
        .expect("Failed to build config")
        .try_deserialize()
        .expect("Failed to deserialize config");

    let api_service = upwood::api::create_service();
    let spec_json = api_service.spec();
    let mut file = fs::File::create(&config.output).expect("Error creating file");
    file.write_all(spec_json.as_bytes())
        .expect("Error writing to file");
    info!("OpenAPI specs generated at {}", config.output.display());
}
