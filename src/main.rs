use crate::tests::tests::{test_cipher, test_confusion, test_decryption_time, test_diffusion, test_encryption_time};
use crate::utils::helpers::{CipherType, FieldElement, to_binary};
use clap::Parser;
use clap::builder::TypedValueParser;

mod utils;
mod mimc;
mod tests;
mod aes;
mod mimc_general;

// TODO: add link to report in readme

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Type of test to be performed.
    #[arg(value_parser=["diffusion", "confusion", "enc-time", "dec-time", "cipher-test"])]
    test_type: String,

    /// Cipher type.
    #[arg(value_parser=["aes", "mimc", "mimcgn"])]
    cipher_type: String,

    /// Block size (only some are implemented for MiMC/MiMCGn, only 128 bit for AES).
    #[arg(
        default_value_t = 17,
        value_parser = clap::builder::PossibleValuesParser::new(["5", "8", "11", "17", "25", "31", "47", "61", "83", "101", "125", "127", "128"])
                .map(|s| s.parse::<u32>().unwrap()))]
    block_size: u32,

    /// Test size (ranging from 1 to u64).
    #[arg(short, long, default_value = "1")]
    test_size: usize,

    /// How many different plaintexts to encrypt/decrypt with one cipher.
    #[arg(short, long, default_value = "1")]
    sample_size : usize,

    /// Plaintext to encrypt. If not given, random one is chosen.
    #[arg(short, long, default_value = None)]
    plaintext: Option<u128>,

    /// Exponent for MiMCGn cipher *x^n*.
    #[arg(short, long, default_value = "3")]
    exponent: u128,

    /// How many rounds to reduce for the MiMCGn cipher.
    #[arg(short, long, default_value = None)]
    round_reduction: Option<usize>
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);

    let cipher_type = match args.cipher_type.as_str() {
        "aes" => CipherType::AES,
        "mimc" => CipherType::MiMC,
        "mimcgn" => CipherType::MiMCGn(args.exponent, args.round_reduction),
        _ => unreachable!()
    };

    let plaintext: Option<FieldElement> = if args.plaintext.is_some() { Some(to_binary(args.plaintext.unwrap_or(1), args.block_size)) } else {None};

    match args.test_type.as_str() {
        "diffusion" => test_diffusion(args.test_size, args.block_size, cipher_type),
        "confusion" => test_confusion(args.test_size, args.block_size, cipher_type),
        "enc-time" => test_encryption_time(args.test_size, args.sample_size, args.block_size, cipher_type),
        "dec-time" => test_decryption_time(args.test_size, args.sample_size, args.block_size, cipher_type),
        "cipher-test" => test_cipher(plaintext, args.block_size, cipher_type),
        _ => unreachable!()
    }
}
