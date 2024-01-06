use crate::mimc::mimc::MiMC;
use crate::utils::helpers::{add_field_elements_over_finite_field, generate_random_bits, multiply_finite_field, power_finite_field, to_decimal};

mod utils;
mod mimc;

fn test_addition() {
    let block_size = 5;
    let a = generate_random_bits(block_size);
    let b = generate_random_bits(block_size);

    let c = add_field_elements_over_finite_field(&a, &b);

    println!("{:?} {:?} {:?}", a, b, c);
    println!("{} {} {}", to_decimal(&a), to_decimal(&b), to_decimal(&c));

    // println!();
    // println!();
    // println!();
    //
    // let c2 = add_field_elements_over_prime_field(&a, &b, 2_i32.pow(block_size) as u32, block_size);
    //
    // println!("{:?} {:?} {:?}", a, b, c2);
    // println!("{} {} {}", to_decimal(&a), to_decimal(&b), to_decimal(&c2));
}

fn test_mul() {
    let block_size = 5;
    let a = generate_random_bits(block_size);
    // let a = vec![1, 1, 1, 1];
    // let b = vec![1, 0, 1, 1];
    let b = generate_random_bits(block_size);

    let c = multiply_finite_field(&a, &b);

    // 3 3 5 8 -> 3 3 5 7
    let d = power_finite_field(&c, 2);

    println!("{:?} {:?} {:?} {:?}", a, b, c, d);
    println!("{} {} {} {}", to_decimal(&a), to_decimal(&b), to_decimal(&c), to_decimal(&d));
}

fn test_mimc() {
    let field = 5;
    let k = MiMC::new(field);
    println!("{}", k);
    let plaintext = generate_random_bits(field);
    println!("Original:   {} {:?}", to_decimal(&plaintext), plaintext);
    let ciphertext = k.encrypt(&plaintext);
    println!("Ciphertext: {} {:?}", to_decimal(&ciphertext), ciphertext);
    let again = k.decrypt(&ciphertext);
    println!("Decrypted:  {} {:?}", to_decimal(&again), again);
}

fn main() {
    // test_addition();
    // test_mul();
    test_mimc();
}
