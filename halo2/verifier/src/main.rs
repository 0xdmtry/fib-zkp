use fib_circuit::FibonacciCircuit;
use halo2_proofs::{
    plonk::{keygen_vk, verify_proof, VerifyingKey, SingleVerifier},
    poly::commitment::Params,
    transcript::{Blake2bRead, Challenge255}
};
use pasta_curves::{
    group::ff::PrimeField,
    vesta,
    Fp as PastaFp,
};
use std::{fs::File, io::Read};


fn main() {
    // 1. SETUP
    // The verifier must use the same circuit size parameter k
    let k = 4;
    let params: Params<vesta::Affine> = Params::new(k);
    let circuit = FibonacciCircuit::default();

    // 2. RE-GENERATE VERIFYING KEY
    // The verifier re-generates the VK deterministically from the same parameters
    let vk = keygen_vk(&params, &circuit).expect("keygen_vk shoudlnot fail");

    // 3. LOAD PROOF AND PUBLIC INPUTS
    let mut proof_file = File::open("artifacts/proof.bin").unwrap();
    let mut proof = Vec::new();
    proof_file.read_to_end(&mut proof).unwrap();


    let mut public_inputs_file = File::open("artifacts/public_inputs.bin").unwrap();
    let mut public_inputs_bytes = Vec::new();
    public_inputs_file.read_to_end(&mut public_inputs_bytes).unwrap();

     // Convert the public input bytes back to field elements
    let public_inputs = public_inputs_bytes
    .chunks_exact(32)
    .map(|chunk| {
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(chunk);
        PastaFp::from_repr(bytes).unwrap()
    })
    .collect::<Vec<_>>();

    // 4. VERIFY THE PROOF
    // Create a verification strategy

    let strategy = SingleVerifier::new(&params);
    let mut transcript = Blake2bRead::<_, _, Challenge255<_>>::init(&proof[..]);

    let result = verify_proof(
        &params,
        &vk,
        strategy,
        &[&[&public_inputs]],
        &mut transcript,
    );

    match result {
        Ok(_) => println!("✅ Proof verified successfully!"),
        Err(e) => println!("❌ Proof verification failed: {:?}", e),
    }
}
