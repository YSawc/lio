#!/bin/bash

assert() {
  expected="$1"
  input="$2"

  cargo run $2
  cc -o ./workspace/tmp ./workspace/tmp.s
  ./workspace/tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 42 42
assert 3 1+2
