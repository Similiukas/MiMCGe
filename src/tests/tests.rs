use std::time::Instant;
use crate::tests::helpers::{choose_cipher, confusion, decryption_encryption, diffusion};
use crate::utils::helpers::CipherType;

/// # Diffusion test for cipher.
///
/// First, initializing the cipher with random round constants. Then generates random key, plaintext and produces
/// ciphertext. With this, changes one bit of plaintext and checks new ciphertext with original to see how many bits
/// have flipped. Repeats this cycle for every bit in plaintext.
///
/// This cycle, without initializing the cipher, is repeated **test_size** times.
///
/// # Note
/// Similar to confusion, changing plaintext bits instead of key being the only difference.
pub fn test_diffusion(test_size: usize, block_size: u32, cipher: CipherType) {
    let start = Instant::now();
    let cipher = choose_cipher(&cipher, block_size);

    let mut result = 0;
    for _ in 0..test_size {
        result += diffusion(&cipher, block_size);
    }

    println!("Final result {} in {:.2?}", (result as f64) / (test_size * block_size.pow(2) as usize) as f64 * 10000.0, start.elapsed());
}

/// # Confusion test for cipher.
///
/// First, initializing the cipher with random round constants.Then takes random key, plaintext and produces ciphertext.
/// With this, changes one bit of key and checks new ciphertext with original to see how many bits have flipped. Repeats
/// this cycle for every bit in key.
///
/// This cycle, without initializing the cipher, is repeated **test_size** times.
///
/// # Note
/// Similar to diffusion, changing key bits instead of plaintext being the only difference.
pub fn test_confusion(test_size: usize, block_size: u32, cipher: CipherType) {
    let start = Instant::now();
    let cipher = choose_cipher(&cipher, block_size);

    let mut result = 0;
    for _ in 0..test_size {
        result += confusion(&cipher, block_size);
    }

    println!("Final result {} in {:.2?}", (result as f64) / (test_size * block_size.pow(2) as usize) as f64 * 10000.0, start.elapsed());
}

/// # Encryption test for cipher
///
/// Initializes cipher with random round constants, generates **sample_size** plaintexts and encrypts all of them
/// sequentially.
///
/// This cycle is repeated **test_size** times.
pub fn test_encryption(test_size: usize, sample_size: usize, block_size: u32, cipher: CipherType) {
    let time = decryption_encryption(false, test_size, sample_size, block_size, cipher);
    println!("Total time to encrypt {test_size}x{sample_size} plaintexts {:.2?}", time);
}

/// # Decryption test for cipher
///
/// Initializes cipher with random round constants, generates **sample_size** plaintexts and decrypts all of them
/// sequentially.
///
/// This cycle is repeated **test_size** times.
pub fn test_decryption(test_size: usize, sample_size: usize, block_size: u32, cipher: CipherType) {
    let time = decryption_encryption(true, test_size, sample_size, block_size, cipher);
    println!("Total time to decrypt {test_size}x{sample_size} ciphertexts {:.2?}", time);
}
