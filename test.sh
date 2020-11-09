#!/bin/bash

mkdir -p ./workspace

assert_llvm() {
  expected="$1"
  input="$2"
  simplified="$3"

echo "Starts llvm tests!"
 echo "------------------------------"
 echo "[[rust output]]"
  cargo run "$2" ll "$3" && lli-11 ./workspace/tmp.ll
  actual="$?"

 echo "[[ shell output ]]"
  if [ "$actual" = "$expected" ]; then
    echo "TEST[[$input => $actual]]\n"
  else
    echo "TEST[[$input => $expected expected, but got $actual]]\n"
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
assert_llvm 14 'fn int { int a = 3*4; 2+a }'
assert_llvm 9 'fn int { int a = 3*4; 2+a-5 }'
assert_llvm 4 'fn int { int a = 3*4; 2+a-5*2 }' simplified
assert_llvm 2 'fn int { int a = 3*4; a/2-2*2 }' simplified
assert_llvm 6 'fn int { int a = 3*4; a/2 }'
assert_llvm 3 'fn int { 3*(6==(3*2)) } '
assert_llvm 3 'fn int { 3*(6==(3*2)) } ' simplified
assert_llvm 3 'fn int { (1*2>=3-(2*1))+2 }'
assert_llvm 3 'fn int { (1*2>=3-(2*1))+2 }' simplified
assert_llvm 5 'fn int { int xx = 4; (1*2>=xx-(2*1))+4 }'
assert_llvm 7 'fn int { int xx = 4; (1*2>=xx-(2*1))+6 }' simplified
assert_llvm 12 'fn int { int a = 3; int b = 4; a*b }'
assert_llvm 36 'fn int { int a = 3; int b = 4; b*a*3 }'
assert_llvm 54 'fn int { int a = 3; int b = a*2; b*a*3 }'
assert_llvm 15 'fn int { int r = 1; int l = 5; l + map 2 4 + + r }' simplified
assert_llvm 120 'fn int { map 1 5 * }'
# assert_llvm 10 'fn int { if (2 == 3) { 5; } else { 10; } }'
# assert_llvm 31 'fn int { if (2 == 3) { 5+3; } else { 10+21; } }' simplified
# assert_llvm 8 'fn int { if (2 < 3) { 5+3; } else { 10; } }'
assert_llvm 2 'fn int { int a = 3; a; a = 2; a }'
assert_llvm 5 'fn int { int a = 5; int b = a; a = 2; b }'
assert_llvm 1 'fn int { 1 }'
assert_llvm 2 'fn int { 2 }'
assert_llvm 3 'fn int { return 3; }'
assert_llvm 0 'fn { _ }'
assert_llvm 0 'fn { int vvv = 55; 0*vvv*0; _ }'
assert_llvm 0 'fn { int vvv = 4; 1+2*vvv/4-1; _ }'
assert_llvm 0 'fn { int vvv = 4; 1+2*vvv/4-1; _ }' simplified
assert_llvm 27 'int g = 9; fn int { g*3 }'
assert_llvm 8 'int g = 8; fn int { int v = 4; 1+v*g/4-1 }'
assert_llvm 8 'int g = 8; fn int { int v = 4; 1+v*g/4-1 }'
assert_llvm 8 'int g = 8; fn int { int v = 4; return 1+v*g/4-1; }'
assert_llvm 0 'int g = 2; fn { int v = 4; 1+v*g/4-1; _ }'
assert_llvm 0 'int g = 2; fn { int v = 4; 1+v*g/4-1; _ }' simplified
assert_llvm 10 'int g = 2; int l = 3; int o = 4; fn int { int v = 4; l+v*o/g-1 }' simplified
assert_llvm 0 'int g = 2; int l = 3; int o = 4; fn { int v = 4; l+v*o/g-1; _ }'
assert_llvm 3 'int g = 10; fn int { g; int g = 3; g }'
assert_llvm 0 'int _g = 10; fn { _ }'
assert_llvm 0 'fn { int _u = 8; _ }'
assert_llvm 16 'fn int { int _u = 8; int a = 2; a*_u }'
assert_llvm 34 'fn int { int _u = 8; int a = 2; 4+2*a*_u-2 }'
assert_llvm 1 'fn int { int i = 1; if (i) { 0 } else { 0 } i }'
assert_llvm 0 'fn { if (2 == 3) { 1; 2; } else { 3; 4; } _ }'
assert_llvm 0 'fn { int i = 9; if (i) { 1; 2; } else { 3*4; 5; } _ }'
assert_llvm 0 'fn { int i = 9; if (i) { i; 2 } else { 3*4; 5 } _ }'
assert_llvm 0 'fn { int i = 9; if (i) { i; _ } else { 3*4; _ } _ }'
assert_llvm 0 'fn { int i = 9; if (i) { i; 2; _ } else { 3*4; _ } _ }'

echo "------------------------------"
echo "All llvm test passed!\n"
