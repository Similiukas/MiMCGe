use std::fmt;
use std::fmt::Formatter;
use crate::utils::helpers::{add_finite_field, Cipher, FieldElement, gcd, generate_round_constants, square_multiply, to_decimal};

/// Finding integer *t* such that **1+t(2^n-1) / e** is an integer.
///
/// This value will be between 1 and exponent - 1. It cannot be > exponent - 1, since then we can reduce t to t = t % e.
///
/// # Panics
///
/// Panics if t is not find in range 0 t < e. Cannot do decryption then, since s, where x^s = x in GF(2^n) does not exist.
fn find_t(exponent: u128, block_size: u32) -> u128 {
    for t in 1..exponent {
        if (1 + t * (2u128.pow(block_size) - 1)) % exponent == 0 {
            return t
        }
    }
    unreachable!()
}

pub struct MiMCGn {
    exponent: u128,
    block_size: u32,
    field: u128,
    t: u128,
    rounds: usize,
    round_constants: Vec<FieldElement>
}

impl MiMCGn {
    pub fn new(exponent: u128, block_size: u32, round_reduction: Option<usize>) -> Self {
        let rounds = (block_size as f32 * 2f32.log(exponent as f32)).ceil() as usize - round_reduction.unwrap_or(0);
        MiMCGn::with_round_constants(exponent, block_size, &generate_round_constants(rounds, block_size))
    }

    pub fn with_round_constants(exponent: u128, block_size: u32, round_constants: &Vec<FieldElement>) -> Self {
        // x^n is a permutation if and only if gcd(exponent, 2^n - 1) = 1
        assert_eq!(gcd(exponent, 2u128.pow(block_size) - 1), 1);
        MiMCGn {
            exponent,
            block_size,
            field: 2u128.pow(block_size),
            t: find_t(exponent, block_size),
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
            state = square_multiply(&temp, self.exponent, self.block_size);
        }
        add_finite_field(&state, &key)
    }

