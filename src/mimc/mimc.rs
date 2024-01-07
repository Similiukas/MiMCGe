use std::fmt;
use std::fmt::Formatter;
use crate::utils::helpers::{add_finite_field, FieldElement, generate_random_bits, power_finite_field, to_decimal};

fn generate_round_constants(size: usize, block_size: u32) -> Vec<FieldElement> {
    let mut result: Vec<FieldElement> = Vec::with_capacity(size);
    result.push(vec![0; block_size as usize]); // c_0 must be 0
    for _ in 1..size {
        result.push(generate_random_bits(block_size));
    }
    result
}

pub struct MiMC {
    block_size: u32,
    rounds: usize,
    field: u32,
    round_constants: Vec<FieldElement>
}

impl MiMC {
    pub fn new(block_size: u32) -> Self {
        // For field 2 ** block_size it must be that block_size is odd
        assert_eq!(block_size % 2, 1, "Block size must be odd");
        let rounds = (block_size as f32 / 3f32.log(2.0)).ceil() as usize;
        let field = 2u32.pow(block_size);
        MiMC {
            block_size,
            rounds,
            field,
            round_constants: generate_round_constants(rounds, block_size)
        }
    }

    pub fn encrypt(&self, plaintext: &FieldElement, key: &FieldElement) -> FieldElement {
        let mut state: FieldElement = plaintext.to_vec();
        for round in 0..self.rounds {
            let mut temp= add_finite_field(&key, &self.round_constants[round]);
            temp = add_finite_field(&state, &temp);
            state = power_finite_field(&temp, 3, self.block_size);
        }
        add_finite_field(&state, &key)
    }

    pub fn decrypt(&self, ciphertext: &FieldElement, key: &FieldElement) -> FieldElement {
        let mut state: FieldElement = ciphertext.to_vec();
        let power: u32 = ((2_i32.pow(self.block_size + 1) - 1) / 3) as u32;
        for round in self.round_constants[..1].iter().chain(self.round_constants[1..].iter().rev()) {
            let mut temp = add_finite_field(&key, round);
            temp = add_finite_field(&state, &temp);
            state = power_finite_field(&temp, power, self.block_size);
        }
        add_finite_field(&state, &key)
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
