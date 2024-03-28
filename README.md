# MiMC with general degree *e* polynomial (MiMCGe)

This is a Computer Science Project for MSci. Mathematics and Computer Science degree at University of Birmingham.

The tool built for the project is a CLI which implements [MiMC](https://eprint.iacr.org/2016/492) cipher but instead of *x^3*, taking a general polynomial *x^e* with an arbitrary exponent *e*.
The tool allows to test **diffusion**, **confusion** and cipher **efficiency**. It also allows to run encryption/decryption for the cipher itself. Additionally, **AES** and original **MiMC** are implemented for comparison.

The full report for the project can be found [here](./report/report.pdf).

## Running the script

First, you need to have [rust with cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.

Then build the optimized binary:
```bash
cargo build --release
```

This will create *./target* directory with the compiled code. Now you can run the CLI tool to see all available options:
```bash
./target/release/mimcge --help
```

## Options

The tool allows to test cipher:
 - **diffusion**: additionally mean, sample standard deviation are provided for null hypothesis testing (*chapter 3.2*).
 - **confusion**: additionally mean, sample standard deviation are provided for null hypothesis testing (*chapter 3.2*).
 - **enc-time**: encrypting *test-size* number of plaintexts *sample-size* amount of times.
 - **dec-time**: decrypting *test-size* number of ciphertexts *sample-size* amount of times.
 - **cipher-test**: encrypts and decrypts random or given plaintext.
 - **generate-test-samples**: provides an encrypted sequence from 0 to *test-size* for NIST STS testing.
 - **start-bit-stream**: starts printing an encrypted sequence from 0 of 32-bit numbers for Dieharder testing.

Implemented ciphers for testing are **AES**, **MiMC** and **MiMCGe** which is the second argument for the tool.

**Note:** only a number of block sizes are implemented. This is because cipher operations are in *GF(2^n)* field, thus, for every different block size *n*, irreducible polynomial need to be provided.
Moreover, AES implementation is provided by the [crate](https://docs.rs/aes/latest/aes), which allows only 128 bit block size. 

### Important remark
MiMCGe ciphers with powers of two exponent (x^2, x^4, etc.) are not secure. This is discussed in chapter 3.3 of the report.

## Examples

- To test MiMCGe with polynomial *x^5*, block size of *17* and random plaintext:
```bash
./target/release/mimcge cipher-test mimcge 17 --exponent 5
```

- To provide specific round constants (this will determine number of rounds), key and plaintext. This way the ciphertext will always be the same. In this case, ciphertext will be 580:
```bash
./target/release/mimcge cipher-test mimcge 11 -e 5 -p 201 -k 11 -R 0 3 443 221 50
```

- To get MiMCGe with polynomial *x^5* diffusion statistic of block_size *17* and *1000* plaintexts. This will also provide sample standard deviation together with mean and expected mean. These can be used to test null hypothesis, whether the test sample is significant enough.
```bash
./target/release/mimcge diffusion mimcge 17 --exponent 5 --test-size 1000
```
