use std::time::{Duration, Instant};
use crate::aes::aes::AES;
use crate::mimc::mimc::MiMC;
use crate::mimc_general::mimc_general::MiMCGe;
use crate::utils::helpers::{Cipher, CipherType, FieldElement, generate_random_bits};


pub fn choose_cipher(t: &CipherType, block_size: u32) -> Box<dyn Cipher> {
    match t {
        CipherType::AES => Box::new(AES{}),
        CipherType::MiMC => Box::new(MiMC::new(block_size)),
        CipherType::MiMCGe(e, rr) => Box::new(MiMCGe::new(*e, block_size, *rr)),
    }
}

pub fn diffusion(cipher: &Box<dyn Cipher>, block_size: u32) -> usize {
    let mut result = 0;
    let key = generate_random_bits(block_size);
    let plaintext = generate_random_bits(block_size);
    let ciphertext = cipher.encrypt(&plaintext, &key);

    for i in 0..(block_size as usize) {
        let mut new_plaintext = plaintext.to_vec();
        new_plaintext[i] ^= 1; // Flip ith bit
        let new_ciphertext = cipher.encrypt(&new_plaintext, &key);
        // Count how many bits have flipped
        result += new_ciphertext.iter().enumerate().fold(0, |acc, (i, bit)| if ciphertext[i] ^ bit >= 1 { acc + 1 } else { acc });
    }

    result
}

pub fn confusion(cipher: &Box<dyn Cipher>, block_size: u32) -> usize {
    let mut result = 0;
    let key = generate_random_bits(block_size);
    let plaintext = generate_random_bits(block_size);
    let ciphertext = cipher.encrypt(&plaintext, &key);

    for i in 0..(block_size as usize) {
        let mut new_key = key.to_vec();
        new_key[i] ^= 1; // Flip ith bit
        let new_ciphertext = cipher.encrypt(&plaintext, &new_key);
        // Count how many bits have flipped
        result += new_ciphertext.iter().enumerate().fold(0, |acc, (i, bit)| if ciphertext[i] ^ bit >= 1 { acc + 1 } else { acc });
    }

    result
}

fn encryption(plaintexts: Vec<FieldElement>, key: FieldElement, cipher: &Box<dyn Cipher>) -> Duration {
    let start = Instant::now();
    for plaintext in plaintexts {
        cipher.encrypt(&plaintext, &key);
    }
    start.elapsed()
}

fn decryption(ciphertexts: Vec<FieldElement>, key: FieldElement, cipher: &Box<dyn Cipher>) -> Duration {
    let start = Instant::now();
    for ciphertext in ciphertexts {
        cipher.decrypt(&ciphertext, &key);
    }
    start.elapsed()
}

pub fn decryption_encryption(decrypt: bool, test_size: usize, sample_size: usize, block_size: u32, cipher: CipherType) -> Duration {
    let mut start = Duration::new(0, 0);
    for _ in 0..test_size {
        let cipher = choose_cipher(&cipher, block_size);
        let mut plaintexts: Vec<FieldElement> = Vec::with_capacity(sample_size);
        for _ in 0..sample_size {
            plaintexts.push(generate_random_bits(block_size));
        }
        if decrypt {
            start += decryption(plaintexts, generate_random_bits(block_size), &cipher);
        } else {
            start += encryption(plaintexts, generate_random_bits(block_size), &cipher);
        }
    }
    start
}
