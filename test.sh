#!/bin/bash

mkdir -p ./workspace

assert_x86_64() {
  expected="$1"
  input="$2"
  simplified="$3"

echo "Starts x86_64 tests!"
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

assert_x86_64 42 'fn int { 42; }'
assert_x86_64 3 'fn int { 1+2; }'
assert_x86_64 15 'fn int { 1+2+3+4+5; }'
assert_x86_64 2 'fn int { 3-1; }'
assert_x86_64 4 'fn int { 6-1-1; }'
assert_x86_64 9 'fn int { 6-1+1+3;}'
assert_x86_64 10 'fn int { 5*2; }'
assert_x86_64 9 'fn int { 5+2*2; }'
assert_x86_64 1 'fn int { 5-2*2; }'
assert_x86_64 2 'fn int { 4/2; }'
assert_x86_64 8 'fn int { 4/2*4; }'
assert_x86_64 8 'fn int { 1+4/2*4-1; }'
assert_x86_64 14 'fn int { 2*(3+4); }'
assert_x86_64 14 'fn int { 2+(3*4); }'
assert_x86_64 0 'fn int { 2+(2-4); }'
assert_x86_64 1 'fn int { 7%2; }'
assert_x86_64 2 'fn int { 2*(3-1)-2*1; }' simplified
assert_x86_64 1 'fn int { 2==2; }'
assert_x86_64 0 'fn int { 2!=2; }'
assert_x86_64 0 'fn int { 2<2; }'
assert_x86_64 0 'fn int { 1<=0; }'
assert_x86_64 1 'fn int { 1>0; }'
assert_x86_64 1 'fn int { 1>=0; }'
assert_x86_64 1 'fn int { 1*2>=3-(2*1); } '
assert_x86_64 3 'fn int { (1*2>=3-(2*1))+2; }'
assert_x86_64 3 'fn int { (1*2>=3-(2*1))+2; }' simplified
assert_x86_64 4 'fn int { 4; }'
assert_x86_64 3 'fn int { 2; 3; }'
assert_x86_64 4 'fn int { return 4; }'
# assert_x86_64 1 'fn int { int a = 3; a; 1; }'
# assert_x86_64 12 'fn int { int a = 3; int b = 4; a*b; }'
# assert_x86_64 36 'fn int { int a = 3; int b = 4; b*a*3; }'
# assert_x86_64 60 'fn int { int a = 3; int b = 4; b*a*5; }' simplified
# assert_x86_64 54 'fn int { int a = 3; int b = a*2; b*a*3; }'
# assert_x86_64 54 'fn int { int a = 3; int b = a*2; b*a*3; }' simplified
# assert_x86_64 15 'fn int { int r = 1; int l = 5; l + map 2 4 + + r; }' simplified
# assert_x86_64 120 'fn int { map 1 5 *; }'
# assert_x86_64 10 'fn int { if (2 == 3) { 5; } else { 10; } }'
# assert_x86_64 31 'fn int { if (2 == 3) { 5+3; } else { 10+21; } }' simplified
# assert_x86_64 8 'fn int { if (2 < 3) { 5+3; } else { 10; } }'
# assert_x86_64 2 'fn int { int a = 3; a; a = 2; a; }'
# assert_x86_64 5 'fn int { int a = 5; int b = a; a = 2; b }'
# assert_x86_64 1 'fn int { 1; }'
# assert_x86_64 2 'fn int { 2 }'
# assert_x86_64 3 'fn int { return 3; }'
# assert_x86_64 0 'fn { _ }'
# assert_x86_64 0 'fn { int vvv = 55; 0*vvv*0; _ }'
# assert_x86_64 0 'fn { int vvv = 4; 1+2*vvv/4-1; _; }'
# assert_x86_64 0 'fn { int vvv = 4; 1+2*vvv/4-1; _ }' simplified
# assert_x86_64 27 'int g = 9; fn int { g*3 }'
# assert_x86_64 8 'int g = 8; fn int { int v = 4; 1+v*g/4-1 }'
# assert_x86_64 8 'int g = 8; fn int { int v = 4; 1+v*g/4-1; }'
# assert_x86_64 8 'int g = 8; fn int { int v = 4; return 1+v*g/4-1; }'
# assert_x86_64 0 'int g = 2; fn { int v = 4; 1+v*g/4-1; _ }'
# assert_x86_64 0 'int g = 2; fn { int v = 4; 1+v*g/4-1; _ }' simplified
# assert_x86_64 10 'int g = 2; int l = 3; int o = 4; fn int { int v = 4; l+v*o/g-1; }' simplified
# assert_x86_64 0 'int g = 2; int l = 3; int o = 4; fn { int v = 4; l+v*o/g-1; _ }'
# assert_x86_64 3 'int g = 10; fn int { g; int g = 3; g }'
# assert_x86_64 0 'int _g = 10; fn { _ }'
# assert_x86_64 0 'fn { int _u = 8; _ }'
# assert_x86_64 16 'fn int { int _u = 8; int a = 2; a*_u }'

echo "------------------------------"
echo "All x86_64 test passed!\n"

assert_llvm() {
  expected="$1"
  input="$2"
  simplified="$3"

echo "Starts llvm tests!"
 echo "------------------------------"
 echo "[[rust output]]"
  cargo run "$2" ll "$3" && lli ./workspace/tmp.ll
  actual="$?"

 echo "[[ shell output ]]"
  if [ "$actual" = "$expected" ]; then
    echo "TEST[[$input => $actual]]"
  else
    echo "TEST[[$input => $expected expected, but got $actual]]"
    exit 1
  fi
}

assert_llvm 42 'fn int { return 42; }'
assert_llvm 0 'fn { _ }'
assert_llvm 0 'fn { int _a = 3*4; _ }'
assert_llvm 7 'fn int { int a = 3+4; return a; }'
assert_llvm 12 'fn int { int a = 3*4; a }'
assert_llvm 8 'fn int { int _a = 3*4; 2+3*2 }' simplified
assert_llvm 8 'fn int { int _a = 3*4; int _b = 6/2; 2+3*2 }' simplified

echo "------------------------------"
echo "All llvm test passed!\n"
