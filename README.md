# lio

[![CircleCI](https://circleci.com/gh/YSawc/lio.svg?style=shield)](https://circleci.com/gh/YSawc/lio)
[![](http://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

A toy compiler implemented by rust.

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
  - [ ] alpha
  - [x] [beta](https://github.com/YSawc/lio/commit/dd6dd3de39019f4c7bec2677140fb22e9f06fcc9)
  - [x] [simplified](https://github.com/YSawc/lio/commit/e2199f937ca5e13c19579430e677ea922cd4cbf5)
- [ ] unused variable checker
  - [ ] global
  - [x] [local](https://github.com/YSawc/lio/commit/da07a3dc4c1985c2116da6e4e94554c51d51e30c)
  - [x] [not checkes for under score variable](https://github.com/YSawc/lio/commit/0c95ef3d9c57e8578d584aaef5dc42fca986a3c9)
- [x] [checker for return type](https://github.com/YSawc/lio/commit/cb7864e64982aeb98adda36f606e96cb451b0784)
