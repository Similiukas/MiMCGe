# MiMC with general degree *n* polynomial (MiMCGn)

This is a Computer Science Project for MSci. Mathematics and Computer Science degree at University of Birmingham.

The tool built for the project is a CLI tool which implements [MiMC]() cipher but instead of *x^3*, taking a general polynomial *x^n* with an arbitrary exponent *n*.
The tool allows to test **diffusion**, **confusion**, cipher **efficiency** and encryption/decryption of the cipher itself. Additionally, **AES** and original **MiMC** are implemented for comparison.

The full report for the project can be found TODO: add link [here]()

## Running the script

First, you need to have [rust with cargo]() installed TODO: link. Then, install cargo dependencies:

```bash
TODO: kaip
```
Then build the optimized binary:
```bash
cargo build --release
```

This will create *./target* directory with the compiled code.
TODO: rename package to mimcgn
Now you can run the CLI tool to see all available options:
```bash
./target/release/mimc_rust --help
```

## Options

The tool allows to test cipher:
 - **diffusion**
 - **confusion**
 - **enc-time** (encrypting *test-size* number of plaintexts *sample-size* amount of times)
 - **dec-time** (decrypting *test-size* number of ciphertexts *sample-size* amount of times)
 - **cipher-test** (encrypts and decrypts random or given plaintext)

Implemented ciphers for testing are **AES**, **MiMC** and **MiMCGn** which is the second argument to the tool.

**Note:** only a number of block sizes are implemented. This is because cipher operations are in *GF(2^n)* field, thus, for every different block size *n*, irreducible polynomial need to be given.
Moreover, AES implementation is provided by the crate [TODO: duoti crate](), which allows only 128 bit block size. 

## Examples

- To test MiMCGn with polynomial *x^5*, block size of *17* and random plaintext:
```bash
./target/release/mimc_rust cipher_test mimcgn 17 --exponent 5
```

- To get MiMCGn with polynomial *x^5* diffusion statistic of block_size *17* and *1000* plaintexts:
```bash
./target/release/mimc_rust diffusion mimcgn 17 --exponent 5 --test-size 1000
```
