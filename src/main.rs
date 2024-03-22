use crate::tests::tests::{encrypt_many, test_cipher, test_confusion, test_decryption_time, test_diffusion, test_encryption_time};
use crate::utils::helpers::{CipherType, FieldElement, generate_random_bits, to_binary};
use clap::Parser;
use clap::builder::TypedValueParser;

mod utils;
mod mimc;
mod experiments;
mod aes;
mod mimc_general;
mod tests;

// TODO: add link to report in readme

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Type of test to be performed.
    #[arg(value_parser=["diffusion", "confusion", "enc-time", "dec-time", "cipher-test", "generate-test-samples"])]
    test_type: String,

    /// Cipher type.
    #[arg(value_parser=["aes", "mimc", "mimcge"])]
    cipher_type: String,

    /// Block size (only some are implemented for MiMC/MiMCGe, only 128 bit for AES).
    #[arg(
        default_value_t = 17,
        value_parser = clap::builder::PossibleValuesParser::new(["5", "8", "11", "17", "25", "31", "33", "47", "61", "83", "101", "125", "127", "128"])
                .map(|s| s.parse::<u32>().unwrap()))]
    block_size: u32,

    /// Test size (ranging from 1 to u64). How many times to repeat the same test.
    #[arg(short, long, default_value = "1")]
    test_size: usize,

    /// How many different plaintexts to encrypt/decrypt with one cipher.
    #[arg(short, long, default_value = "1")]
    sample_size : usize,

    /// Plaintext to encrypt. If not given, random one is chosen.
    #[arg(short, long, default_value = None)]
    plaintext: Option<u128>,

    /// Key used in encryption.
    #[arg(short, long, default_value = None)]
    key: Option<u128>,

    /// Exponent for MiMCGe cipher *x^n*.
    #[arg(short, long, default_value = "3")]
    exponent: u128,

    /// How many rounds to reduce for the MiMCGe cipher.
    #[arg(short, long, default_value = None)]
    round_reduction: Option<usize>,

    /// Key used in encryption. (Only when using encrypt option)
    #[arg(short, long, default_value = "0")]
    key: u128,

    /// Round constants used for MiMCGe cipher. (Only when using encrypt option).
    #[arg(short='R', long, num_args = 1..)]
    round_constants: Vec<u128>,

    /// How many rounds to reduce for the MiMCGe cipher. (Ignored if round constants are given)
    #[arg(short, long, default_value = None)]
    round_reduction: Option<usize>,
}

fn main() {
    let args = Args::parse();
    // println!("{:?}", args);

    let cipher_type = match args.cipher_type.as_str() {
        "aes" => CipherType::AES,
        "mimc" => CipherType::MiMC,
        "mimcge" => CipherType::MiMCGe(args.exponent, &args.round_constants, args.round_reduction),
        _ => unreachable!()
    };

    let plaintext: FieldElement =
        if args.plaintext.is_some() {
            to_binary(args.plaintext.unwrap_or(0), args.block_size)
        } else {
            generate_random_bits(args.block_size)
        };

    let key: FieldElement =
        if args.key.is_some() {
            to_binary(args.key.unwrap_or(0), args.block_size)
        } else {
            generate_random_bits(args.block_size)
        };

    match args.test_type.as_str() {
        "diffusion" => test_diffusion(args.test_size, args.block_size, cipher_type),
        "confusion" => test_confusion(args.test_size, args.block_size, cipher_type),
        "enc-time" => test_encryption_time(args.test_size, args.sample_size, args.block_size, cipher_type),
        "dec-time" => test_decryption_time(args.test_size, args.sample_size, args.block_size, cipher_type),
        "cipher-test" => test_cipher(plaintext, args.block_size, cipher_type),
        "generate-test-samples" => encrypt_many(args.test_size, to_binary(args.key, args.block_size), args.block_size, args.exponent, args.round_constants),
        _ => unreachable!()
    }
}
