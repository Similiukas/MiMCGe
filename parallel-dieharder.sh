#!/bin/bash

number_of_cores=5

for (( i=15; i<=32; i++ )); do
    ( .run-bit-stream.sh | dieharder -d 203 -n "$i" -g 200 ) &
    if (( i % number_of_cores == 0 )); then wait; fi
done
