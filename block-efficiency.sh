#!/bin/bash

# Define the number of times to call the executable
block_sizes=(5 8 11 17 25 31 47 61 83 101 125 127)

for block in "${block_sizes[@]}"; do
    # Call the executable
    ./target/release/mimcge enc-time mimcge "$block" --exponent 24 --test-size 1000 --sample-size 1000
    # Wait for the executable to finish before proceeding
    wait
done

