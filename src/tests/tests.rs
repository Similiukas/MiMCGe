use std::time::Instant;
use crate::mimc::mimc::MiMC;
use crate::utils::helpers::{FieldElement, generate_random_bits, generate_round_constants};

/// Diffusion test for cipher. Initializes the cipher with random key, plaintext and produces ciphertext. Then changes
/// one bit of plaintext and checks new ciphertext with original to see how many bits have flipped. Repeats this cycle
/// for every bit in plaintext.
fn diffusion(block_size: u32, round_constants: &Vec<FieldElement>) -> f64 {
    let mut result = 0.0;
    let cipher = MiMC::with_round_constants(block_size, &round_constants);
    let key = generate_random_bits(block_size);
    let plaintext = generate_random_bits(block_size);
    let ciphertext = cipher.encrypt(&plaintext, &key);

    for i in 0..(block_size as usize) {
        let mut new_plaintext = plaintext.to_vec();
        new_plaintext[i] ^= 1; // Flip bit
        let new_ciphertext = cipher.encrypt(&new_plaintext, &key);
        // Count how many bits have flipped
        result += new_ciphertext.iter().enumerate().fold(0.0, |acc, (i, bit)| if ciphertext[i] ^ bit >= 1 { acc + 1.0 } else { acc });
    }

    result / (block_size.pow(2) as f64)
}

/// Confusion test for cipher. Initializes the cipher with random key, plaintext and produces ciphertext. Then changes
/// one bit of key and checks new ciphertext with original to see how many bits have flipped. Repeats this cycle for
/// every bit in key.
///
/// Similar to diffusion, changing key bits instead of plaintext being the only difference.
fn confusion(block_size: u32, round_constants: &Vec<FieldElement>) -> f64 {
    let mut result = 0.0;
    let cipher = MiMC::with_round_constants(block_size, round_constants);
    let key = generate_random_bits(block_size);
    let plaintext = generate_random_bits(block_size);
    let ciphertext = cipher.encrypt(&plaintext, &key);

    for i in 0..(block_size as usize) {
        let mut new_key = key.to_vec();
        new_key[i] ^= 1; // Flip bit
        let new_ciphertext = cipher.encrypt(&plaintext, &new_key);
        // Count how many bits have flipped
        result += new_ciphertext.iter().enumerate().fold(0.0, |acc, (i, bit)| if ciphertext[i] ^ bit >= 1 { acc + 1.0 } else { acc });
    }

    result / (block_size.pow(2) as f64)
}

pub fn test_diffusion(test_size: usize, block_size: u32) {
    let start = Instant::now();
    let rounds = (block_size as f32 / 3f32.log(2.0)).ceil() as usize;
    let mut result = 0.0;
    for _ in 0..test_size {
        result += diffusion(block_size, &generate_round_constants(rounds, block_size));
    }

    println!("Final result {} in {:.2?}", result / (test_size as f64), start.elapsed());
}

pub fn test_confusion(test_size: usize, block_size: u32) {
    let start = Instant::now();
    let rounds = (block_size as f32 / 3f32.log(2.0)).ceil() as usize;
    let mut result = 0.0;
    for _ in 0..test_size {
        result += confusion(block_size, &generate_round_constants(rounds, block_size));
    }

    println!("Final result {} in {:.2?}", result / (test_size as f64), start.elapsed());
}
