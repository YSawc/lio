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

assert 42 '42;'
assert 3 '1+2;'
assert 15 '1+2+3+4+5;'
assert 2 '3-1;'
assert 4 '6-1-1;'
assert 9 '6-1+1+3;'
assert 10 '5*2;'
assert 9 '5+2*2;'
assert 1 '5-2*2;'
assert 2 '4/2;'
assert 8 '4/2*4;'
assert 8 '1+4/2*4-1;'
assert 14 '2*(3+4);'
assert 14 '2+(3*4);'
assert 0 '2+(2-4);'
assert 1 '7%2;'
assert 2 '2*(3-1)-2*1;' simplified
assert 1 '2==2;'
assert 0 '2!=2;'
assert 0 '2<2;'
assert 0 '1<=0;'
assert 1 '1>0;'
assert 1 '1>=0;'
assert 1 '1*2>=3-(2*1);'
assert 3 '(1*2>=3-(2*1))+2;'
assert 3 '(1*2>=3-(2*1))+2;' simplified
assert 4 '4'
assert 3 '2; 3'
assert 4 'return 4;'
assert 1 'int a = 3; 1;'
assert 4 'int a = 3; int b = 4; b;'
assert 36 'int a = 3; int b = 4; b*a*3;'
assert 60 'int a = 3; int b = 4; b*a*5;' simplified
assert 54 'int a = 3; int b = a*2; b*a*3;'
assert 54 'int a = 3; int b = a*2; b*a*3;' simplified
assert 15 'int r = 1; int l = 5; l + map 2 4 + + r;' simplified
assert 120 'map 1 5 *;'
assert 10 'if (2 == 3) 5; else 10;'
assert 31 'if (2 == 3) 5+3; else 10+21;' simplified
assert 8 'if (2 < 3) 5+3; else 10;'
assert 2 'int a = 3; a = 2; a'

echo "------------------------------"
echo "All test passed!"
