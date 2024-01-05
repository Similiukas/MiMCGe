use crate::feistel_mimc::feistel_mimc::FeistelMiMC;
use crate::mimc::mimc::MiMC;
use crate::utils::helpers::{add_field_elements_over_finite_field, add_field_elements_over_prime_field, FieldElement, generate_random_bits, generate_random_element, multiply_finite_field, to_binary, to_decimal};

mod utils;
mod mimc;
mod feistel_mimc;

fn test_addition() {
    let block_size = 4;
    let a = generate_random_bits(block_size);
    let b = generate_random_bits(block_size);

    let c = add_field_elements_over_finite_field(&a, &b);

    println!("{:?} {:?} {:?}", a, b, c);
    println!("{} {} {}", to_decimal(&a), to_decimal(&b), to_decimal(&c));

    println!();
    println!();
    println!();

    let c2 = add_field_elements_over_prime_field(&a, &b, 2_i32.pow(block_size as u32) as u32, block_size);

    println!("{:?} {:?} {:?}", a, b, c2);
    println!("{} {} {}", to_decimal(&a), to_decimal(&b), to_decimal(&c2));
}

fn test_mul() {
    let block_size = 4;
    let a = generate_random_bits(block_size);
    // let a = vec![1, 1, 1, 1];
    // let b = vec![1, 0, 1, 1];
    let b = generate_random_bits(block_size);

    let c = multiply_finite_field(&a, &b);

    println!("{:?} {:?} {:?}", a, b, c);
    println!("{} {} {}", to_decimal(&a), to_decimal(&b), to_decimal(&c));
}

fn test_mimc() {
    let field = 5;
    let k = MiMC::new(field);
    let mut plaintext = generate_random_bits(field as usize);
    println!("{}", k);
    println!("Original:   {} {:?}", to_decimal(&plaintext), plaintext);
    let ciphertext = k.encrypt(&plaintext);
    println!("Ciphertext: {} {:?}", to_decimal(&ciphertext), ciphertext);
    let again = k.decrypt(&ciphertext);
    println!("{:?}", again);
}

fn test_feistel() {
    let field = 5;
    let k = FeistelMiMC::new(field);
    println!("{}", k);
}

fn main() {
    // test_addition();
    test_mul();
    // test_mimc();
    // test_feistel();
}
