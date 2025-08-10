use methods::METHOD_ID;
use risc0_zkvm::{Receipt};
use std::fs;

fn main() {
    println!("Verifying the proof from ./artifacts/receipt.bin");

    let serialized_receipt = fs::read("./artifacts/receipt.bin").expect("Failed to read receipt. Did you run the prover first?");
    let receipt: Receipt = bincode::deserialize(&serialized_receipt).expect("Failed to deserialize receipt.");

    receipt.verify(METHOD_ID).expect("Proof verification failed.");

    let fib_result: u64 = receipt.journal.decode().expect("Failed to decode journal.");

    println!("Verification successful!");
    println!("The guest code has proven that the 20th Fibonacci number is: {}", fib_result);
}
