use rand::{Rng, thread_rng};

pub type FieldElement = Vec<u8>;

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
pub fn generate_random_bits(block_size: usize) -> FieldElement {
    let mut result: Vec<u8> = Vec::with_capacity(block_size);
    for _ in 0..block_size {
        result.push(thread_rng().gen_range(0..=1));
    }
    result
}

pub fn generate_random_element(field: u32) -> FieldElement {
    let block_size = (field as f32).log(2.0).ceil() as usize;
    to_binary(thread_rng().gen_range(0..field), block_size)
}
/// Converts bit array to decimal expression
pub fn to_decimal(bits: &FieldElement) -> u32 {
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

pub fn to_binary(number: u32, block_size: usize) -> FieldElement {
    let mut result = FieldElement::new();
    let mut state = number;
    while state > 0 {
        result.push((state % 2) as u8);
        state /= 2;
    }

    // Pad to right size
    result.append(&mut vec![0u8; block_size - result.len()]);
    result.reverse();
    result
}

/// Adds elements over 2^n field for any n
pub fn add_field_elements_over_finite_field(a: &FieldElement, b: &FieldElement) -> FieldElement {
    let mut result = a.to_vec();
    for i in 0..result.len() {
        result[i] = a[i] ^ b[i];
    }
    result
}

// https://en.wikipedia.org/wiki/Finite_field_arithmetic#C_programming_example
pub fn multiply_finite_field(a: &FieldElement, b: &FieldElement) -> FieldElement {
    // let irreducible_poly: FieldElement = vec![1, 0, 0, 1, 1]; // x^4 + x + 1
    let mut p = 0u32;
    let mut a_n = to_decimal(&a);
    let mut b_n = to_decimal(&b);
    while a_n != 0 && b_n != 0 {
        if (b_n & 1) == 1 {
            p ^= a_n;
        }

        if (a_n & 0x8) >= 1 {
            a_n = (a_n << 1) ^ 0x13 // // x^4 + x + 1
        } else {
            a_n <<= 1;
        }
        b_n >>= 1;
    }
    to_binary(p as u32, 4)
}

/// Adds elements over F_p field, where p is prime
pub fn add_field_elements_over_prime_field(a: &FieldElement, b: &FieldElement, field: u32, block_size: usize) -> FieldElement {
    to_binary((to_decimal(a) + to_decimal(b)) % field, block_size)
}

pub fn multiply_over_prime_field(a: &FieldElement, b: &FieldElement, field: u32, block_size: usize) -> FieldElement {
    to_binary((to_decimal(a) * to_decimal(b)) % field, block_size)
}

pub fn power_over_prime_field(a: &FieldElement, pow: u32, field: u32, block_size: usize) -> FieldElement {
    let mut result: FieldElement = a.to_vec();
    for _ in 0..pow {
        result = multiply_over_prime_field(&result, a, field, block_size);
    }
    result
}