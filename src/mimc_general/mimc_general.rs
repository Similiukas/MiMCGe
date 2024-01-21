use std::fmt;
use std::fmt::Formatter;
use crate::utils::helpers::{add_finite_field, Cipher, FieldElement, gcd, generate_round_constants, power_finite_field, to_decimal};

pub struct MiMCGn {
    exponent: usize,
    block_size: u32,
    field: u128,
    t: usize,
    rounds: usize,
    round_constants: Vec<FieldElement>
}

impl MiMCGn {
    pub fn new(exponent: usize, block_size: u32) -> Self {
        let rounds = (block_size as f32 * 2f32.log(exponent as f32)).ceil() as usize;
        MiMCGn::with_round_constants(exponent, block_size, &generate_round_constants(rounds, block_size))
    }

    pub fn with_round_constants(exponent: usize, block_size: u32, round_constants: &Vec<FieldElement>) -> Self {
        // x^n is a permutation if and only if gcd(exponent, 2^n - 1) = 1
        assert_eq!(gcd(exponent, 2usize.pow(block_size) - 1), 1);
        MiMCGn {
            exponent,
            block_size,
            field: 2u128.pow(block_size),
            t: exponent - (block_size % (exponent - 1) as u32) as usize,
            rounds: round_constants.len(),
            round_constants: round_constants.to_vec()
        }
    }
}

impl Cipher for MiMCGn {
    fn encrypt(&self, plaintext: &FieldElement, key: &FieldElement) -> FieldElement {
        let mut state: FieldElement = plaintext.to_vec();
        for round in 0..self.rounds {
            let mut temp= add_finite_field(&key, &self.round_constants[round]);
            temp = add_finite_field(&state, &temp);
            state = power_finite_field(&temp, self.exponent, self.block_size);
        }
        add_finite_field(&state, &key)
    }

    fn decrypt(&self, ciphertext: &FieldElement, key: &FieldElement) -> FieldElement {
        assert!(self.block_size <= 31, "Decryption for 2^31 field is not implemented (too slow)");
        let mut state: FieldElement = ciphertext.to_vec();
        let power = (self.t * (2usize.pow(self.block_size) - 1) + 1) / self.exponent;
        for round in self.round_constants[..1].iter().chain(self.round_constants[1..].iter().rev()) {
            let mut temp = add_finite_field(&key, round);
            temp = add_finite_field(&state, &temp);
            state = power_finite_field(&temp, power, self.block_size);
        }
        add_finite_field(&state, &key)
    }
}

impl fmt::Display for MiMCGn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let converted_rc: &Vec<u128> = &self.round_constants.iter().map(|x| to_decimal(x)).collect();

        f.debug_struct("MiMC")
            .field("\n  exponent", &self.exponent)
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
    use crate::mimc_general::mimc_general::MiMCGn;
    use crate::utils::helpers::{Cipher, to_binary};

    #[test]
    fn encrypt_3() {
        // 0, 25, 25, 12
        let round_constants = vec![vec![0;5], vec![1,1,0,0,1], vec![1,1,0,0,1], vec![0,1,1,0,0]];
        let cipher = MiMCGn::with_round_constants(3, 5, &round_constants);
        // Plaintext 15, key 15, ciphertext 6
        assert_eq!(cipher.encrypt(&vec![0,1,1,1,1], &vec![0,1,1,1,1]), vec![0,0,1,1,0]);
    }

    #[test]
    fn decrypt_3() {
        // 0, 25, 25, 12
        let round_constants = vec![vec![0;5], vec![1,1,0,0,1], vec![1,1,0,0,1], vec![0,1,1,0,0]];
        let cipher = MiMCGn::with_round_constants(3, 5, &round_constants);
        // Ciphertext 15, key 15, plaintext 6
        assert_eq!(cipher.decrypt(&vec![0,0,1,1,0], &vec![0,1,1,1,1]), vec![0,1,1,1,1]);
    }

    #[test]
    fn encrypt_5() {
        // 0, 30, 11
        let round_constants = vec![vec![0;5], vec![1,1,1,1,0], vec![0,1,0,1,1]];
        let cipher = MiMCGn::with_round_constants(5, 5, &round_constants);
        // Plaintext 16, key 23, ciphertext 12
        assert_eq!(cipher.encrypt(&vec![1,0,0,0,0], &vec![1,0,1,1,1]), vec![0,1,1,0,0]);
    }

    #[test]
    fn decrypt_5() {
        // 0, 30, 11
        let round_constants = vec![vec![0;5], vec![1,1,1,1,0], vec![0,1,0,1,1]];
        let cipher = MiMCGn::with_round_constants(5, 5, &round_constants);
        // Ciphertext 12, key 23, plaintext 16
        assert_eq!(cipher.decrypt(&vec![0,1,1,0,0], &vec![1,0,1,1,1]), vec![1,0,0,0,0]);
    }

    #[test]
    fn encrypt_7() {
        let block_size = 11;
        // 0, 501, 1136, 2029
        let round_constants = vec![vec![0;11], to_binary(501, block_size), to_binary(1136, block_size), to_binary(2029, block_size)];
        let cipher = MiMCGn::with_round_constants(7, block_size, &round_constants);
        // Plaintext 1440, key 154, ciphertext 1029
        assert_eq!(cipher.encrypt(&to_binary(1440, block_size), &to_binary(154, block_size)), to_binary(1029, block_size));
    }

    #[test]
    fn decrypt_7() {
        let block_size = 11;
        // 0, 501, 1136, 2029
        let round_constants = vec![vec![0;11], to_binary(501, block_size), to_binary(1136, block_size), to_binary(2029, block_size)];
        let cipher = MiMCGn::with_round_constants(7, block_size, &round_constants);
        // Ciphertext 1440, key 154, plaintext 1029
        assert_eq!(cipher.decrypt(&to_binary(1029, block_size), &to_binary(154, block_size)), to_binary(1440, block_size));
    }
}
