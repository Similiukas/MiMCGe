use crate::utils::helpers::{add_finite_field, multiply_finite_field, power_finite_field, square_multiply, to_decimal};

#[test]
fn convert_to_decimal() {
    assert_eq!(to_decimal(&vec![1,1,0,1,0,1]), 53);
}

#[test]
fn add_over_finite_field() {
    let a = vec![1,0,1,1,0]; // 22
    let b = vec![0,0,0,0,0]; // 0
    assert_eq!(add_finite_field(&a, &b), vec![1,0,1,1,0]); // 22
}

#[test]
fn add_over_finite_field_17() {
    let a = vec![1,0,1,1,0,1,0,0,0,1,1,1,0,1,1,0,1]; // 92397
    let b = vec![1,0,0,0,1,0,0,0,0,1,0,1,1,1,0,0,0]; // 69816
    assert_eq!(add_finite_field(&a, &b), vec![0,0,1,1,1,1,0,0,0,0,1,0,1,0,1,0,1]); // 30805
}

#[test]
fn add_over_finite_field_25() {
    let a = vec![0,0,0,0,0,1,0,0,0,1,1,1,1,1,0,0,1,1,1,1,1,1,1,1,1]; // 588287
    let b = vec![1,0,1,0,1,1,1,0,0,0,0,1,1,1,1,0,0,0,1,1,1,0,1,1,0]; // 22822006
    assert_eq!(add_finite_field(&a, &b), vec![1,0,1,0,1,0,1,0,0,1,1,0,0,0,1,0,1,1,0,0,0,1,0,0,1]); // 22332809
}

#[test]
fn multiply_over_finite_field_zero() {
    let a = vec![1,1,0,1,0]; // 26
    let b = vec![0,0,0,0,0]; // 0
    assert_eq!(multiply_finite_field(&a, &b, 5), vec![0,0,0,0,0]); // 0
}

#[test]
fn multiply_over_finite_field_one() {
    let a = vec![1,1,0,1,0]; // 26
    let b = vec![0,0,0,0,1]; // 1
    assert_eq!(multiply_finite_field(&a, &b, 5), vec![1,1,0,1,0]); // 26
}

#[test]
fn multiply_over_finite_field_5() {
    let a = vec![1,1,0,1,0]; // 26
    let b = vec![1,0,0,0,0]; // 16
    assert_eq!(multiply_finite_field(&a, &b, 5), vec![1,1,1,0,0]); // 28
}

#[test]
fn multiply_over_finite_field_17() {
    let a = vec![1,0,1,1,0,1,0,0,0,1,1,1,0,1,1,0,1]; // 92397
    let b = vec![1,0,0,0,1,0,0,0,0,1,0,1,1,1,0,0,0]; // 69816
    assert_eq!(multiply_finite_field(&a, &b, 17), vec![0,0,0,1,1,0,1,1,0,0,1,1,0,0,0,0,1]); // 13921
}

#[test]
fn multiply_over_finite_field_25() {
    let a = vec![0,0,0,0,0,1,0,0,0,1,1,1,1,1,0,0,1,1,1,1,1,1,1,1,1]; // 588287
    let b = vec![1,0,1,0,1,1,1,0,0,0,0,1,1,1,1,0,0,0,1,1,1,0,1,1,0]; // 22822006
    assert_eq!(multiply_finite_field(&a, &b, 25), vec![0,0,1,1,0,0,0,1,0,1,1,0,0,1,0,0,0,0,0,1,1,1,0,0,0]); // 6473784
}

#[test]
fn multiply_over_finite_field_31() {
    let a = vec![0,0,0,0,1,0,0,1,0,1,1,0,0,1,1,1,0,0,1,0,0,1,1,0,1,0,1,1,0,0,1]; // 78877529
    let b = vec![1,0,1,0,0,0,0,1,0,1,1,0,1,0,0,0,0,1,1,0,0,1,1,1,0,1,1,0,1,1,1]; // 1353986999
    assert_eq!(multiply_finite_field(&a, &b, 31), vec![1,1,1,1,1,1,1,1,0,1,1,1,0,0,1,0,1,1,1,0,0,0,0,1,0,0,1,0,1,0,1]); // 2142859413
}

#[test]
fn power_over_finite_field_5() {
    let a = vec![0,0,1,0,0]; // 4
    assert_eq!(power_finite_field(&a, 5, 5), vec![1,0,0,0,1]); // 17
}

#[test]
fn power_over_finite_field_17() {
    let a = vec![0,1,0,1,1,1,0,0,0,0,0,1,0,0,1,1,0]; // 47142
    assert_eq!(power_finite_field(&a, 5, 17), vec![0,0,1,1,1,1,0,1,1,1,1,1,1,1,0,0,1]); // 31737
}

#[test]
fn square_over_finite_field_17() {
    let a = vec![0,1,0,1,1,1,0,0,0,0,0,1,0,0,1,1,0]; // 47142
    assert_eq!(square_multiply(&a, 5, 17), vec![0,0,1,1,1,1,0,1,1,1,1,1,1,1,0,0,1]); // 31737
}

#[test]
fn square_over_finite_field_25() {
    let a = vec![0,1,0,1,1,1,0,0,0,0,0,1,0,0,1,1,0,1,0,1,1,1,0,1,0]; // 12068538
    assert_eq!(square_multiply(&a, 5135, 25), vec![1,0,1,1,1,1,1,0,1,0,1,1,1,1,1,0,0,0,0,0,1,1,1,1,1]); // 25000991
}
