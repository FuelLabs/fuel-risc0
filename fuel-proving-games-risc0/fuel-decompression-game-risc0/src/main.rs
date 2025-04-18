use alloy_sol_types::SolType;
use fuel_zkvm_primitives_prover::games::decompression_game::{prove, PublicValuesStruct};
use risc0_zkvm::guest::env;

fn main() {
    let bytes: Vec<u8> = env::read();
    let proof = prove(&bytes).expect("Proof generation failed");
    let bytes = PublicValuesStruct::abi_encode(&proof);

    env::commit(&bytes);
}
