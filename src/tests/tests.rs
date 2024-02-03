use std::time::Instant;
use crate::aes::aes::AES;
use crate::mimc::mimc::MiMC;
use crate::mimc_general::mimc_general::MiMCGn;
use crate::utils::helpers::{Cipher, CipherType, generate_random_bits};

/// Diffusion test for cipher. Initializes the cipher with random key, plaintext and produces ciphertext. Then changes
/// one bit of plaintext and checks new ciphertext with original to see how many bits have flipped. Repeats this cycle
/// for every bit in plaintext.
fn diffusion(cipher: &Box<dyn Cipher>, block_size: u32) -> f64 {
    let mut result = 0.0;
    let key = generate_random_bits(block_size);
    let plaintext = generate_random_bits(block_size);
    let ciphertext = cipher.encrypt(&plaintext, &key);

    for i in 0..(block_size as usize) {
        let mut new_plaintext = plaintext.to_vec();
        new_plaintext[i] ^= 1; // Flip ith bit
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
fn confusion(cipher: &Box<dyn Cipher>, block_size: u32) -> f64 {
    let mut result = 0.0;
    let key = generate_random_bits(block_size);
    let plaintext = generate_random_bits(block_size);
    let ciphertext = cipher.encrypt(&plaintext, &key);

    for i in 0..(block_size as usize) {
        let mut new_key = key.to_vec();
        new_key[i] ^= 1; // Flip ith bit
        let new_ciphertext = cipher.encrypt(&plaintext, &new_key);
        // Count how many bits have flipped
        result += new_ciphertext.iter().enumerate().fold(0.0, |acc, (i, bit)| if ciphertext[i] ^ bit >= 1 { acc + 1.0 } else { acc });
    }

    result / (block_size.pow(2) as f64)
}

fn choose_cipher(t: CipherType, block_size: u32) -> Box<dyn Cipher> {
    match t {
        CipherType::AES => Box::new(AES{}),
        CipherType::MiMC => Box::new(MiMC::new(block_size)),
        CipherType::MiMCGn => Box::new(MiMCGn::new(3, block_size)),
    }
}

pub fn test_diffusion(test_size: usize, block_size: u32, cipher: CipherType) {
    let start = Instant::now();
    let cipher = choose_cipher(cipher, block_size);

    // TODO: right now building the cipher once, that is, RC are chosen once. Is this what we want?
    let mut result = 0.0;
    for _ in 0..test_size {
        result += diffusion(&cipher, block_size);
    }

    println!("Final result {} in {:.2?}", result / (test_size as f64), start.elapsed());
}

pub fn test_confusion(test_size: usize, block_size: u32, cipher: CipherType) {
    let start = Instant::now();
    let cipher = choose_cipher(cipher, block_size);

    let mut result = 0.0;
    for _ in 0..test_size {
        result += confusion(&cipher, block_size);
    }

    println!("Final result {} in {:.2?}", result / (test_size as f64), start.elapsed());
}
