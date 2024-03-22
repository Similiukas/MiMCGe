use crate::utils::helpers::{add_finite_field, generate_random_bits, multiply_finite_field};

/// Associativity: (a + b) + c = a + (b + c) and (a * b) * c = a * (b * c)
#[test]
fn associativity_5() {
    let a = generate_random_bits(5);
    let b = generate_random_bits(5);
    let c = generate_random_bits(5);
    assert_eq!(add_finite_field(&add_finite_field(&a, &b), &c), add_finite_field(&a, &add_finite_field(&b, &c)));
    assert_eq!(multiply_finite_field(&multiply_finite_field(&a, &b, 5), &c, 5), multiply_finite_field(&a, &multiply_finite_field(&b, &c, 5), 5));
}

#[test]
fn associativity_8() {
    let a = generate_random_bits(8);
    let b = generate_random_bits(8);
    let c = generate_random_bits(8);
    assert_eq!(add_finite_field(&add_finite_field(&a, &b), &c), add_finite_field(&a, &add_finite_field(&b, &c)));
    assert_eq!(multiply_finite_field(&multiply_finite_field(&a, &b, 8), &c, 8), multiply_finite_field(&a, &multiply_finite_field(&b, &c, 8), 8));
}

#[test]
fn associativity_11() {
    let a = generate_random_bits(11);
    let b = generate_random_bits(11);
    let c = generate_random_bits(11);
    assert_eq!(add_finite_field(&add_finite_field(&a, &b), &c), add_finite_field(&a, &add_finite_field(&b, &c)));
    assert_eq!(multiply_finite_field(&multiply_finite_field(&a, &b, 11), &c, 11), multiply_finite_field(&a, &multiply_finite_field(&b, &c, 11), 11));
}

#[test]
fn associativity_17() {
    let a = generate_random_bits(17);
    let b = generate_random_bits(17);
    let c = generate_random_bits(17);
    assert_eq!(add_finite_field(&add_finite_field(&a, &b), &c), add_finite_field(&a, &add_finite_field(&b, &c)));
    assert_eq!(multiply_finite_field(&multiply_finite_field(&a, &b, 17), &c, 17), multiply_finite_field(&a, &multiply_finite_field(&b, &c, 17), 17));
}

#[test]
fn associativity_25() {
    let a = generate_random_bits(25);
    let b = generate_random_bits(25);
    let c = generate_random_bits(25);
    assert_eq!(add_finite_field(&add_finite_field(&a, &b), &c), add_finite_field(&a, &add_finite_field(&b, &c)));
    assert_eq!(multiply_finite_field(&multiply_finite_field(&a, &b, 25), &c, 25), multiply_finite_field(&a, &multiply_finite_field(&b, &c, 25), 25));
}

#[test]
fn associativity_31() {
    let a = generate_random_bits(31);
    let b = generate_random_bits(31);
    let c = generate_random_bits(31);
    assert_eq!(add_finite_field(&add_finite_field(&a, &b), &c), add_finite_field(&a, &add_finite_field(&b, &c)));
    assert_eq!(multiply_finite_field(&multiply_finite_field(&a, &b, 31), &c, 31), multiply_finite_field(&a, &multiply_finite_field(&b, &c, 31), 31));
}


#[test]
fn associativity_33() {
    let a = generate_random_bits(33);
    let b = generate_random_bits(33);
    let c = generate_random_bits(33);
    assert_eq!(add_finite_field(&add_finite_field(&a, &b), &c), add_finite_field(&a, &add_finite_field(&b, &c)));
    assert_eq!(multiply_finite_field(&multiply_finite_field(&a, &b, 33), &c, 33), multiply_finite_field(&a, &multiply_finite_field(&b, &c, 33), 33));
}

#[test]
fn associativity_47() {
    let a = generate_random_bits(47);
    let b = generate_random_bits(47);
    let c = generate_random_bits(47);
    assert_eq!(add_finite_field(&add_finite_field(&a, &b), &c), add_finite_field(&a, &add_finite_field(&b, &c)));
    assert_eq!(multiply_finite_field(&multiply_finite_field(&a, &b, 47), &c, 47), multiply_finite_field(&a, &multiply_finite_field(&b, &c, 47), 47));
}

#[test]
fn associativity_61() {
    let a = generate_random_bits(61);
    let b = generate_random_bits(61);
    let c = generate_random_bits(61);
    assert_eq!(add_finite_field(&add_finite_field(&a, &b), &c), add_finite_field(&a, &add_finite_field(&b, &c)));
    assert_eq!(multiply_finite_field(&multiply_finite_field(&a, &b, 61), &c, 61), multiply_finite_field(&a, &multiply_finite_field(&b, &c, 61), 61));
}

#[test]
fn associativity_83() {
    let a = generate_random_bits(83);
    let b = generate_random_bits(83);
    let c = generate_random_bits(83);
    assert_eq!(add_finite_field(&add_finite_field(&a, &b), &c), add_finite_field(&a, &add_finite_field(&b, &c)));
    assert_eq!(multiply_finite_field(&multiply_finite_field(&a, &b, 83), &c, 83), multiply_finite_field(&a, &multiply_finite_field(&b, &c, 83), 83));
}

#[test]
fn associativity_101() {
    let a = generate_random_bits(101);
    let b = generate_random_bits(101);
    let c = generate_random_bits(101);
    assert_eq!(add_finite_field(&add_finite_field(&a, &b), &c), add_finite_field(&a, &add_finite_field(&b, &c)));
    assert_eq!(multiply_finite_field(&multiply_finite_field(&a, &b, 101), &c, 101), multiply_finite_field(&a, &multiply_finite_field(&b, &c, 101), 101));
}

#[test]
fn associativity_125() {
    let a = generate_random_bits(125);
    let b = generate_random_bits(125);
    let c = generate_random_bits(125);
    assert_eq!(add_finite_field(&add_finite_field(&a, &b), &c), add_finite_field(&a, &add_finite_field(&b, &c)));
    assert_eq!(multiply_finite_field(&multiply_finite_field(&a, &b, 125), &c, 125), multiply_finite_field(&a, &multiply_finite_field(&b, &c, 125), 125));
}

#[test]
fn associativity_127() {
    let a = generate_random_bits(127);
    let b = generate_random_bits(127);
    let c = generate_random_bits(127);
    assert_eq!(add_finite_field(&add_finite_field(&a, &b), &c), add_finite_field(&a, &add_finite_field(&b, &c)));
    assert_eq!(multiply_finite_field(&multiply_finite_field(&a, &b, 127), &c, 127), multiply_finite_field(&a, &multiply_finite_field(&b, &c, 127), 127));
}