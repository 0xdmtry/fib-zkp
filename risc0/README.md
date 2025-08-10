# Risc0 Fibonacci ZKP

This project is for demonstration of a complete, two-party zero-knowledge proof application built with the Risc0 zkVM. It shows how a **Prover** can execute a program and generate a cryptographic proof, and how a completely separate **Verifier** can check that proof to confirm the result without re-running the computation.

## About The Project

The project proves the calculation of the 20th Fibonacci number. It is structured to model a real-world scenario where two parties interact:

  * **The Guest Program**: A simple Rust function that calculates a Fibonacci number. This is the logic that runs inside the zkVM and gets proven.
  * **The Prover**: A Rust application that executes the guest, generates the ZK-STARK proof, and saves it to a file (`receipt.bin`).
  * **The Verifier**: A separate Rust application that reads `receipt.bin`, verifies the cryptographic proof, and confirms the public output.

This demonstrates the core value of ZKPs: proving a computation was done correctly without trusting the party who ran it.

-----

## Project Structure

The project is a Cargo workspace containing the three main components:

```
r0-fibonacci-zkp/
├── prover/         # The Prover application
├── verifier/       # The Verifier application
└── methods/        # Contains the Guest program
    └── guest/
```

-----

## Setup

Before running the project, you need to set up the Risc0 toolchain.

1.  **Install `rzup`**: This is the Risc0 toolchain manager.

    ```bash
    cargo install rzup
    ```

2.  **Install Risc0 Toolchain**: Use `rzup` to install the necessary compilers for the zkVM.

    ```bash
    rzup install
    ```

-----

## How to Run

The workflow involves two distinct steps, run from the project's root directory.

#### Step 1: Run the Prover

This command compiles and runs the prover, which executes the Fibonacci guest code and generates the proof file in the `artifacts/` directory.

```bash
cargo run --release --bin prover
```

**Expected Prover Output:**

```
   Compiling ...
    Finished release [optimized + debuginfo] target(s) in ...
     Running `target/release/prover`
Prover: Setting up the executor environment...
Prover: Running the prover to execute guest code and generate proof...
Prover: ✅ Proof generated successfully.
```

#### Step 2: Run the Verifier

This command runs the separate verifier application. It reads the `receipt.bin` file and verifies the proof.

```bash
cargo run --bin verifier
```

**Expected Verifier Output:**

```
   Compiling ...
    Finished dev [optimized] target(s) in ...
     Running `target/debug/verifier`
Verifier: Verifying the receipt in memory...
Verifier: ✅ Cryptographic verification successful.
Verifier: The guest code has proven that the 20th Fibonacci number is: 6765
```

-----

## What This Demonstrates

This project successfully showcases a complete ZKP lifecycle:

  * A **Prover** performs a computation and seals the result and proof in a `Receipt`.
  * The `Receipt` is the only artifact transferred to the **Verifier**.
  * The **Verifier**, without re-running the expensive computation, can mathematically confirm the integrity of the proof and trust the public result.
