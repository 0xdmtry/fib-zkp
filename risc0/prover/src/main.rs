use methods::{METHOD_ELF, METHOD_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};
use std::fs;

fn main() {
    let input: u64 = 20;

    println!("Setting up the executor enviroment...");

    let env = ExecutorEnv::builder()
                .write(&input)
                .unwrap()
                .build()
                .unwrap();


    let prover = default_prover();

    println!("Running the prover to execute guest code and generate proof...");

    let prove_info = prover.prove(env, METHOD_ELF).unwrap();
    let receipt = prove_info.receipt;
    let serialized_receipt = bincode::serialize(&receipt).unwrap();

    fs::create_dir_all("./artifacts").unwrap();
    fs::write("./artifacts/receipt.bin", serialized_receipt).unwrap();

    println!("Proof generated and saved to ./artifacts/receipt.bin");
    println!("Guest Code ID: {:?}", METHOD_ID);
}
