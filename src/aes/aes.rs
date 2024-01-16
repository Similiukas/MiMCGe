use crate::utils::helpers::{Cipher, FieldElement, to_binary, to_decimal};
use aes::Aes128;
use aes::cipher::generic_array::GenericArray;
use aes::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};

/// Cuts bit array element to u8 element array
///
/// # Example
/// ```
/// assert_eq!(to_u8_slices(vec![1,0,0,1,0,1,0,0, 0,0,0,0,0,0,1,0]), vec![148, 2])
/// ```
fn to_u8_slices(elem: &FieldElement) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::with_capacity(16);
    for i in 0..16 {
        result.push(to_decimal(&elem[(i * 8)..((i * 8) + 8)]) as u8);
    }
    result
}

fn from_u8_slices(elem: &Vec<u8>) -> FieldElement {
    let mut result = Vec::with_capacity(128);
    for i in 0..16 {
        result.append(&mut to_binary(elem[i] as u128, 8));
    }
    result
}

pub struct AES {}

impl Cipher for AES {
    fn encrypt(&self, plaintext: &FieldElement, key: &FieldElement) -> FieldElement {
        let _key = to_u8_slices(&key);
        let cipher = Aes128::new(GenericArray::from_slice(&*_key));
        let mut binding = to_u8_slices(&plaintext);
        let _plaintext = GenericArray::from_mut_slice(&mut binding[0..16]);
        cipher.encrypt_block(_plaintext);

        from_u8_slices(&_plaintext.to_vec())
    }

    fn decrypt(&self, ciphertext: &FieldElement, key: &FieldElement) -> FieldElement {
        let _key = to_u8_slices(&key);
        let cipher = Aes128::new(GenericArray::from_slice(&*_key));
        let mut binding = to_u8_slices(&ciphertext);
        let _ciphertext = GenericArray::from_mut_slice(&mut binding[0..16]);
        cipher.decrypt_block(_ciphertext);

        from_u8_slices(&_ciphertext.to_vec())
    }
}