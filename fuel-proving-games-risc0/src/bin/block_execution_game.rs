//! An end-to-end example of using the RISC Zero ZKVM to generate and verify a proof of execution
//! of the FuelVM.
//!
//! This program starts a node with a transaction, serializes the input, and passes it to the ZKVM.
//! It then verifies the generated proof to ensure correctness.
//!
//! You can run this script using the following command:
//! ```shell
//! RISC0_DEV_MODE=1 RUST_LOG=info cargo run --release --bin fuel-block-execution-game-risc0 -- --help
//! ```
//!
//! The `RISC0_DEV_MODE=1` flag enables development mode, and `RUST_LOG=info` configures logging
//! for better visibility.
use clap::{Parser, Subcommand};
use fuel_proving_games_risc0::block_execution_game::defaults;
use fuel_zkvm_primitives_test_fixtures::block_execution_fixtures::fixtures::Fixture;

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
#[clap(
    name = "command",
    about = "The command to execute",
    rename_all = "snake_case"
)]
enum Command {
    ExecuteFixture {
        #[arg(value_enum)]
        fixture: Fixture,
    },
    ProveFixture {
        #[arg(value_enum)]
        fixture: Fixture,
    },
}

fn main() -> fuel_proving_games_risc0::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    match args.command {
        Command::ExecuteFixture { fixture } => {
            tracing::info!("Executing the fixture");

            // Get a GameExecutor with the default executor
            let executor = defaults::game_executor();

            // Execute the fixture
            let report = executor.execute_fixture(fixture)?;
            tracing::info!("fixture executed successfully.");

            // Record the number of cycles executed.
            tracing::info!("Number of cycles: {}", report.cycles());
        }
        Command::ProveFixture { fixture } => {
            tracing::info!("Proving and verifying the fixture");

            // Get a GameProver with the default prover
            let prover = defaults::game_prover();

            // Prove the fixture
            let prove_info = prover.prove_fixture(fixture)?;

            // Verify the receipt
            prover.verify_receipt(&prove_info.receipt)?;

            tracing::info!("Fixture proved and verified successfully.");
        }
    }

    Ok(())
}
