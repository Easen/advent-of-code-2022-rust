#!/bin/bash

if [[ -z "${SESSION_COOKIE-""}" ]]; then
    echo "No session token set in \$SESSION_COOKIE."
    exit 1
fi

for i in {1..25}; do
    # your-unix-command-here
    URL=https://adventofcode.com/2022/day/$i/input
    echo $URL
    curl "$URL" --cookie "session=$SESSION_COOKIE" -s | tee "inputs/$i.txt"
done
