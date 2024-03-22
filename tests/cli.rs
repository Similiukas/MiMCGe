use std::process::Command;
use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use predicates::prelude::predicate;

#[test]
fn cipher_test_happy_path_11() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mimcge")?;

    cmd.arg("cipher-test").arg("mimcge").arg("11")
        .args(&["-p", "1868"])
        .args(&["-k", "1362"])
        .args(&["-e", "7"])
        .args(&["-R", "0", "773", "996", "1417"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Plaintext:  1868 [1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0]\nCiphertext: 1962 [1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 0]\nDecrypted:  1868 [1, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0]"));

    Ok(())
}

#[test]
fn cipher_test_happy_path_33() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mimcge")?;

    cmd.arg("cipher-test").arg("mimcge").arg("33")
        .args(&["-p", "2121644265"])
        .args(&["-k", "2704582896"])
        .args(&["-e", "11"])
        .args(&["-R", "0", "4167564917", "2955227280", "6014621339", "7070376341", "3894014214", "5647237025", "681616375", "6711229718", "2588429073"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Plaintext:  2121644265 [0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1]\nCiphertext: 6523462206 [1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 1, 0]"));

    Ok(())
}

#[test]
fn error_wrong_cipher() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mimcge")?;

    cmd.arg("enc-time").arg("mimce");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));

    Ok(())
}

#[test]
fn error_wrong_block_size() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("mimcge")?;

    cmd.arg("enc-time").arg("mimcge").arg("64");

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));

    Ok(())
}
