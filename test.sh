#!/bin/bash

cargo build --release

test() {
    cat tests/$1 | target/release/abridge -c > tests/$1.zip
    cat tests/$1.zip | target/release/abridge -d > tests/$1.cmp
    if diff tests/$1 tests/$1.cmp; then
        echo Pass
    else
        echo Fail
    fi
}

cleanup() {
    rm tests/*.cmp
    rm tests/*.zip
}

test "small.txt"
test "english.txt"
test "french.txt"
test "spanish.txt"
cleanup
