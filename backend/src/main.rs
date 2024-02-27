mod shared;
mod sponsor;
mod txn_listener;
mod txn_processor;
mod verifier;
use anyhow::Ok;
use clap::Parser;
use dotenv::dotenv;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
enum Command {
    GenerateContractsApiSpecs(txn_processor::OpenApiConfig),
    ContractsApi(txn_processor::ListenerAndApiConfig),
    GenerateVerifierApiSpecs(verifier::OpenApiConfig),
    VerifierApi(verifier::ApiConfig),
    GenerateSponsorApiSpecs(sponsor::OpenApiConfig),
    SponsorApi(sponsor::ApiConfig),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    match Command::parse() {
        Command::GenerateContractsApiSpecs(config) => {
            txn_processor::generate_api_client(config).await?
        }
        Command::ContractsApi(config) => txn_processor::run_api_server_and_listener(config).await?,
        Command::VerifierApi(config) => verifier::run_api_server(config).await?,
        Command::GenerateVerifierApiSpecs(config) => verifier::generate_api_client(config).await?,
        Command::SponsorApi(config) => sponsor::run_api_server(config).await?,
        Command::GenerateSponsorApiSpecs(config) => sponsor::generate_api_client(config).await?,
    }
    Ok(())
}
