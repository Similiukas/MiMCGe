use std::collections::HashMap;
use rand::{Rng, thread_rng};
use lazy_static::lazy_static;

pub type FieldElement = Vec<u8>;

pub enum CipherType<'a> {
    AES,
    MiMC,
    MiMCGe(u128, &'a Vec<u128>, Option<usize>),
}

lazy_static! {
    static ref IRREDUCIBLE_POLYNOMIALS: HashMap<u32, (u128, u128)> = HashMap::from([
        (5, (0x10, 0x25)),                                                              // x^4,  x^5 + x^2 + 1
        (8, (0x80, 0x11D)),                                                             // x^7,  x^8 + x^4 + x^3 + x^2 + 1
        (11, (0x400, 0x805)),                                                           // x^10, x^11 + x^2 + 1
        (17, (0x10000, 0x20009)),                                                       // x^16, x^17 + x^3 + 1
        (25, (0x1000000, 0x2000145)),                                                   // x^24, x^25 + x^8 + x^6 + x^2 + 1
        (31, (0x40000000, 0x80000009)),                                                 // x^30, x^31 + x^3 + 1
        (33, (0x100000000, 0x200003D49)),                                               // x^32, x^33 + x^13 + x^12 + x^11 + x^10 + x^8 + x^6 + x^3 + 1
        (47, (0x400000000000, 0x800000000021)),                                         // x^46, x^47 + x^5 + 1
        (61, (0x1000000000000000, 0x2000000000000027)),                                 // x^60, x^61 + x^5 + x^2 + x + 1
        (83, (0x400000000000000000000, 0x800000000000000000095)),                       // x^82, x^83 + x^7 + x^4 + x^2 + 1
        (101, (0x10000000000000000000000000, 0x200000000000000000000000C3)),            // x^100, x^101 + x^7 + x^6 + x + 1
        (125, (0x10000000000000000000000000000000, 0x200000000000000000000000004BBE69)),// x^124, x^125 + x^22 + x^19 + x^17 + x^16 + x^15 + x^13 + x^12 + x^11 + x^10 + x^9 + x^6 + x^5 + x^3 + 1
        (127, (0x40000000000000000000000000000000, 0x80000000000000000000000000000003)) // x^126, x^127 + x + 1
    ]);
}

pub trait Cipher {
    fn encrypt(&self, plaintext: &FieldElement, key: &FieldElement) -> FieldElement;
    fn decrypt(&self, ciphertext: &FieldElement, key: &FieldElement) -> FieldElement;
}

pub fn gcd(a: u128, b: u128) -> u128 {
    return if b == 0 { a } else { gcd(b, a % b ) }
}

/// Generates random bit array of size `block_size`. Bits are from right to left, i.e. [0, 1] -> 1
/// and [1, 0] -> 2
/// # Example
/// ```
/// let bits = generate_random_bits(4);
///
/// assert_eq!(bits, vec![0, 1, 1, 1]);
/// ```
pub fn generate_random_bits(block_size: u32) -> FieldElement {
    let mut result: Vec<u8> = Vec::with_capacity(block_size as usize);
    for _ in 0..block_size {
        result.push(thread_rng().gen_range(0..=1));
    }
    result
}

/// Generates random round constants for MiMC type cipher, where the first constant is 0.
pub fn generate_round_constants(size: usize, block_size: u32) -> Vec<FieldElement> {
    let mut result: Vec<FieldElement> = Vec::with_capacity(size);
    result.push(vec![0; block_size as usize]); // c_0 must be 0
    for _ in 1..size {
        result.push(generate_random_bits(block_size));
    }
    result
}

/// Converts bit array to decimal expression
pub fn to_decimal(bits: &[u8]) -> u128 {
    let mut result: u128 = 0;
    let mut multiple = 1;
    for &bit in bits.iter().rev() {
        result += (bit as u128) * multiple;
        multiple *= 2;
    }
    result
}

/// Converts number to bit array expression
pub fn to_binary(number: u128, block_size: u32) -> FieldElement {
    let b = block_size as usize;
    let mut result = vec![0u8; b];
    let mut state = number;
    for i in 0..b {
        result[b - i - 1] = (state & 1) as u8;
        state >>= 1;
    }

    result
}

/// Adds elements over 2^n field for any n
pub fn add_finite_field(a: &FieldElement, b: &FieldElement) -> FieldElement {
    let mut result = a.to_vec();
    for i in 0..result.len() {
        result[i] = a[i] ^ b[i];
    }
    result
}

fn _multiply_finite_field(mut a: u128, mut b: u128, block_size: u32) -> u128 {
    let poly = IRREDUCIBLE_POLYNOMIALS[&block_size];
    let mut p = 0u128;
    while a != 0 && b != 0 {
        if (b & 1) >= 1 {
            p ^= a;
        }

        if (a & poly.0) >= 1 {
            a = (a << 1) ^ poly.1;
        } else {
            a <<= 1;
        }
        b >>= 1;
    }
    p
}

/// Multiplication in extension field 2^n for n as `block_size`. For every `block_size`, multiplication must be
/// implemented separately as this is f(x) * g(x) mod h(x) where h(x) is the irreducible polynomial (equivalent to prime
/// number in rings) which is different for every field.
///
/// This specific implementation uses [Russian peasant multiplication
/// algorithm](https://en.wikipedia.org/wiki/Finite_field_arithmetic#C_programming_example). This can be further
/// optimized using [precomputed tables](https://en.wikipedia.org/wiki/Finite_field_arithmetic#Generator_based_tables),
/// [hardware specific instructions](https://en.wikipedia.org/wiki/Carry-less_product) or any other method. However, for
/// small enough fields, this method is rather fast enough.
pub fn multiply_finite_field(a: &FieldElement, b: &FieldElement, block_size: u32) -> FieldElement {
    assert!(IRREDUCIBLE_POLYNOMIALS.contains_key(&block_size), "Multiplication for this block size is not implemented");
    to_binary(_multiply_finite_field(to_decimal(&a), to_decimal(&b), block_size), block_size)
}

/// Naive approach for exponentiation in finite field. The implementation is to just multiply the number n times.
///
/// This can be optimized further using [square and multiply](https://en.wikipedia.org/wiki/Exponentiation_by_squaring)
/// method or similar to reduce the number of multiplications.
pub fn power_finite_field(a: &FieldElement, exponent: usize, block_size: u32) -> FieldElement {
    let mut result: FieldElement = a.to_vec();
    for _ in 0..exponent - 1 {
        result = multiply_finite_field(&result, a, block_size);
    }
    result
}

fn _square_multiply(y: u128, x: u128, exponent: u128, block_size: u32) -> u128 {
    return if exponent == 0 { y }
    else if exponent % 2 == 0 { _square_multiply(y, _multiply_finite_field(x, x, block_size), exponent / 2, block_size) }
    else { _square_multiply(_multiply_finite_field(x, y, block_size), _multiply_finite_field(x, x, block_size), (exponent - 1) / 2, block_size) }
}

/// Fast exponentiation implementation using [square and multiply](https://en.wikipedia.org/wiki/Exponentiation_by_squaring) algorithm.
pub fn square_multiply(a: &FieldElement, exponent: u128, block_size: u32) -> FieldElement {
    assert!(IRREDUCIBLE_POLYNOMIALS.contains_key(&block_size), "Multiplication for this block size is not implemented");
    to_binary(_square_multiply(1, to_decimal(&a), exponent, block_size), block_size)
}
