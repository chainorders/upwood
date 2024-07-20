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
    Listener(txn_processor::ListenerConfig),
    ContractsApi(txn_processor::ContractsApiConfig),
    GenerateVerifierApiSpecs(verifier::OpenApiConfig),
    VerifierApi(verifier::ApiConfig),
    GenerateSponsorApiSpecs(sponsor::OpenApiConfig),
    SponsorApi(sponsor::ApiConfig),
}

/// Main entry point for the application.
/// Parses the command line arguments and runs the appropriate subcommand.
///
/// The subcommands are:
/// - `listener`: Runs Indexer / Listener
/// - `contracts-api`: Runs Contracts API server & Contracts events processor
/// - `generate-contracts-api-specs`: Generates OpenAPI specs Contracts API
/// - `generate-verifier-api-specs`: Generates OpenAPI specs for verifier API
/// - `verifier-api`: Runs verifier API server
/// - `generate-sponsor-api-specs`: Generates OpenAPI specs for Sponsor API
/// - `sponsor-api`: Runs the sponsor API server
/// - `generate-sponsor-api-specs`: Generates OpenApi specs for Sponsor API
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    match Command::parse() {
        Command::Listener(config) => txn_processor::run_listener(config).await?,
        Command::ContractsApi(config) => txn_processor::run_api_server(config).await?,
        Command::GenerateContractsApiSpecs(config) => {
            txn_processor::generate_open_api_specs(config).await?
        }
        Command::VerifierApi(config) => verifier::run_api_server(config).await?,
        Command::GenerateVerifierApiSpecs(config) => {
            verifier::generate_open_api_specs(config).await?
        }
        Command::SponsorApi(config) => sponsor::run_api_server(config).await?,
        Command::GenerateSponsorApiSpecs(config) => {
            sponsor::generate_open_api_specs(config).await?
        }
    }
    Ok(())
}
