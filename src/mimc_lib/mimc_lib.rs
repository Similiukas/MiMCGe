use std::fmt;
use std::fmt::Formatter;
use g2p::{g2p, GaloisField};
use crate::utils::helpers::{generate_random_element, to_decimal};

static BLOCK_SIZE: u32 = 17;

g2p!(GF, 17);

fn generate_round_constants(size: usize, field: u32) -> Vec<GF> {
    let mut result: Vec<GF> = Vec::with_capacity(size);
    result.push(GF::ZERO); // c_0 must be 0
    for _ in 1..size {
        result.push((to_decimal(&generate_random_element(field as u128)) as u32).into());
    }
    result
}

pub struct MiMCLib {
    rounds: usize,
    field: u32,
    round_constants: Vec<GF>
}

impl MiMCLib {
    pub fn new() -> Self {
        // For field 2 ^ block_size it must be that block_size is odd
        assert_eq!(BLOCK_SIZE % 2, 1, "Block size must be odd");
        let rounds = (BLOCK_SIZE as f32 / 3f32.log(2.0)).ceil() as usize;
        let field = 2u32.pow(BLOCK_SIZE);
        MiMCLib {
            rounds,
            field,
            round_constants: generate_round_constants(rounds, field)
        }
    }

    pub fn encrypt(&self, plaintext: &GF) -> GF{
        let mut state = plaintext.to_owned();
        for &round in &self.round_constants {
            state = (state + round).pow(3);
        }
        state
    }

    pub fn decrypt(&self, ciphertext: &GF) -> GF {
        let mut state = *ciphertext;
        let power = ((2_i32.pow(BLOCK_SIZE + 1) - 1) / 3) as usize;
        for &round in self.round_constants[..1].iter().chain(self.round_constants[1..].iter().rev()) {
            state = (state + round).pow(power);
        }
        state
    }
}

impl fmt::Display for MiMCLib {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        // let converted_rc: &Vec<u32> = &self.round_constants.iter().map(|x| to_decimal(x)).collect();

        f.debug_struct("MiMC using g2p library")
            .field("\n  block size", &BLOCK_SIZE)
            .field("\n  rounds", &self.rounds)
            .field(&*format!("\n  field [2^{}]", BLOCK_SIZE), &self.field)
            .field("\n  round constants", &self.round_constants)
            .finish()
    }
}
