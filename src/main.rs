use g2p::GaloisField;
use crate::mimc::mimc::MiMC;
use crate::tests::tests::{test_confusion, test_diffusion};
use crate::utils::helpers::{add_finite_field, Cipher, CipherType, generate_random_bits, multiply_finite_field, power_finite_field, to_decimal};

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
    let block_size = 17;
    let a = generate_random_bits(block_size);
    let b = generate_random_bits(block_size);

    let c = multiply_finite_field(&a, &b, block_size);

    let d = power_finite_field(&c, 2, block_size);

    println!("{:?} {:?} {:?} {:?}", a, b, c, d);
    println!("{} {} {} {}\n", to_decimal(&a), to_decimal(&b), to_decimal(&c), to_decimal(&d));
}

fn test_mimc() {
    let block_size = 127;
    let k = MiMC::new(block_size);
    println!("{k}");
    let key = generate_random_bits(block_size);
    println!("Key {} {:?}", to_decimal(&key), key);
    let plaintext = generate_random_bits(block_size);
    println!("Original:   {} {:?}", to_decimal(&plaintext), plaintext);
    let ciphertext = k.encrypt(&plaintext, &key);
    println!("Ciphertext: {} {:?}", to_decimal(&ciphertext), ciphertext);
    // let again = k.decrypt(&ciphertext, &key);
    // println!("Decrypted:  {} {:?}\n\n", to_decimal(&again), again);

    // let l = MiMCLib::new();
    // println!("{l}");
    // let plaintext: GF = (to_decimal(&generate_random_bits(17))).into();
    // println!("Original:   {:?}", plaintext);
    // let ciphertext = l.encrypt(&plaintext);
    // println!("Ciphertext: {:?}", ciphertext);
    // let p_again = l.decrypt(&ciphertext);
    // println!("Decrypted:  {:?}", p_again);
}

fn main() {
    // test_addition();
    // test_mul();
    // test_mimc();
    test_diffusion(100, 61, CipherType::MiMC);
    // test_confusion(1000, 128);
}
