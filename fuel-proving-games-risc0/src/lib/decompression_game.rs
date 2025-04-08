use crate::common::{GameConfig, GameExecutor, GameProver};
use crate::elf::{FUEL_DECOMPRESSION_GAME_RISC0_ELF, FUEL_DECOMPRESSION_GAME_RISC0_ID};
use fuel_zkvm_primitives_test_fixtures::decompression_fixtures::Fixture;
use risc0_zkvm::{ProveInfo, SessionInfo};
use std::rc::Rc;

/// Configuration for the Decompression Game
#[derive(Debug, Clone)]
pub struct DecompressionGame;

impl GameConfig for DecompressionGame {
    type Fixture = Fixture;

    fn elf() -> &'static [u8] {
        FUEL_DECOMPRESSION_GAME_RISC0_ELF
    }

    fn id() -> &'static [u32; 8] {
        &FUEL_DECOMPRESSION_GAME_RISC0_ID
    }

    fn get_fixture_input(fixture: &Self::Fixture) -> impl serde::Serialize {
        Fixture::get_input_for_fixture(fixture)
    }
}

/// Type alias for Decompression Game Prover
pub type DecompressionProver<P> = GameProver<P, DecompressionGame>;

/// Type alias for Decompression Game Executor
pub type DecompressionExecutor<E> = GameExecutor<E, DecompressionGame>;

/// Convenience functions for working with the default prover and executor
pub mod defaults {
    use super::*;

    /// Get a DecompressionProver with the default RISC-0 prover
    pub fn game_prover() -> DecompressionProver<Rc<dyn risc0_zkvm::Prover>> {
        DecompressionProver::new(risc0_zkvm::default_prover())
    }

    /// Get a DecompressionExecutor with the default RISC-0 executor
    pub fn game_executor() -> DecompressionExecutor<Rc<dyn risc0_zkvm::Executor>> {
        DecompressionExecutor::new(risc0_zkvm::default_executor())
    }

    /// Prove a fixture with the default prover
    pub fn prove_fixture(fixture: Fixture) -> crate::Result<ProveInfo> {
        game_prover().prove_fixture(fixture)
    }

    /// Execute a fixture with the default executor
    pub fn execute_fixture(fixture: Fixture) -> crate::Result<SessionInfo> {
        game_executor().execute_fixture(fixture)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::create_csv_writer;
    use serde::Serialize;

    #[derive(Serialize)]
    struct ExecutionReport {
        fixture: Fixture,
        cycle_count: u64,
    }

    #[derive(Serialize)]
    struct ProvingReport {
        fixture: Fixture,
        proving_time: u128,
        verification_time: u128,
    }

    #[test]
    fn run_all_fixtures_and_collect_report() {
        let fixtures = fuel_zkvm_primitives_test_fixtures::decompression_fixtures::all_fixtures();
        let mut wtr = create_csv_writer("FUEL_RISC0_REPORT", "fuel_risc0_report.csv");

        // Create a reusable executor
        let executor = defaults::game_executor();

        for fixture in fixtures {
            // Execute the fixture with the new API
            let executor_info = executor.execute_fixture(fixture.clone()).unwrap();

            let report = ExecutionReport {
                fixture: fixture.clone(),
                cycle_count: executor_info.cycles(),
            };

            wtr.serialize(report).expect("Couldn't write report to CSV");
            wtr.flush().expect("Couldn't flush CSV writer");
        }
    }

    #[test]
    fn prove_all_fixtures_and_collect_report() {
        let fixtures = fuel_zkvm_primitives_test_fixtures::decompression_fixtures::all_fixtures();
        let mut wtr = create_csv_writer("FUEL_RISC0_REPORT", "fuel_risc0_report.csv");

        // Create a reusable prover
        let prover = defaults::game_prover();

        for fixture in fixtures {
            // Prove the fixture with the new API
            let start_time = std::time::Instant::now();
            let prove_info = prover.prove_fixture(fixture.clone()).unwrap();
            let proving_time = start_time.elapsed().as_millis();

            let start_time = std::time::Instant::now();
            prover
                .verify_receipt(&prove_info.receipt)
                .expect("Proof verification failed.");
            let verification_time = start_time.elapsed().as_millis();

            let report = ProvingReport {
                fixture: fixture.clone(),
                proving_time,
                verification_time,
            };

            wtr.serialize(report).expect("Couldn't write report to CSV");
            wtr.flush().expect("Couldn't flush CSV writer");
        }
    }
}
