use std::fmt;
use std::fmt::Formatter;
use crate::utils::helpers::{add_finite_field, Cipher, FieldElement, gcd, generate_round_constants, power_finite_field, to_decimal};

// TODO: could possibly make this more generic into any x^n polynomial, just need to assure GCD(n, field - 1) = 1
pub struct MiMC5 {
    block_size: u32,
    field: u128,
    t: usize,
    rounds: usize,
    round_constants: Vec<FieldElement>
}

impl MiMC5 {
    pub fn new(block_size: u32) -> Self {
        let rounds = (block_size as f32 / 5f32.log(2.0)).ceil() as usize;
        MiMC5::with_round_constants(block_size, &generate_round_constants(rounds, block_size))
    }

    pub fn with_round_constants(block_size: u32, round_constants: &Vec<FieldElement>) -> Self {
        // It's a permutation if and only if gcd(5, 2^n - 1) = 1
        assert_eq!(gcd(5, 2usize.pow(block_size) - 1), 1);
        MiMC5 {
            block_size,
            field: 2u128.pow(block_size),
            t: (5 - block_size % 4) as usize,
            rounds: round_constants.len(),
            round_constants: round_constants.to_vec()
        }
    }
}

impl Cipher for MiMC5 {
    fn encrypt(&self, plaintext: &FieldElement, key: &FieldElement) -> FieldElement {
        let mut state: FieldElement = plaintext.to_vec();
        for round in 0..self.rounds {
            let mut temp= add_finite_field(&key, &self.round_constants[round]);
            temp = add_finite_field(&state, &temp);
            state = power_finite_field(&temp, 5, self.block_size);
        }
        add_finite_field(&state, &key)
    }

    fn decrypt(&self, ciphertext: &FieldElement, key: &FieldElement) -> FieldElement {
        assert!(self.block_size <= 31, "Decryption for 2^31 field is not implemented (too slow)");
        let mut state: FieldElement = ciphertext.to_vec();
        let power = (self.t * (2usize.pow(self.block_size) - 1) + 1) / 5;
        for round in self.round_constants[..1].iter().chain(self.round_constants[1..].iter().rev()) {
            let mut temp = add_finite_field(&key, round);
            temp = add_finite_field(&state, &temp);
            state = power_finite_field(&temp, power, self.block_size);
        }
        add_finite_field(&state, &key)
    }
}

impl fmt::Display for MiMC5 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let converted_rc: &Vec<u128> = &self.round_constants.iter().map(|x| to_decimal(x)).collect();

        f.debug_struct("MiMC")
            .field("\n  block size", &self.block_size)
            .field("\n  rounds", &self.rounds)
            .field("\n t", &self.t)
            .field(&*format!("\n  field [2^{}]", &self.block_size), &self.field)
            .field("\n  round constants", converted_rc)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::mimc_5::mimc_5::MiMC5;
    use crate::utils::helpers::Cipher;

    #[test]
    fn encrypt_small() {
        // 0, 30, 11
        let round_constants = vec![vec![0;5], vec![1,1,1,1,0], vec![0,1,0,1,1]];
        let cipher = MiMC5::with_round_constants(5, &round_constants);
        // Plaintext 16, key 23, ciphertext 12
        assert_eq!(cipher.encrypt(&vec![1,0,0,0,0], &vec![1,0,1,1,1]), vec![0,1,1,0,0]);
    }

    #[test]
    fn decrypt_small() {
        // 0, 30, 11
        let round_constants = vec![vec![0;5], vec![1,1,1,1,0], vec![0,1,0,1,1]];
        let cipher = MiMC5::with_round_constants(5, &round_constants);
        // Ciphertext 12, key 23, plaintext 16
        assert_eq!(cipher.decrypt(&vec![0,1,1,0,0], &vec![1,0,1,1,1]), vec![1,0,0,0,0]);
    }
}
