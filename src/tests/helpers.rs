use std::time::{Duration, Instant};
use crate::aes::aes::AES;
use crate::mimc::mimc::MiMC;
use crate::mimc_general::mimc_general::MiMCGn;
use crate::utils::helpers::{Cipher, CipherType, FieldElement, generate_random_bits};


pub fn choose_cipher(t: &CipherType, block_size: u32) -> Box<dyn Cipher> {
    match t {
        CipherType::AES => Box::new(AES{}),
        CipherType::MiMC => Box::new(MiMC::new(block_size)),
        CipherType::MiMCGn(e) => Box::new(MiMCGn::new(*e, block_size)),
    }
}

pub fn diffusion(cipher: &Box<dyn Cipher>, block_size: u32) -> f64 {
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

pub fn confusion(cipher: &Box<dyn Cipher>, block_size: u32) -> f64 {
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

pub fn encryption(plaintexts: Vec<FieldElement>, key: FieldElement, cipher: &Box<dyn Cipher>) -> Duration {
    let start = Instant::now();
    for plaintext in plaintexts {
        cipher.encrypt(&plaintext, &key);
    }
    start.elapsed()
}

pub fn decryption(ciphertexts: Vec<FieldElement>, key: FieldElement, cipher: &Box<dyn Cipher>) -> Duration {
    let start = Instant::now();
    for ciphertext in ciphertexts {
        cipher.decrypt(&ciphertext, &key);
    }
    start.elapsed()
}
