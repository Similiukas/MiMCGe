#!/bin/bash

# Define the number of times to call the executable
num_calls=40

# Loop for num_calls times
for (( i=2; i<=num_calls; i++ )); do
     # Call the executable
    ./target/release/mimcge enc-time mimcge 11 --exponent "$i" --test-size 1000 --sample-size 1000

     # Wait for the executable to finish before proceeding
    wait
done
