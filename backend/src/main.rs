mod txn_listener;
mod txn_processor;
mod verifier;

use anyhow::Ok;
use clap::Parser;
use dotenv::dotenv;
use verifier::{
    generate_verifier_api_frontend_client, run_verifier_api_server, VerifierApiConfig,
    VerifierApiSwaggerConfig,
};

use txn_processor::{
    generate_contracts_api_frontend_client, run_contracts_api_server, ContractsApiSwaggerConfig,
    ContractsListenerAndApiConfig,
};

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
enum Command {
    GenerateContractsApiSpecs(ContractsApiSwaggerConfig),
    ContractsApi(ContractsListenerAndApiConfig),
    VerifierApi(VerifierApiConfig),
    GenerateVerifierApiSpecs(VerifierApiSwaggerConfig),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    match Command::parse() {
        Command::GenerateContractsApiSpecs(config) => {
            generate_contracts_api_frontend_client(config).await?
        }
        Command::ContractsApi(config) => run_contracts_api_server(config).await?,
        Command::VerifierApi(config) => run_verifier_api_server(config).await?,
        Command::GenerateVerifierApiSpecs(config) => {
            generate_verifier_api_frontend_client(config).await?
        }
    }
    Ok(())
}
