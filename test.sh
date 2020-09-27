#!/bin/bash

assert() {
  expected="$1"
  input="$2"

 echo "------------------------------"
 echo "[[rust output]]"
  cargo run $2
  cc -o ./workspace/tmp ./workspace/tmp.s
  ./workspace/tmp
  actual="$?"

 echo "[[ shell output ]]"
  if [ "$actual" = "$expected" ]; then
    echo "TEST[[$input => $actual]]"
  else
    echo "TEST[[$input => $expected expected, but got $actual]]"
    exit 1
  fi
}

assert 42 42
assert 3 1+2
assert 15 1+2+3+4+5

echo "------------------------------"
echo "All test passed!"
