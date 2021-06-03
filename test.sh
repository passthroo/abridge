#!/bin/bash

cargo build --release

test() {
    cat tests/$1 | target/release/abridge -c > tests/$1.tzip
    cat tests/$1.tzip | target/release/abridge -d > tests/$1.cmp
    if diff tests/$1 tests/$1.cmp; then
        echo +Pass
    else
        echo !Fail
    fi
    rm tests/$1.cmp tests/$1.tzip
}

test "small.txt"
test "english.txt"
test "french.txt"
test "spanish.txt"
