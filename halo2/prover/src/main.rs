use fib_circuit::FibonacciCircuit;
use halo2_proofs::{
    plonk::{create_proof, keygen_pk, keygen_vk},
    poly::commitment::Params,
    transcript::{Blake2bWrite, Challenge255}
};
use pasta_curves::{group::ff::PrimeField, vesta, Fp};
use rand_core::OsRng;
use std::fs::File;
use std::io::Write;

fn main() {
    // 1. SETUP
    // The size of our circuit, defined as 2^k
    // k=4 means a circuit of max 2^4 = 16 rows
    let k = 4;

    // Generate a temporary, insecure universal setup ("SRS") for development
    // In production, you would use a pre-generated, ceremony-derived SRS
    let params: Params<vesta::Affine> = Params::new(k);

    // Create a new, empty circuit
    let circuit = FibonacciCircuit::default();

    // 2. KEY GENERATION
    // Generate the Verifying Key (VK)
    let vk = keygen_vk(&params, &circuit).expect("keygen_vk should not fail");

    // Generate the Proving Key (PK)
    let pk = keygen_pk(&params, vk.clone(), &circuit).expect("keygen_pk should not fail");

    // 3. PROVE
    // Define the private inputs for our circuit
    let a = Fp::from(1);
    let b = Fp::from(1);

    // The public input is the final value of the sequence we expect
    // After 10 steps starting with a=1, b=1, the final `a` value is 55
    let public_inputs = vec![Fp::from(55)];

    // Create an instance of our circuit with the private inputs
    let proof_circuit = FibonacciCircuit { a, b };

    // Create a transcript for the proof
    let mut transcript = Blake2bWrite::<_, _, Challenge255<_>>::init(vec![]);

    // Create the proof
    create_proof(
        &params,
        &pk,
        &[proof_circuit],
        &[&[&public_inputs]],
        OsRng,
        &mut transcript,
    ).expect("proof generation should not fail");

    let proof = transcript.finalize();

    // 4. SAVE PROOF AND PUBLIC INPUTS
    // Save the proof to a file
    let mut proof_file = File::create("artifacts/proof.bin").unwrap();
    proof_file.write_all(&proof).unwrap();

    // Save the public inputs to file
    let mut public_inputs_file = File::create("artifacts/public_inputs.bin").unwrap();
    for fp in &public_inputs {
        public_inputs_file.write_all(&fp.to_repr()).unwrap();
    }

    println!("Proof and Verifying Key saved successfully.");
}
