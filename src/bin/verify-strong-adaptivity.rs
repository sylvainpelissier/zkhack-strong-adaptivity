#![allow(unused, unreachable_code)]
use std::str::FromStr;

use ark_ed_on_bls12_381::Fr;
use ark_ff::Field;
use strong_adaptivity::{Instance, Proof, Witness, prove, data::puzzle_data};
use strong_adaptivity::verify;
use strong_adaptivity::PUZZLE_DESCRIPTION;
use prompt::{puzzle, welcome};
use rand::SeedableRng;
use rand_chacha::ChaCha20Rng;

fn main() {
    let rng = &mut ChaCha20Rng::from_seed([
        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        1, 1,
    ]);
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let ck = puzzle_data();

    let a = Fr::from(1);
    let (comm_1, r_1) = ck.commit_with_rng(a, rng);
    let (comm_2, r_2) = ck.commit_with_rng(a, rng);
    let instance = Instance { comm_1, comm_2 };
    let witness = Witness { a, r_1, r_2 };
    let proof = prove(&ck, &instance, &witness, rng);
    
    let a_1 = a;
    let r_1 = witness.r_1;
    let r_2 = witness.r_2;

    // Compute a_2
    let a_2 = Fr::from_str("4002217735577189182531079544363823770694568796438165289072703799982355110675").unwrap();

    assert!(verify(&ck, &instance, &proof));
    // Check that commitments are correct
    assert_eq!(ck.commit_with_explicit_randomness(a_1, r_1), instance.comm_1);
    assert_eq!(ck.commit_with_explicit_randomness(a_2, r_2), instance.comm_2);
    // Check that messages are unequal
    assert_ne!(a_1, a_2);
}