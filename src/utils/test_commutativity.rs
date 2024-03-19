use crate::utils::helpers::{add_finite_field, generate_random_bits, multiply_finite_field};

/// Commutativity: a + b = b + a and a * b = b * a
#[test]
fn commutativity_5() {
    let a = generate_random_bits(5);
    let b = generate_random_bits(5);
    assert_eq!(add_finite_field(&a, &b), add_finite_field(&b, &a));
    assert_eq!(multiply_finite_field(&a, &b, 5), multiply_finite_field(&b, &a, 5));
}

#[test]
fn commutativity_8() {
    let a = generate_random_bits(8);
    let b = generate_random_bits(8);
    assert_eq!(add_finite_field(&a, &b), add_finite_field(&b, &a));
    assert_eq!(multiply_finite_field(&a, &b, 8), multiply_finite_field(&b, &a, 8));
}

#[test]
fn commutativity_11() {
    let a = generate_random_bits(11);
    let b = generate_random_bits(11);
    assert_eq!(add_finite_field(&a, &b), add_finite_field(&b, &a));
    assert_eq!(multiply_finite_field(&a, &b, 11), multiply_finite_field(&b, &a, 11));
}

#[test]
fn commutativity_17() {
    let a = generate_random_bits(17);
    let b = generate_random_bits(17);
    assert_eq!(add_finite_field(&a, &b), add_finite_field(&b, &a));
    assert_eq!(multiply_finite_field(&a, &b, 17), multiply_finite_field(&b, &a, 17));
}

#[test]
fn commutativity_25() {
    let a = generate_random_bits(25);
    let b = generate_random_bits(25);
    assert_eq!(add_finite_field(&a, &b), add_finite_field(&b, &a));
    assert_eq!(multiply_finite_field(&a, &b, 25), multiply_finite_field(&b, &a, 25));
}

#[test]
fn commutativity_31() {
    let a = generate_random_bits(31);
    let b = generate_random_bits(31);
    assert_eq!(add_finite_field(&a, &b), add_finite_field(&b, &a));
    assert_eq!(multiply_finite_field(&a, &b, 31), multiply_finite_field(&b, &a, 31));
}

#[test]
fn commutativity_33() {
    let a = generate_random_bits(33);
    let b = generate_random_bits(33);
    assert_eq!(add_finite_field(&a, &b), add_finite_field(&b, &a));
    assert_eq!(multiply_finite_field(&a, &b, 33), multiply_finite_field(&b, &a, 33));
}

#[test]
fn commutativity_47() {
    let a = generate_random_bits(47);
    let b = generate_random_bits(47);
    assert_eq!(add_finite_field(&a, &b), add_finite_field(&b, &a));
    assert_eq!(multiply_finite_field(&a, &b, 47), multiply_finite_field(&b, &a, 47));
}

#[test]
fn commutativity_61() {
    let a = generate_random_bits(61);
    let b = generate_random_bits(61);
    assert_eq!(add_finite_field(&a, &b), add_finite_field(&b, &a));
    assert_eq!(multiply_finite_field(&a, &b, 61), multiply_finite_field(&b, &a, 61));
}

#[test]
fn commutativity_83() {
    let a = generate_random_bits(83);
    let b = generate_random_bits(83);
    assert_eq!(add_finite_field(&a, &b), add_finite_field(&b, &a));
    assert_eq!(multiply_finite_field(&a, &b, 83), multiply_finite_field(&b, &a, 83));
}

#[test]
fn commutativity_101() {
    let a = generate_random_bits(101);
    let b = generate_random_bits(101);
    assert_eq!(add_finite_field(&a, &b), add_finite_field(&b, &a));
    assert_eq!(multiply_finite_field(&a, &b, 101), multiply_finite_field(&b, &a, 101));
}

#[test]
fn commutativity_125() {
    let a = generate_random_bits(125);
    let b = generate_random_bits(125);
    assert_eq!(add_finite_field(&a, &b), add_finite_field(&b, &a));
    assert_eq!(multiply_finite_field(&a, &b, 125), multiply_finite_field(&b, &a, 125));
}

#[test]
fn commutativity_127() {
    let a = generate_random_bits(127);
    let b = generate_random_bits(127);
    assert_eq!(add_finite_field(&a, &b), add_finite_field(&b, &a));
    assert_eq!(multiply_finite_field(&a, &b, 127), multiply_finite_field(&b, &a, 127));
}
