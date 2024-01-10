use std::collections::HashMap;
use rand::{Rng, thread_rng};
use lazy_static::lazy_static;

pub type FieldElement = Vec<u8>;

lazy_static! {
    static ref IRREDUCIBLE_POLYNOMIALS: HashMap<u32, (u32, u32)> = HashMap::from([
        (5, (0x10, 0x25)),              // x^4,  x^5 + x^2 + 1
        (17, (0x10000, 0x20009)),       // x^16, x^17 + x^3 + 1
        (25, (0x1000000, 0x2000145)),   // x^24, x^25 + x^8 + x^6 + x^2 + 1
        (31, (0x40000000, 0x80000009)), // x^30, x^31 + x^3 + 1
    ]);
}

pub trait Cipher {
    fn encrypt(&self, plaintext: &FieldElement, key: &FieldElement) -> FieldElement;
    fn decrypt(&self, ciphertext: &FieldElement, key: &FieldElement) -> FieldElement;
}

pub fn gcd(a: usize, b: usize) -> usize {
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

/// Generates random number in the specified field as binary element and pads to correct block size.
pub fn generate_random_element(field: u32) -> FieldElement {
    let block_size = (field as f32).log(2.0).ceil() as u32;
    to_binary(thread_rng().gen_range(0..field), block_size)
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
pub fn to_decimal(bits: &[u8]) -> u32 {
    let mut result = 0;
    let mut multiple = 1;
    for &bit in bits.iter().rev() {
        if bit == 1u8 {
            result += multiple;
        }
        multiple *= 2;
    }
    result
}

/// Converts number to bit array expression
pub fn to_binary(number: u32, block_size: u32) -> FieldElement {
    let mut result = FieldElement::new();
    let mut state = number;
    while state > 0 {
        result.push((state % 2) as u8);
        state /= 2;
    }

    // Pad to correct size
    result.append(&mut vec![0u8; (block_size as usize) - result.len()]);
    result.reverse();
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
    let poly = IRREDUCIBLE_POLYNOMIALS[&block_size];
    let mut p = 0u32;
    let mut a_n = to_decimal(&a);
    let mut b_n = to_decimal(&b);
    while a_n != 0 && b_n != 0 {
        if (b_n & 1) >= 1 {
            p ^= a_n;
        }

        if (a_n & poly.0) >= 1 {
            a_n = (a_n << 1) ^ poly.1;
        } else {
            a_n <<= 1;
        }
        b_n >>= 1;
    }
    to_binary(p, block_size)
}

/// Naive approach for exponentiation in finite field. The implementation is to just multiply the number n times.
///
/// This can be optimized further using [square and multiply](https://en.wikipedia.org/wiki/Exponentiation_by_squaring)
/// method or similar to reduce the number of multiplications.
pub fn power_finite_field(a: &FieldElement, exponent: u32, block_size: u32) -> FieldElement {
    let mut result: FieldElement = a.to_vec();
    for _ in 0..exponent - 1 {
        result = multiply_finite_field(&result, a, block_size);
    }
    result
}

/// Adds elements over F_p field, where p is prime
pub fn add_field_elements_over_prime_field(a: &FieldElement, b: &FieldElement, field: u32, block_size: u32) -> FieldElement {
    to_binary((to_decimal(a) + to_decimal(b)) % field, block_size)
}

pub fn multiply_over_prime_field(a: &FieldElement, b: &FieldElement, field: u32, block_size: u32) -> FieldElement {
    to_binary((to_decimal(a) * to_decimal(b)) % field, block_size)
}

pub fn power_over_prime_field(a: &FieldElement, pow: u32, field: u32, block_size: u32) -> FieldElement {
    let mut result: FieldElement = a.to_vec();
    for _ in 0..pow {
        result = multiply_over_prime_field(&result, a, field, block_size);
    }
    result
}