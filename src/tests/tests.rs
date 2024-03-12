use std::time::Instant;
use crate::mimc_general::mimc_general::MiMCGe;
use crate::tests::helpers::{choose_cipher, confusion, decryption_encryption, diffusion};
use crate::utils::helpers::{Cipher, CipherType, FieldElement, generate_random_bits, to_binary, to_decimal};

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
pub fn test_diffusion(test_size: usize, block_size: u32, cipher_type: CipherType) {
    let start = Instant::now();
    let cipher = choose_cipher(&cipher_type, block_size);

    let mut result = 0;
    for _ in 0..test_size {
        result += diffusion(&cipher, block_size);
    }

    println!("Diffusion tested with {test_size} plaintexts\nFinal result {} in {:.2?}", (result as f64) / (test_size * block_size.pow(2) as usize) as f64 * 10000.0, start.elapsed());
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
pub fn test_confusion(test_size: usize, block_size: u32, cipher_type: CipherType) {
    let start = Instant::now();
    let cipher = choose_cipher(&cipher_type, block_size);

    let mut result = 0;
    for _ in 0..test_size {
        result += confusion(&cipher, block_size);
    }

    println!("Confusion tested with {test_size} plaintexts\nFinal result {} in {:.2?}", (result as f64) / (test_size * block_size.pow(2) as usize) as f64 * 10000.0, start.elapsed());
}

/// # Encryption efficiency test for cipher
///
/// Initializes cipher with random round constants, generates **sample_size** plaintexts and encrypts all of them
/// sequentially.
///
/// This cycle is repeated **test_size** times.
///
/// Returns the time it takes to encrypt **test_size** x **sample_size** plaintexts.
pub fn test_encryption_time(test_size: usize, sample_size: usize, block_size: u32, cipher_type: CipherType) {
    let time = decryption_encryption(false, test_size, sample_size, block_size, cipher_type);
    println!("Total time to encrypt {test_size}x{sample_size} plaintexts {:.6?}", time);
}

/// # Decryption efficiency test for cipher
///
/// Initializes cipher with random round constants, generates **sample_size** plaintexts and decrypts all of them
/// sequentially.
///
/// This cycle is repeated **test_size** times.
///
/// Returns the time it takes to decrypt **test_size** x **sample_size** ciphertexts.
pub fn test_decryption_time(test_size: usize, sample_size: usize, block_size: u32, cipher_type: CipherType) {
    let time = decryption_encryption(true, test_size, sample_size, block_size, cipher_type);
    println!("Total time to decrypt {test_size}x{sample_size} ciphertexts {:.6?}", time);
}

/// # Simple encryption and decryption test
///
/// Check if the cipher correctly decrypts the encrypted message
pub fn test_cipher(plaintext: FieldElement, block_size: u32, cipher_type: CipherType) {
    let cipher = choose_cipher(&cipher_type, block_size);
    let key = generate_random_bits(block_size);
    let start = Instant::now();
    let ciphertext = cipher.encrypt(&plaintext, &key);
    let decrypted = cipher.decrypt(&ciphertext, &key);
    println!("Plaintext:  {} {:?}\nCiphertext: {} {:?}\nDecrypted:  {} {:?}\nTime: {:.2?}", to_decimal(&plaintext), plaintext, to_decimal(&ciphertext), ciphertext, to_decimal(&decrypted), decrypted, start.elapsed());
    assert_eq!(decrypted, plaintext);
}

pub fn encrypt_many(test_size: usize, key: FieldElement, block_size: u32, exponent: u128, round_constants: Vec<u128>) {
    let rc = round_constants.iter().map(|x| to_binary(*x, block_size)).collect::<Vec<FieldElement>>();
    let cipher = MiMCGe::with_round_constants(exponent, block_size, &rc);
    for i in 0..test_size {
        println!("{}", cipher.encrypt(&to_binary(i as u128, block_size), &key).into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
    }
}
