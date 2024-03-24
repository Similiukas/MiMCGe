#!/bin/bash

max_reduction=20
block_size=31
exponent=3

for (( i=0; i<=max_reduction; i++ )); do
    # Call the executable
    ./target/release/mimcge confusion mimcge "$block_size" --exponent "$exponent" --test-size 10000 --round-reduction "$i"
    # Wait for the executable to finish before proceeding
    wait
done

