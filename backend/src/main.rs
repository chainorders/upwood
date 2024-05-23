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
/// - `generate-contracts-api-specs`: Generates the OpenAPI specs for the
///   contracts API.
/// - `contracts-api`: Runs the contracts API server and the contracts events
///   processor.
/// - `generate-verifier-api-specs`: Generates the OpenAPI specs for the
///   verifier API.
/// - `verifier-api`: Runs the verifier API server.
/// - `generate-sponsor-api-specs`: Generates the OpenAPI specs for the sponsor
///   API.
/// - `sponsor-api`: Runs the sponsor API server.
///
/// # Returns
///
/// Returns `Ok(())` if the subcommand runs successfully, otherwise returns an
/// `anyhow::Result` with an error.
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    match Command::parse() {
        Command::Listener(config) => txn_processor::run_listener(config).await?,
        Command::ContractsApi(config) => txn_processor::run_api(config).await?,
        Command::GenerateContractsApiSpecs(config) => {
            txn_processor::generate_api_client(config).await?
        }
        Command::VerifierApi(config) => verifier::run_api_server(config).await?,
        Command::GenerateVerifierApiSpecs(config) => verifier::generate_api_client(config).await?,
        Command::SponsorApi(config) => sponsor::run_api_server(config).await?,
        Command::GenerateSponsorApiSpecs(config) => sponsor::generate_api_client(config).await?,
    }
    Ok(())
}
