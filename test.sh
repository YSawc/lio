#!/bin/bash

mkdir -p workspace
cd workspace

assert() {
  expected="$1"
  input="$2"

  cargo run $2
  cc -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 42 42
