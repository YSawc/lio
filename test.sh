#!/bin/bash

mkdir -p workspace
cd workspace

assert() {
  expected="$1"
  input="$2"

  cargo run > main.s $input
  as main.s -o main.o
  ld -s -o a.out main.o
  ./a.out
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 42 42
