pub mod block_execution_game;
mod common;
pub mod decompression_game;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// This error occurs when the input cannot be deserialized using bincode
    #[error("failed to deserialize input: `{0}`")]
    FailedToDeserializeInput(#[from] bincode::Error),
    /// This error occurs when the input cannot be written to the prover env
    #[error("failed to write input to environment: `{0}`")]
    FailedToWriteInputToProverEnv(anyhow::Error),
    /// This error occurs when the prover env cannot be built
    #[error("failed to build prover environment: `{0}`")]
    FailedToBuildProverEnv(anyhow::Error),
    /// This error occurs when the public outputs from the zkvm cannot be deserialized
    #[error("failed to deserialize public output: `{0}`")]
    FailedToDeserializePublicOutput(anyhow::Error),
    /// This error occurs when the proving game fails to execute
    #[error("failed to execute proving game: `{0}`")]
    FailedToExecuteProvingGame(anyhow::Error),
    /// This error occurs when the proving game fails to prove
    #[error("failed to prove proving game: `{0}`")]
    FailedToProveProvingGame(anyhow::Error),
    /// This error occurs when proof verification fails
    #[error("failed to verify proof: `{0}`")]
    FailedToVerifyProof(anyhow::Error),
    /// This error occurs when a fault/mismatch is detected
    #[error("FAULT: `{0}`")]
    Fault(String),
}

pub type Result<T> = core::result::Result<T, Error>;

pub(crate) mod elf {
    include!(concat!(env!("OUT_DIR"), "/methods.rs"));
}
