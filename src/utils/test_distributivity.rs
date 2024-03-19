use crate::utils::helpers::{add_finite_field, generate_random_bits, multiply_finite_field};

/// Distributivity: (a + b) * c = (a * c) + (b * c)
#[test]
fn distributivity_5() {
    let a = generate_random_bits(5);
    let b = generate_random_bits(5);
    let c = generate_random_bits(5);
    assert_eq!(multiply_finite_field(&add_finite_field(&a, &b), &c, 5), add_finite_field(&multiply_finite_field(&a, &c, 5), &multiply_finite_field(&b, &c, 5)));
}

#[test]
fn distributivity_8() {
    let a = generate_random_bits(8);
    let b = generate_random_bits(8);
    let c = generate_random_bits(8);
    assert_eq!(multiply_finite_field(&add_finite_field(&a, &b), &c, 8), add_finite_field(&multiply_finite_field(&a, &c, 8), &multiply_finite_field(&b, &c, 8)));
}

#[test]
fn distributivity_11() {
    let a = generate_random_bits(11);
    let b = generate_random_bits(11);
    let c = generate_random_bits(11);
    assert_eq!(multiply_finite_field(&add_finite_field(&a, &b), &c, 11), add_finite_field(&multiply_finite_field(&a, &c, 11), &multiply_finite_field(&b, &c, 11)));
}

#[test]
fn distributivity_17() {
    let a = generate_random_bits(17);
    let b = generate_random_bits(17);
    let c = generate_random_bits(17);
    assert_eq!(multiply_finite_field(&add_finite_field(&a, &b), &c, 17), add_finite_field(&multiply_finite_field(&a, &c, 17), &multiply_finite_field(&b, &c, 17)));
}

#[test]
fn distributivity_25() {
    let a = generate_random_bits(25);
    let b = generate_random_bits(25);
    let c = generate_random_bits(25);
    assert_eq!(multiply_finite_field(&add_finite_field(&a, &b), &c, 25), add_finite_field(&multiply_finite_field(&a, &c, 25), &multiply_finite_field(&b, &c, 25)));
}

#[test]
fn distributivity_31() {
    let a = generate_random_bits(31);
    let b = generate_random_bits(31);
    let c = generate_random_bits(31);
    assert_eq!(multiply_finite_field(&add_finite_field(&a, &b), &c, 31), add_finite_field(&multiply_finite_field(&a, &c, 31), &multiply_finite_field(&b, &c, 31)));
}

#[test]
fn distributivity_33() {
    let a = generate_random_bits(33);
    let b = generate_random_bits(33);
    let c = generate_random_bits(33);
    assert_eq!(multiply_finite_field(&add_finite_field(&a, &b), &c, 33), add_finite_field(&multiply_finite_field(&a, &c, 33), &multiply_finite_field(&b, &c, 33)));
}

#[test]
fn distributivity_47() {
    let a = generate_random_bits(47);
    let b = generate_random_bits(47);
    let c = generate_random_bits(47);
    assert_eq!(multiply_finite_field(&add_finite_field(&a, &b), &c, 47), add_finite_field(&multiply_finite_field(&a, &c, 47), &multiply_finite_field(&b, &c, 47)));
}

#[test]
fn distributivity_61() {
    let a = generate_random_bits(61);
    let b = generate_random_bits(61);
    let c = generate_random_bits(61);
    assert_eq!(multiply_finite_field(&add_finite_field(&a, &b), &c, 61), add_finite_field(&multiply_finite_field(&a, &c, 61), &multiply_finite_field(&b, &c, 61)));
}

#[test]
fn distributivity_83() {
    let a = generate_random_bits(83);
    let b = generate_random_bits(83);
    let c = generate_random_bits(83);
    assert_eq!(multiply_finite_field(&add_finite_field(&a, &b), &c, 83), add_finite_field(&multiply_finite_field(&a, &c, 83), &multiply_finite_field(&b, &c, 83)));
}

#[test]
fn distributivity_101() {
    let a = generate_random_bits(101);
    let b = generate_random_bits(101);
    let c = generate_random_bits(101);
    assert_eq!(multiply_finite_field(&add_finite_field(&a, &b), &c, 101), add_finite_field(&multiply_finite_field(&a, &c, 101), &multiply_finite_field(&b, &c, 101)));
}

#[test]
fn distributivity_125() {
    let a = generate_random_bits(125);
    let b = generate_random_bits(125);
    let c = generate_random_bits(125);
    assert_eq!(multiply_finite_field(&add_finite_field(&a, &b), &c, 125), add_finite_field(&multiply_finite_field(&a, &c, 125), &multiply_finite_field(&b, &c, 125)));
}

#[test]
fn distributivity_127() {
    let a = generate_random_bits(127);
    let b = generate_random_bits(127);
    let c = generate_random_bits(127);
    assert_eq!(multiply_finite_field(&add_finite_field(&a, &b), &c, 127), add_finite_field(&multiply_finite_field(&a, &c, 127), &multiply_finite_field(&b, &c, 127)));
}
