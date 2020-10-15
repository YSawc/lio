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

assert 42 'fn int { 42; }'
assert 3 'fn int { 1+2; }'
assert 15 'fn int { 1+2+3+4+5; }'
assert 2 'fn int { 3-1; }'
assert 4 'fn int { 6-1-1; }'
assert 9 'fn int { 6-1+1+3;}'
assert 10 'fn int { 5*2; }'
assert 9 'fn int { 5+2*2; }'
assert 1 'fn int { 5-2*2; }'
assert 2 'fn int { 4/2; }'
assert 8 'fn int { 4/2*4; }'
assert 8 'fn int { 1+4/2*4-1; }'
assert 14 'fn int { 2*(3+4); }'
assert 14 'fn int { 2+(3*4); }'
assert 0 'fn int { 2+(2-4); }'
assert 1 'fn int { 7%2; }'
assert 2 'fn int { 2*(3-1)-2*1; }' simplified
assert 1 'fn int { 2==2; }'
assert 0 'fn int { 2!=2; }'
assert 0 'fn int { 2<2; }'
assert 0 'fn int { 1<=0; }'
assert 1 'fn int { 1>0; }'
assert 1 'fn int { 1>=0; }'
assert 1 'fn int { 1*2>=3-(2*1); } '
assert 3 'fn int { (1*2>=3-(2*1))+2; }'
assert 3 'fn int { (1*2>=3-(2*1))+2; }' simplified
assert 4 'fn int { 4; }'
assert 3 'fn int { 2; 3; }'
assert 4 'fn int { return 4; }'
assert 1 'fn int { int a = 3; a; 1; }'
assert 12 'fn int { int a = 3; int b = 4; a*b; }'
assert 36 'fn int { int a = 3; int b = 4; b*a*3; }'
assert 60 'fn int { int a = 3; int b = 4; b*a*5; }' simplified
assert 54 'fn int { int a = 3; int b = a*2; b*a*3; }'
assert 54 'fn int { int a = 3; int b = a*2; b*a*3; }' simplified
assert 15 'fn int { int r = 1; int l = 5; l + map 2 4 + + r; }' simplified
assert 120 'fn int { map 1 5 *; }'
assert 10 'fn int { if (2 == 3) 5; else 10; }'
assert 31 'fn int { if (2 == 3) 5+3; else 10+21; }' simplified
assert 8 'fn int { if (2 < 3) 5+3; else 10; }'
assert 2 'fn int { int a = 3; a; a = 2; a; }'
assert 5 'fn int { int a = 5; int b = a; a = 2; b }'
assert 1 'fn int { 1; }'
assert 2 'fn int { 2 }'
assert 3 'fn int { return 3; }'
assert 0 'fn { _ }'
assert 0 'fn { int vvv = 55; 0*vvv*0; _ }'
assert 0 'fn { int vvv = 4; 1+2*vvv/4-1; _; }'
assert 0 'fn { int vvv = 4; 1+2*vvv/4-1; _ }' simplified
assert 27 'int g = 9; fn int { g*3 }'
assert 8 'int g = 8; fn int { int v = 4; 1+v*g/4-1 }'
assert 8 'int g = 8; fn int { int v = 4; 1+v*g/4-1; }'
# assert 8 'int g = 8; fn int { int v = 4; return 1+v*g/4-1; }'
assert 0 'int g = 2; fn { int v = 4; 1+v*g/4-1; _ }'
assert 0 'int g = 2; fn { int v = 4; 1+v*g/4-1; _ }' simplified
assert 10 'int g = 2; int l = 3; int o = 4; fn int { int v = 4; l+v*o/g-1; }' simplified
assert 0 'int g = 2; int l = 3; int o = 4; fn { int v = 4; l+v*o/g-1; _ }'

echo "------------------------------"
echo "All test passed!"
