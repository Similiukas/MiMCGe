use crate::tests::tests::{test_cipher, test_confusion, test_decryption_time, test_diffusion, test_encryption_time};
use crate::utils::helpers::{CipherType, FieldElement, to_binary};
use clap::Parser;

mod utils;
mod mimc;
mod tests;
mod aes;
mod mimc_general;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Test type
    #[arg(value_parser=["diffusion", "confusion", "enc-time", "dec-time", "cipher-test"])]
    test_type: String,

    /// Cipher type
    #[arg(value_parser=["aes", "mimc", "mimcgn"])]
    cipher_type: String,

    //TODO: fix this#[arg(value_parser=[5, 8, 11, 17, 25, 31, 47, 61, 83, 101, 125, 127])]
    /// Block size (only some valid are implemented)
    block_size: u32,

    /// Test size (ranging from 1 to u64)
    #[arg(short, long, default_value = "1")]
    test_size: usize,

    #[arg(short, long, default_value = "1")]
    sample_size : usize,

    #[arg(short, long, default_value = None)]
    plaintext: Option<u128>,

    #[arg(short, long, default_value = "3")]
    exponent: u128,

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
