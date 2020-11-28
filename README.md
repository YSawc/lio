# lio

[![CircleCI](https://circleci.com/gh/YSawc/lio.svg?style=shield)](https://circleci.com/gh/YSawc/lio)
[![](http://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A toy compiler emits llvm-ir implemented by rust.

## Dependencies

- llvm11:
ex)
```
sudo bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"`
```

## Tests

[Tests of Rust](https://github.com/YSawc/lio/tree/master/src/tests)

[Tests with shell](https://github.com/YSawc/lio/blob/master/test.sh)

## Mainly roadmap
- [x] arithmetic operations
- [x] return type
  - [x] [void](https://github.com/YSawc/lio/commit/72baca71be1b0dad59960c3c76b9c6e53bf00f6d)
  - [x] int
- [x] variable
  - [x] [global](https://github.com/YSawc/lio/commit/4df4ee844b75715870242e2cbe8bfa00ae52ca02)
  - [x] [local](https://github.com/YSawc/lio/commit/3df3c89b00e1ceab2925bc02fead9df5c5fc7c78)
  - [x] [simplified initializing](https://github.com/YSawc/lio/commit/3c4b0c609fa4e742342443ebc280ca9ea8e016a5)
- [ ] simplified
  - [x] [beta](https://github.com/YSawc/lio/commit/dd6dd3de39019f4c7bec2677140fb22e9f06fcc9)
  - [x] [simplified](https://github.com/YSawc/lio/commit/e2199f937ca5e13c19579430e677ea922cd4cbf5)
  - [x] [simplified mixed variable](https://github.com/YSawc/lio/commit/f843da7c562e6ea6aa8a2a486b31d2e7d991ad46)
- [ ] unused variable checker
  - [x] [global](https://github.com/YSawc/lio/commit/a8c70e5e3824b5b61afaf07ff636bdeca46b41b6)
  - [x] [local](https://github.com/YSawc/lio/commit/da07a3dc4c1985c2116da6e4e94554c51d51e30c)
  - [x] [not checkes for under score variable](https://github.com/YSawc/lio/commit/0c95ef3d9c57e8578d584aaef5dc42fca986a3c9)
- [x] [checker for return type](https://github.com/YSawc/lio/commit/cb7864e64982aeb98adda36f606e96cb451b0784)
- [x] [checker return same types for statements](https://github.com/YSawc/lio/commit/5a23a108d809bf110e7ad18df9791b4fb606fc82)
- [x] multiple if label [1](https://github.com/YSawc/lio/commit/3b0f2ec7e102c8cea78baa0c24f715f59339632c), [2](https://github.com/YSawc/lio/commit/52bd10a754e341e00d6080c917420807db17ddd8), [3](https://github.com/YSawc/lio/commit/3b0f2ec7e102c8cea78baa0c24f715f59339632c)
- [ ] pipe
  - [x] [express opened immmediate used as return value](https://github.com/YSawc/lio/commit/2ecefecf6447c71f5dd78fcb442df74989033d66)
- [ ] touple
- [ ] multiple error notification
  - [x] [tokenize](https://github.com/YSawc/lio/commit/d09ae5afe26fd3daf1dcddd3dd333224cffe247c)
  - [ ] parser
  - [ ] whole of program
- [ ] assign with block statements
  - [x] [single block statement](https://github.com/YSawc/lio/commit/d0b4bd2475de2e61b3b4ae9e656829b27d1d030f)
  - [x] [while statement](https://github.com/YSawc/lio/commit/09ed8f255ffb97dfedfe21efb973f0af92db1a71)
  - [x] [if-else statement](https://github.com/YSawc/lio/commit/d242bcebcd65fcbcbb4bbf1104d4021fd5c43326)
- [ ] inheritance value
  - [x] [single block statement](https://github.com/YSawc/lio/commit/1948e97d17145848501717822e74a97813a7f8cb)
  - [x] [while statement](https://github.com/YSawc/lio/commit/09ed8f255ffb97dfedfe21efb973f0af92db1a71)
  - [x] [if-else statement](https://github.com/YSawc/lio/commit/7782886caa70e076a3d48a1a71f772846a83628e)
- [ ] lifetime
