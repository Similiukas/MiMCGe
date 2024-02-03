use std::time::Instant;
use g2p::GaloisField;
use crate::mimc::mimc::MiMC;
use crate::mimc_general::mimc_general::MiMCGn;
use crate::tests::tests::{test_confusion, test_diffusion};
use crate::utils::helpers::{add_finite_field, Cipher, CipherType, generate_random_bits, multiply_finite_field, power_finite_field, square_multiply, to_decimal};

mod utils;
mod mimc;
mod mimc_lib;
mod tests;
mod aes;
mod mimc_general;

fn test_addition() {
    let block_size = 5;
    let a = generate_random_bits(block_size);
    let b = generate_random_bits(block_size);

    let c = add_finite_field(&a, &b);

    println!("{:?} {:?} {:?}", a, b, c);
    println!("{} {} {}\n\n", to_decimal(&a), to_decimal(&b), to_decimal(&c));
}

fn test_mul() {
    let block_size = 61;
    let a = generate_random_bits(block_size);
    let b = generate_random_bits(block_size);

    let c = multiply_finite_field(&a, &b, block_size);
    let start = Instant::now();
    let d = power_finite_field(&c, 2049, block_size);
    println!("{:.2?}", start.elapsed());

    println!("{:?} {:?} {:?} {:?}", a, b, c, d);
    println!("{} {} {} {}\n", to_decimal(&a), to_decimal(&b), to_decimal(&c), to_decimal(&d));


    let start1 = Instant::now();
    let d1 = square_multiply(&c, 2049, block_size);
    println!("{:.2?}", start1.elapsed());

    println!("{:?}", d1);
    println!("{}", to_decimal(&d1));
}

fn test_mimc() {
    let block_size = 127;
    // let k = MiMCGn::new(17, block_size);
    let k = MiMC::new(block_size);
    let start = Instant::now();
    println!("{k}");
    let key = generate_random_bits(block_size);
    println!("Key {} {:?}", to_decimal(&key), key);
    let plaintext = generate_random_bits(block_size);
    println!("Original:   {} {:?}", to_decimal(&plaintext), plaintext);
    let ciphertext = k.encrypt(&plaintext, &key);
    println!("Ciphertext: {} {:?}", to_decimal(&ciphertext), ciphertext);
    let p_again = k.decrypt(&ciphertext, &key);
    println!("Plaintext:  {} {:?}", to_decimal(&p_again), p_again);
    println!("Time {:.2?}", start.elapsed());
}

fn main() {
    // test_addition();
    // test_mul();
    test_mimc();
    // test_diffusion(1000, 31, CipherType::MiMCGn);
    // test_confusion(1000, 128);
}
