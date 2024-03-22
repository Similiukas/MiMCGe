use crate::utils::helpers::{generate_random_bits, square_multiply, to_binary, to_decimal};

// Tests identity: for 1^n = 1 for any n
#[test]
fn finite_field_multiplicative_identity_5() {
    assert_eq!(square_multiply(&vec![0,0,0,0,1], 5,5), vec![0,0,0,0,1]);
}

#[test]
fn finite_field_multiplicative_identity_8() {
    assert_eq!(square_multiply(&vec![0,0,0,0,0,0,0,1], 5,8), vec![0,0,0,0,0,0,0,1]);
}

#[test]
fn finite_field_multiplicative_identity_11() {
    assert_eq!(square_multiply(&vec![0,0,0,0,0,0,0,0,0,0,1], 5,11), vec![0,0,0,0,0,0,0,0,0,0,1]);
}

#[test]
fn finite_field_multiplicative_identity_17() {
    assert_eq!(square_multiply(&vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1], 5,17), vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1]);
}

#[test]
fn finite_field_multiplicative_identity_31() {
    assert_eq!(square_multiply(&to_binary(1,31), 5,31), to_binary(1, 31));
}

#[test]
fn finite_field_multiplicative_identity_33() {
    assert_eq!(square_multiply(&to_binary(1,33), 5,33), to_binary(1, 33));
}

#[test]
fn finite_field_multiplicative_identity_47() {
    assert_eq!(square_multiply(&to_binary(1,47), 5,47), to_binary(1, 47));
}

#[test]
fn finite_field_multiplicative_identity_61() {
    assert_eq!(square_multiply(&to_binary(1,61), 5,61), to_binary(1, 61));
}

#[test]
fn finite_field_multiplicative_identity_83() {
    assert_eq!(square_multiply(&to_binary(1,83), 5,83), to_binary(1, 83));
}

#[test]
fn finite_field_multiplicative_identity_101() {
    assert_eq!(square_multiply(&to_binary(1,101), 5,101), to_binary(1, 101));
}

#[test]
fn finite_field_multiplicative_identity_125() {
    assert_eq!(square_multiply(&to_binary(1,125), 5,125), to_binary(1, 125));
}

#[test]
fn finite_field_multiplicative_identity_127() {
    assert_eq!(square_multiply(&to_binary(1,127), 5,127), to_binary(1, 127));
}

// ---------------------------------------------
// Tests for x^q = x in GF(q). In this case, q = 2^n
#[test]
fn finite_field_multiplicative_inverse_5() {
    let a = generate_random_bits(5);
    assert_eq!(square_multiply(&a, 2u128.pow(5), 5), a)
}

#[test]
fn finite_field_multiplicative_inverse_8() {
    let a = generate_random_bits(8);
    assert_eq!(square_multiply(&a, 2u128.pow(8), 8), a)
}

#[test]
fn finite_field_multiplicative_inverse_11() {
    let a = generate_random_bits(11);
    assert_eq!(square_multiply(&a, 2u128.pow(11), 11), a)
}

#[test]
fn finite_field_multiplicative_inverse_17() {
    let a = generate_random_bits(17);
    assert_eq!(square_multiply(&a, 2u128.pow(17), 17), a)
}

#[test]
fn finite_field_multiplicative_inverse_31() {
    let a = generate_random_bits(31);
    assert_eq!(square_multiply(&a, 2u128.pow(31), 31), a)
}

#[test]
fn finite_field_multiplicative_inverse_33() {
    let a = generate_random_bits(33);
    assert_eq!(square_multiply(&a, 2u128.pow(33), 33), a)
}

#[test]
fn finite_field_multiplicative_inverse_47() {
    let a = generate_random_bits(47);
    assert_eq!(square_multiply(&a, 2u128.pow(47), 47), a)
}

#[test]
fn finite_field_multiplicative_inverse_61() {
    let a = generate_random_bits(61);
    assert_eq!(square_multiply(&a, 2u128.pow(61), 61), a)
}

#[test]
fn finite_field_multiplicative_inverse_83() {
    let a = generate_random_bits(83);
    assert_eq!(square_multiply(&a, 2u128.pow(83), 83), a)
}

#[test]
fn finite_field_multiplicative_inverse_101() {
    let a = generate_random_bits(101);
    assert_eq!(square_multiply(&a, 2u128.pow(101), 101), a)
}

#[test]
fn finite_field_multiplicative_inverse_125() {
    let a = generate_random_bits(125);
    assert_eq!(square_multiply(&a, 2u128.pow(125), 125), a)
}

#[test]
fn finite_field_multiplicative_inverse_127() {
    let a = generate_random_bits(127);
    assert_eq!(square_multiply(&a, 2u128.pow(127), 127), a)
}
