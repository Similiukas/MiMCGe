use std::io;
use std::io::Write;
use std::time::Instant;
use crate::experiments::helpers::{choose_cipher, confusion, decryption_encryption, diffusion, standard_deviation, to_32_bit};
use crate::utils::helpers::{CipherType, FieldElement, to_binary, to_decimal};

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

    let mut sum = 0.0;
    let mut ssq = 0.0;
    for _ in 0..test_size {
        let r = diffusion(&cipher, block_size) as f64;
        sum += r;
        ssq += r.powi(2);
    }
    let mean = sum / test_size as f64;
    let std_deviation = standard_deviation(ssq, sum, test_size);
    let expected_mean = 0.5 * (block_size.pow(2) as f64);

    println!("Diffusion tested with {test_size} plaintexts");
    println!("Final result {} in {:.2?}", sum / (test_size * block_size.pow(2) as usize) as f64 * 10000.0, start.elapsed());
    println!("Expected mean: {} mean: {} standard deviation: {}", expected_mean, mean, std_deviation);
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

    let mut sum = 0.0;
    let mut ssq = 0.0;
    for _ in 0..test_size {
        let r = confusion(&cipher, block_size) as f64;
        sum += r;
        ssq += r.powi(2);
    }
    let mean = sum / test_size as f64;
    let std_deviation = standard_deviation(ssq, sum, test_size);
    let expected_mean = 0.5 * (block_size.pow(2) as f64);

    println!("Confusion tested with {test_size} plaintexts");
    println!("Final result {} in {:.2?}", sum / (test_size * block_size.pow(2) as usize) as f64 * 10000.0, start.elapsed());
    println!("Calculated r {} expected mean: {} mean: {} standard deviation: {}", sum, expected_mean, mean, std_deviation);
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
pub fn test_cipher(plaintext: FieldElement, block_size: u32, key: FieldElement, cipher_type: CipherType) {
    let cipher = choose_cipher(&cipher_type, block_size);
    let start = Instant::now();
    let ciphertext = cipher.encrypt(&plaintext, &key);
    let decrypted = cipher.decrypt(&ciphertext, &key);
    println!("Plaintext:  {} {:?}\nCiphertext: {} {:?}\nDecrypted:  {} {:?}\nTime: {:.2?}", to_decimal(&plaintext), plaintext, to_decimal(&ciphertext), ciphertext, to_decimal(&decrypted), decrypted, start.elapsed());
    assert_eq!(decrypted, plaintext);
}

/// Encrypts a sequential list of numbers from 0 up to specified *test_size*.
///
/// These numbers are then printed to standard output as ASCII bit array (1s and 0s).
/// Each line represents a different number.
pub fn encrypt_seq(test_size: usize, block_size: u32, key: FieldElement, cipher_type: CipherType) {
    let cipher = choose_cipher(&cipher_type, block_size);
    for i in 0..test_size {
        println!("{}", cipher.encrypt(&to_binary(i as u128, block_size), &key).into_iter().map(|x| x.to_string()).collect::<Vec<String>>().join(""));
    }
}

/// Encrypts a sequential list of numbers from 0 up to  2^32 < 4531145293 < 2^33.
///
/// This outputs encrypted sequential numbers to the standard output in 32 bits. If the cipher encrypts
/// numbers larger than 32 bits, then the low end is discarded. This function will never end and when
/// the sequence reaches the end, the cycle repeats.
pub fn encrypt_seq_stream(block_size: u32, key: FieldElement, cipher_type: CipherType) {
    let cipher = choose_cipher(&cipher_type, block_size);

    let mut i = 0u128;
    loop {
        io::stdout().write_all(&to_32_bit(cipher.encrypt(&to_binary(i, 33), &key))).unwrap();
        i = (i + 1) % 4531145293;
    }
}