    fn decrypt(&self, ciphertext: &FieldElement, key: &FieldElement) -> FieldElement {
        assert!(self.block_size <= 125, "Decryption for 2^125 field is not implemented (overflow of decryption exponent)");
        let mut state: FieldElement = ciphertext.to_vec();
        let power = (self.t * (2u128.pow(self.block_size) - 1) + 1) / self.exponent;
        for round in self.round_constants[..1].iter().chain(self.round_constants[1..].iter().rev()) {
            let mut temp = add_finite_field(&key, round);
            temp = add_finite_field(&state, &temp);
            state = square_multiply(&temp, power, self.block_size);
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
            .field("\n  t", &self.t)
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
    fn encrypt_2() {
        let block = 8;
        // 0, 79, 42, 125, 150, 10, 103, 30
        let round_constants = vec![vec![0;8], to_binary(79, block), to_binary(42, block), to_binary(125, block),
                                   to_binary(150, block), to_binary(10, block), to_binary(103, block), to_binary(30, block)];
        let cipher = MiMCGn::with_round_constants(2, block, &round_constants);
        // Plaintext 143, key 162, ciphertext 83
        assert_eq!(cipher.encrypt(&to_binary(143, block), &to_binary(162, block)), to_binary(83, block));
    }

    #[test]
    fn decrypt_2() {
        let block = 8;
        // 0, 79, 42, 125, 150, 10, 103, 30
        let round_constants = vec![vec![0;8], to_binary(79, block), to_binary(42, block), to_binary(125, block),
                                   to_binary(150, block), to_binary(10, block), to_binary(103, block), to_binary(30, block)];
        let cipher = MiMCGn::with_round_constants(2, block, &round_constants);
        // Ciphertext 83, key 162, ciphertext 143
        assert_eq!(cipher.decrypt(&to_binary(83, block), &to_binary(162, block)), to_binary(143, block));
    }

    // Same test as in with original MiMC x^3
    #[test]
    fn encrypt_3() {
        // 0, 5, 22, 16
        let round_constants = vec![vec![0;5], vec![0,0,1,0,1], vec![1,0,1,1,0], vec![1,0,0,0,0]];
        let cipher = MiMCGn::with_round_constants(3, 5, &round_constants);
        // Plaintext 15, key 29, ciphertext 7
        assert_eq!(cipher.encrypt(&vec![0,1,1,1,1], &vec![1,1,1,0,1]), vec![0,0,1,1,1]);
    }

    // Same test as in with original MiMC x^3
    #[test]
    fn decrypt_3() {
        // 0, 5, 22, 16
        let round_constants = vec![vec![0;5], vec![0,0,1,0,1], vec![1,0,1,1,0], vec![1,0,0,0,0]];
        let cipher = MiMCGn::with_round_constants(3, 5, &round_constants);
        // Ciphertext 7, key 29, plaintext 15
        assert_eq!(cipher.decrypt(&vec![0,0,1,1,1], &vec![1,1,1,0,1]), vec![0,1,1,1,1]);
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
        let block = 11;
        // 0, 501, 1136, 2029
        let round_constants = vec![vec![0;11], to_binary(501, block), to_binary(1136, block), to_binary(2029, block)];
        let cipher = MiMCGn::with_round_constants(7, block, &round_constants);
        // Plaintext 1440, key 154, ciphertext 1029
        assert_eq!(cipher.encrypt(&to_binary(1440, block), &to_binary(154, block)), to_binary(1029, block));
    }

    #[test]
    fn decrypt_7() {
        let block = 11;
        // 0, 501, 1136, 2029
        let round_constants = vec![vec![0;11], to_binary(501, block), to_binary(1136, block), to_binary(2029, block)];
        let cipher = MiMCGn::with_round_constants(7, block, &round_constants);
        // Ciphertext 1440, key 154, plaintext 1029
        assert_eq!(cipher.decrypt(&to_binary(1029, block), &to_binary(154, block)), to_binary(1440, block));
    }

    #[test]
    fn encrypt_largest_3() {
        let block = 125;
        let round_constants = vec![to_binary(0, block), to_binary(22518598525362737223782453369640873505, block), to_binary(41279493745247100212424497075042082289, block), to_binary(5107248381825823756822893588559890131, block), to_binary(17134787330113970672398341660256277911, block), to_binary(41379477521495789941276970052125042591, block), to_binary(6257383778005196176979691700957136288, block),
            to_binary(15730425326360043386545811709543603502, block), to_binary(8226002342377594062487335185734003576, block), to_binary(28306188454952390457464481796346627754, block), to_binary(26414913368863978723687991365005809983, block), to_binary(26190354078703677320956332953613612927, block), to_binary(18739727043509433097578378796840726609, block), to_binary(13508989441608251041007833449921519880, block),
            to_binary(38381964355223679481018416380889084556, block), to_binary(31161814714623194397408346364489716770, block), to_binary(32934457635429160815107860710513742589, block), to_binary(26741680603146037214397872553555190766, block), to_binary(3107426661571313149565869224512046096, block), to_binary(40198687631088615974585082090700986998, block), to_binary(5476969655650457568097010922063797835, block),
            to_binary(41461862553193984450999192143908749498, block), to_binary(12327403700919129259788074806075185938, block), to_binary(4688292551730950202753753342430543751, block), to_binary(21905517153202268016662264333486113258, block), to_binary(40298766568895148109296556534951900641, block), to_binary(7431960764102833751099153944386722509, block), to_binary(41597672896597230345689763217693153997, block),
            to_binary(39356322169560908611575791106241040910, block), to_binary(30915125800993868940638839796466169844, block), to_binary(7707638768011105844732221982838828339, block), to_binary(33174363695575065474040085269150833698, block), to_binary(36982357710749791314228438301434523725, block), to_binary(19199018493051685511967287122738540215, block), to_binary(40814375592240133474309999032349509611, block),
            to_binary(5485505981129994479439989387629513167, block), to_binary(18200596548306610510030168557287998566, block), to_binary(28074850514512958691340130478009550182, block), to_binary(37920797015914161870459494996451215556, block), to_binary(21050846053341268473358818826145389604, block), to_binary(34978730106863042178059482795274579996, block), to_binary(16640784828529582217405800935335623427, block),
            to_binary(14114032100252213628432831366356214055, block), to_binary(23449122208521263582712105148016793814, block), to_binary(42455541929312132737715909265759290100, block), to_binary(12488447674234989749856990624263179660, block), to_binary(38271021498491585088981180160936744832, block), to_binary(21273202135632886455021824132386351032, block), to_binary(42199420419474960633636624546229779561, block),
            to_binary(6891303654180050448666157782054320675, block), to_binary(26025095744652179736363571671131702415, block), to_binary(19564141832034057701144679755146673853, block), to_binary(17249764329103120432139810318146750750, block), to_binary(15217432852446460228639605851859388557, block), to_binary(12878646208280817961286200191382358430, block), to_binary(1087373804502272802957995835970161013, block),
            to_binary(35797366831230768258722877554563483929, block), to_binary(11960160329115209900930986940476452226, block), to_binary(18896113138883958064421435497817475600, block), to_binary(2158883091397684885863752555626906586, block), to_binary(25138877804784333102186542262583242232, block), to_binary(20021012291658384193256540135270439351, block), to_binary(1989622572028916443533248947617776030, block),
            to_binary(7205033936298891818499388494006800195, block), to_binary(17497161803080260119745458385898769753, block), to_binary(37578834402327662055394134160005729299, block), to_binary(21240978322657773079645023711625750132, block), to_binary(37236911606149191099742906052946416063, block), to_binary(42081022960725496538871605827198703624, block), to_binary(18684432868189542060755476402097972613, block),
            to_binary(5883437725093564569545747673014498194, block), to_binary(27031668972804118613304002349961229405, block), to_binary(9831444421268533385150305261192003966, block), to_binary(14151207877237846092529077669506552902, block), to_binary(31004815350678692215653382429727003270, block), to_binary(8246329830944397747374334927475534030, block), to_binary(16610515120689042289211774724145727829, block),
            to_binary(2072040885702662349062282296309023930, block), to_binary(8381710917421248726076725161839749053, block)];
        let cipher = MiMCGn::with_round_constants(3, block, &round_constants);
        assert_eq!(cipher.encrypt(&to_binary(9468632022148749579697753766856589000, block), &to_binary(8467268564892985217747340741738563498, block)), to_binary(8019515472302977383992575657936144960, block));
    }

    #[test]
    fn decrypt_largest_3() {
        let block = 125;
        let round_constants = vec![to_binary(0, block), to_binary(22518598525362737223782453369640873505, block), to_binary(41279493745247100212424497075042082289, block), to_binary(5107248381825823756822893588559890131, block), to_binary(17134787330113970672398341660256277911, block), to_binary(41379477521495789941276970052125042591, block), to_binary(6257383778005196176979691700957136288, block),
                                   to_binary(15730425326360043386545811709543603502, block), to_binary(8226002342377594062487335185734003576, block), to_binary(28306188454952390457464481796346627754, block), to_binary(26414913368863978723687991365005809983, block), to_binary(26190354078703677320956332953613612927, block), to_binary(18739727043509433097578378796840726609, block), to_binary(13508989441608251041007833449921519880, block),
                                   to_binary(38381964355223679481018416380889084556, block), to_binary(31161814714623194397408346364489716770, block), to_binary(32934457635429160815107860710513742589, block), to_binary(26741680603146037214397872553555190766, block), to_binary(3107426661571313149565869224512046096, block), to_binary(40198687631088615974585082090700986998, block), to_binary(5476969655650457568097010922063797835, block),
                                   to_binary(41461862553193984450999192143908749498, block), to_binary(12327403700919129259788074806075185938, block), to_binary(4688292551730950202753753342430543751, block), to_binary(21905517153202268016662264333486113258, block), to_binary(40298766568895148109296556534951900641, block), to_binary(7431960764102833751099153944386722509, block), to_binary(41597672896597230345689763217693153997, block),
                                   to_binary(39356322169560908611575791106241040910, block), to_binary(30915125800993868940638839796466169844, block), to_binary(7707638768011105844732221982838828339, block), to_binary(33174363695575065474040085269150833698, block), to_binary(36982357710749791314228438301434523725, block), to_binary(19199018493051685511967287122738540215, block), to_binary(40814375592240133474309999032349509611, block),
                                   to_binary(5485505981129994479439989387629513167, block), to_binary(18200596548306610510030168557287998566, block), to_binary(28074850514512958691340130478009550182, block), to_binary(37920797015914161870459494996451215556, block), to_binary(21050846053341268473358818826145389604, block), to_binary(34978730106863042178059482795274579996, block), to_binary(16640784828529582217405800935335623427, block),
                                   to_binary(14114032100252213628432831366356214055, block), to_binary(23449122208521263582712105148016793814, block), to_binary(42455541929312132737715909265759290100, block), to_binary(12488447674234989749856990624263179660, block), to_binary(38271021498491585088981180160936744832, block), to_binary(21273202135632886455021824132386351032, block), to_binary(42199420419474960633636624546229779561, block),
                                   to_binary(6891303654180050448666157782054320675, block), to_binary(26025095744652179736363571671131702415, block), to_binary(19564141832034057701144679755146673853, block), to_binary(17249764329103120432139810318146750750, block), to_binary(15217432852446460228639605851859388557, block), to_binary(12878646208280817961286200191382358430, block), to_binary(1087373804502272802957995835970161013, block),
                                   to_binary(35797366831230768258722877554563483929, block), to_binary(11960160329115209900930986940476452226, block), to_binary(18896113138883958064421435497817475600, block), to_binary(2158883091397684885863752555626906586, block), to_binary(25138877804784333102186542262583242232, block), to_binary(20021012291658384193256540135270439351, block), to_binary(1989622572028916443533248947617776030, block),
                                   to_binary(7205033936298891818499388494006800195, block), to_binary(17497161803080260119745458385898769753, block), to_binary(37578834402327662055394134160005729299, block), to_binary(21240978322657773079645023711625750132, block), to_binary(37236911606149191099742906052946416063, block), to_binary(42081022960725496538871605827198703624, block), to_binary(18684432868189542060755476402097972613, block),
                                   to_binary(5883437725093564569545747673014498194, block), to_binary(27031668972804118613304002349961229405, block), to_binary(9831444421268533385150305261192003966, block), to_binary(14151207877237846092529077669506552902, block), to_binary(31004815350678692215653382429727003270, block), to_binary(8246329830944397747374334927475534030, block), to_binary(16610515120689042289211774724145727829, block),
                                   to_binary(2072040885702662349062282296309023930, block), to_binary(8381710917421248726076725161839749053, block)];
        let cipher = MiMCGn::with_round_constants(3, block, &round_constants);
        assert_eq!(cipher.decrypt(&to_binary(8019515472302977383992575657936144960, block), &to_binary(8467268564892985217747340741738563498, block)), to_binary(9468632022148749579697753766856589000, block));
    }

    #[test]
    fn encrypt_largest_17() {
        let block = 125;
        let round_constants = vec![to_binary(0, block), to_binary(22518598525362737223782453369640873505, block), to_binary(41279493745247100212424497075042082289, block), to_binary(5107248381825823756822893588559890131, block), to_binary(17134787330113970672398341660256277911, block), to_binary(41379477521495789941276970052125042591, block), to_binary(6257383778005196176979691700957136288, block),
                                   to_binary(15730425326360043386545811709543603502, block), to_binary(8226002342377594062487335185734003576, block), to_binary(28306188454952390457464481796346627754, block), to_binary(26414913368863978723687991365005809983, block), to_binary(26190354078703677320956332953613612927, block), to_binary(18739727043509433097578378796840726609, block), to_binary(13508989441608251041007833449921519880, block),
                                   to_binary(38381964355223679481018416380889084556, block), to_binary(31161814714623194397408346364489716770, block), to_binary(32934457635429160815107860710513742589, block), to_binary(26741680603146037214397872553555190766, block), to_binary(3107426661571313149565869224512046096, block), to_binary(40198687631088615974585082090700986998, block), to_binary(5476969655650457568097010922063797835, block),
                                   to_binary(41461862553193984450999192143908749498, block), to_binary(12327403700919129259788074806075185938, block), to_binary(4688292551730950202753753342430543751, block), to_binary(21905517153202268016662264333486113258, block), to_binary(40298766568895148109296556534951900641, block), to_binary(7431960764102833751099153944386722509, block), to_binary(41597672896597230345689763217693153997, block),
                                   to_binary(39356322169560908611575791106241040910, block), to_binary(30915125800993868940638839796466169844, block), to_binary(7707638768011105844732221982838828339, block)];
        let cipher = MiMCGn::with_round_constants(17, block, &round_constants);
        assert_eq!(cipher.encrypt(&to_binary(3539930619944888682700143720924760077, block), &to_binary(2072040885702662349062282296309023930, block)), to_binary(1985296284827060896312693521822282292, block));
    }

    #[test]
    fn decrypt_largest_17() {
        let block = 125;
        let round_constants = vec![to_binary(0, block), to_binary(22518598525362737223782453369640873505, block), to_binary(41279493745247100212424497075042082289, block), to_binary(5107248381825823756822893588559890131, block), to_binary(17134787330113970672398341660256277911, block), to_binary(41379477521495789941276970052125042591, block), to_binary(6257383778005196176979691700957136288, block),
                                   to_binary(15730425326360043386545811709543603502, block), to_binary(8226002342377594062487335185734003576, block), to_binary(28306188454952390457464481796346627754, block), to_binary(26414913368863978723687991365005809983, block), to_binary(26190354078703677320956332953613612927, block), to_binary(18739727043509433097578378796840726609, block), to_binary(13508989441608251041007833449921519880, block),
                                   to_binary(38381964355223679481018416380889084556, block), to_binary(31161814714623194397408346364489716770, block), to_binary(32934457635429160815107860710513742589, block), to_binary(26741680603146037214397872553555190766, block), to_binary(3107426661571313149565869224512046096, block), to_binary(40198687631088615974585082090700986998, block), to_binary(5476969655650457568097010922063797835, block),
                                   to_binary(41461862553193984450999192143908749498, block), to_binary(12327403700919129259788074806075185938, block), to_binary(4688292551730950202753753342430543751, block), to_binary(21905517153202268016662264333486113258, block), to_binary(40298766568895148109296556534951900641, block), to_binary(7431960764102833751099153944386722509, block), to_binary(41597672896597230345689763217693153997, block),
                                   to_binary(39356322169560908611575791106241040910, block), to_binary(30915125800993868940638839796466169844, block), to_binary(7707638768011105844732221982838828339, block)];
        let cipher = MiMCGn::with_round_constants(17, block, &round_constants);
        assert_eq!(cipher.decrypt(&to_binary(1985296284827060896312693521822282292, block), &to_binary(2072040885702662349062282296309023930, block)), to_binary(3539930619944888682700143720924760077, block));
    }
}
