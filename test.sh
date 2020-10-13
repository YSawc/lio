#!/bin/bash

mkdir -p ./workspace

assert() {
  expected="$1"
  input="$2"
  simplified="$3"

 echo "------------------------------"
 echo "[[rust output]]"
  cargo run "$2" "$3" && cc -o ./workspace/tmp ./workspace/tmp.s && ./workspace/tmp
  actual="$?"

 echo "[[ shell output ]]"
  if [ "$actual" = "$expected" ]; then
    echo "TEST[[$input => $actual]]"
  else
    echo "TEST[[$input => $expected expected, but got $actual]]"
    exit 1
  fi
}

assert 42 'fn { 42; }'
assert 3 'fn { 1+2; }'
assert 15 'fn { 1+2+3+4+5; }'
assert 2 'fn { 3-1; }'
assert 4 'fn { 6-1-1; }'
assert 9 'fn { 6-1+1+3;}'
assert 10 'fn { 5*2; }'
assert 9 'fn { 5+2*2; }'
assert 1 'fn { 5-2*2; }'
assert 2 'fn { 4/2; }'
assert 8 'fn { 4/2*4; }'
assert 8 'fn { 1+4/2*4-1; }'
assert 14 'fn { 2*(3+4); }'
assert 14 'fn { 2+(3*4); }'
assert 0 'fn { 2+(2-4); }'
assert 1 'fn { 7%2; }'
assert 2 'fn { 2*(3-1)-2*1; }' simplified
assert 1 'fn { 2==2; }'
assert 0 'fn { 2!=2; }'
assert 0 'fn { 2<2; }'
assert 0 'fn { 1<=0; }'
assert 1 'fn { 1>0; }'
assert 1 'fn { 1>=0; }'
assert 1 'fn { 1*2>=3-(2*1); } '
assert 3 'fn { (1*2>=3-(2*1))+2; }'
assert 3 'fn { (1*2>=3-(2*1))+2; }J' simplified
assert 4 'fn { 4; }'
assert 3 'fn { 2; 3; }'
assert 4 'fn { return 4; }'
assert 1 'fn { int a = 3; 1; }'
assert 12 'fn { int a = 3; int b = 4; a*b; }'
assert 36 'fn { int a = 3; int b = 4; b*a*3; }'
assert 60 'fn { int a = 3; int b = 4; b*a*5; }' simplified
assert 54 'fn { int a = 3; int b = a*2; b*a*3; }'
assert 54 'fn { int a = 3; int b = a*2; b*a*3; }' simplified
assert 15 'fn { int r = 1; int l = 5; l + map 2 4 + + r; }' simplified
assert 120 'fn { map 1 5 *; }'
assert 10 'fn { if (2 == 3) 5; else 10; }'
assert 31 'fn { if (2 == 3) 5+3; else 10+21; }' simplified
assert 8 'fn { if (2 < 3) 5+3; else 10; }'
assert 2 'fn { int a = 3; a; a = 2; a; }'
assert 5 'fn { int a = 5; int b = a; a = 2; b }'
assert 1 'fn { 1; }'
assert 2 'fn { 2 }'
assert 3 'fn { return 3; }'

echo "------------------------------"
echo "All test passed!"
