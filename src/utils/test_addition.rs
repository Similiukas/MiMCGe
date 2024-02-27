use crate::utils::helpers::{add_finite_field, generate_random_bits, to_binary};

// Tests 0 + a = a in GF(2^n)
#[test]
fn finite_field_additive_identity_5() {
    let a = generate_random_bits(5);
    assert_eq!(add_finite_field(&vec![0,0,0,0,0], &a), a)
}

#[test]
fn finite_field_additive_identity_8() {
    let a = generate_random_bits(8);
    assert_eq!(add_finite_field(&vec![0,0,0,0,0,0,0,0], &a), a)
}

#[test]
fn finite_field_additive_identity_11() {
    let a = generate_random_bits(11);
    assert_eq!(add_finite_field(&vec![0,0,0,0,0,0,0,0,0,0,0], &a), a)
}

#[test]
fn finite_field_additive_identity_17() {
    let a = generate_random_bits(17);
    assert_eq!(add_finite_field(&vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0], &a), a)
}

#[test]
fn finite_field_additive_identity_31() {
    let a = generate_random_bits(31);
    assert_eq!(add_finite_field(&to_binary(0, 31), &a), a)
}

#[test]
fn finite_field_additive_identity_47() {
    let a = generate_random_bits(47);
    assert_eq!(add_finite_field(&to_binary(0, 47), &a), a)
}

#[test]
fn finite_field_additive_identity_61() {
    let a = generate_random_bits(61);
    assert_eq!(add_finite_field(&to_binary(0,61), &a), a)
}

#[test]
fn finite_field_additive_identity_83() {
    let a = generate_random_bits(83);
    assert_eq!(add_finite_field(&to_binary(0, 83), &a), a)
}

#[test]
fn finite_field_additive_identity_101() {
    let a = generate_random_bits(101);
    assert_eq!(add_finite_field(&to_binary(0, 101), &a), a)
}

#[test]
fn finite_field_additive_identity_125() {
    let a = generate_random_bits(125);
    assert_eq!(add_finite_field(&to_binary(0,125), &a), a)
}

#[test]
fn finite_field_additive_identity_127() {
    let a = generate_random_bits(127);
    assert_eq!(add_finite_field(&to_binary(0,127), &a), a)
}
