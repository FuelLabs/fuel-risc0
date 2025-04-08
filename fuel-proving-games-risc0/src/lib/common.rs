use risc0_zkvm::{ExecutorEnvBuilder, ProveInfo, Prover, Receipt, SessionInfo};
use serde::Serialize;
use std::fmt::Debug;

/// Trait for defining game-specific behavior and constants
pub trait GameConfig: Debug + Clone {
    /// The type of fixture used in this game
    type Fixture: Clone;

    /// Get the RISC-0 ELF for this game
    fn elf() -> &'static [u8];

    /// Get the RISC-0 ID for this game (used for verification)
    fn id() -> &'static [u32; 8];

    /// Get raw input for a specific fixture
    fn get_fixture_input(fixture: &Self::Fixture) -> impl Serialize;
}

/// A generic prover for zkVM games
#[derive(Debug)]
pub struct GameProver<P, G> {
    prover: P,
    _game: std::marker::PhantomData<G>,
}

impl<P, G> GameProver<P, G>
where
    P: AsRef<dyn Prover>,
    G: GameConfig,
{
    /// Create a new GameProver wrapping the given RISC-0 prover
    pub fn new(prover: P) -> Self {
        Self {
            prover,
            _game: std::marker::PhantomData,
        }
    }

    /// Prove the game using raw input bytes
    pub fn prove(&self, input: &[u8]) -> crate::Result<ProveInfo> {
        let mut env = ExecutorEnvBuilder::default();
        env.write_slice(input);
        let env = env.build().map_err(crate::Error::FailedToBuildProverEnv)?;

        self.prover
            .as_ref()
            .prove(env, G::elf())
            .map_err(crate::Error::FailedToProveProvingGame)
    }

    /// Prove the game using a serializable input
    pub fn prove_serializable<T: Serialize>(&self, input: &T) -> crate::Result<ProveInfo> {
        let mut env = ExecutorEnvBuilder::default();
        env.write(input)
            .map_err(crate::Error::FailedToWriteInputToProverEnv)?;

        let env = env.build().map_err(crate::Error::FailedToBuildProverEnv)?;

        self.prover
            .as_ref()
            .prove(env, G::elf())
            .map_err(crate::Error::FailedToProveProvingGame)
    }

    /// Prove using a test fixture
    pub fn prove_fixture(&self, fixture: G::Fixture) -> crate::Result<ProveInfo> {
        let raw_input = G::get_fixture_input(&fixture);
        self.prove_serializable(&raw_input)
    }

    /// Configure a custom environment and then prove with it
    pub fn prove_with_env<F>(&self, f: F) -> crate::Result<ProveInfo>
    where
        F: FnOnce(&mut ExecutorEnvBuilder<'_>) -> crate::Result<()>,
    {
        let mut env = ExecutorEnvBuilder::default();
        f(&mut env)?;

        let env = env.build().map_err(crate::Error::FailedToBuildProverEnv)?;

        self.prover
            .as_ref()
            .prove(env, G::elf())
            .map_err(crate::Error::FailedToProveProvingGame)
    }

    /// Verify a receipt
    pub fn verify_receipt(&self, receipt: &Receipt) -> crate::Result<()> {
        receipt
            .verify(G::id().clone())
            .map_err(|e| crate::Error::FailedToVerifyProof(anyhow::anyhow!(e)))
    }
}

/// A generic executor for zkVM games
#[derive(Debug)]
pub struct GameExecutor<E, G> {
    executor: E,
    _game: std::marker::PhantomData<G>,
}

impl<E, G> GameExecutor<E, G>
where
    E: AsRef<dyn risc0_zkvm::Executor>,
    G: GameConfig,
{
    /// Create a new GameExecutor wrapping the given RISC-0 executor
    pub fn new(executor: E) -> Self {
        Self {
            executor,
            _game: std::marker::PhantomData,
        }
    }

    /// Execute the game using raw input bytes
    pub fn execute(&self, input: &[u8]) -> crate::Result<SessionInfo> {
        let mut env = ExecutorEnvBuilder::default();
        env.write_slice(input);
        let env = env.build().map_err(crate::Error::FailedToBuildProverEnv)?;

        self.executor
            .as_ref()
            .execute(env, G::elf())
            .map_err(crate::Error::FailedToExecuteProvingGame)
    }

    /// Execute the game using a serializable input
    pub fn execute_serializable<T: Serialize>(&self, input: &T) -> crate::Result<SessionInfo> {
        let mut env = ExecutorEnvBuilder::default();
        env.write(input)
            .map_err(crate::Error::FailedToWriteInputToProverEnv)?;

        let env = env.build().map_err(crate::Error::FailedToBuildProverEnv)?;

        self.executor
            .as_ref()
            .execute(env, G::elf())
            .map_err(crate::Error::FailedToExecuteProvingGame)
    }

    /// Execute using a test fixture
    pub fn execute_fixture(&self, fixture: G::Fixture) -> crate::Result<SessionInfo> {
        let raw_input = G::get_fixture_input(&fixture);
        self.execute_serializable(&raw_input)
    }

    /// Configure a custom environment and then execute with it
    pub fn execute_with_env<F>(&self, f: F) -> crate::Result<SessionInfo>
    where
        F: FnOnce(&mut ExecutorEnvBuilder<'_>) -> crate::Result<()>,
    {
        let mut env = ExecutorEnvBuilder::default();
        f(&mut env)?;

        let env = env.build().map_err(crate::Error::FailedToBuildProverEnv)?;

        self.executor
            .as_ref()
            .execute(env, G::elf())
            .map_err(crate::Error::FailedToExecuteProvingGame)
    }
}

/// Helper function to create CSV writer for reports
#[cfg(test)]
pub fn create_csv_writer(
    file_path_env_var: &str,
    default_path: &str,
) -> csv::Writer<std::fs::File> {
    let file_path = std::env::var(file_path_env_var).unwrap_or(default_path.to_string());
    csv::WriterBuilder::new()
        .flexible(true)
        .from_path(file_path)
        .expect("Couldn't create CSV writer")
}
