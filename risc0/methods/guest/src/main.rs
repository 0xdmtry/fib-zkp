#![no_std]
#![no_main]

use risc0_zkvm::guest::env;
risc0_zkvm::guest::entry!(main);

pub fn main() {

    let input: u64 = env::read();

    let mut result: u64 = 0;
    let mut n: u64 = 1;

    for _ in 0..input {
        let next = result.wrapping_add(n);
        result=n;
        n=next;
    }

    env::commit(&result);
}
