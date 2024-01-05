use std::fmt;
use std::fmt::{Formatter, Pointer};
use crate::utils::helpers::{add_field_elements_over_finite_field, add_field_elements_over_prime_field, FieldElement, gcd, generate_random_bits, generate_random_element, multiply_over_prime_field, power_over_prime_field, to_decimal};

fn generate_round_constants(size: usize, block_size: u32) -> Vec<FieldElement> {
    let mut result: Vec<FieldElement> = Vec::with_capacity(size);
    result.push(vec![0; size]); // c_0 must be 0
    for _ in 1..size {
        result.push(generate_random_bits(block_size as usize));
        // result.push(generate_random_element(field));
    }
    result
}

pub struct MiMC {
    pub block_size: u32,
    rounds: usize,
    field: u32,
    round_constants: Vec<FieldElement>
}

impl MiMC {
    pub fn new(block_size: u32) -> Self {
        // For field 2 ** block_size
        // let rounds = ((field as f32).log(2.0) / 3f32.log(2.0)).ceil() as usize;
        // For prime field, we need cubing function to be a permutation, i.e. gcd(3, p-1) = 1 (original paper chapter 5)
        // assert_eq!(gcd(3, (field - 1) as usize), 1);
        let rounds = (block_size as f32 / 3f32.log(2.0)).ceil() as usize;
        let field = 2u32.pow(block_size);
        println!("rounds {block_size} {rounds} {field}");
        MiMC {
            block_size,
            rounds,
            field,
            round_constants: generate_round_constants(rounds, block_size)
        }
    }

    pub fn encrypt(&self, plaintext: &FieldElement) -> FieldElement {
        let mut state: FieldElement = plaintext.to_vec();
        for round in 0..self.rounds {
            // let temp = state + &self.round_constants[round];
            let temp1 = add_field_elements_over_finite_field(&state, &self.round_constants[round]);
            // state = temp ** 3;
            let temp2 = multiply_over_prime_field(&temp1, &temp1, self.field, self.block_size as usize);
            state = multiply_over_prime_field(&temp2, &temp1, self.field, self.block_size as usize);
        }
        state
    }

    pub fn decrypt(&self, ciphertext: &FieldElement) -> FieldElement {
        let mut state: FieldElement = ciphertext.to_vec();
        let power: u32 = ((2_i32.pow(self.field + 1) - 1) / 3) as u32;
        for round in self.round_constants.iter().rev() {
            let temp = add_field_elements_over_prime_field(&state, round, self.field, self.block_size as usize);
            state = power_over_prime_field(&temp, power, self.field, self.block_size as usize);
        }
        state
    }
}

impl fmt::Display for MiMC {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let converted_rc: &Vec<u32> = &self.round_constants.iter().map(|x| to_decimal(x)).collect();

        f.debug_struct("MiMC")
            .field("\n  block size", &self.block_size)
            .field("\n  rounds", &self.rounds)
            .field(&*format!("\n  field [2^{}]", &self.block_size), &self.field)
            .field("\n  round constants", converted_rc)
            .finish()
    }
}
