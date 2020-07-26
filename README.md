# envtestkit

[![ci](https://github.com/codinaut/envtestkit/workflows/ci/badge.svg)](https://github.com/codinaut/envtestkit/actions?query=workflow%3Aci)
[![crates.io](https://img.shields.io/crates/v/envtestkit.svg)](https://crates.io/crates/envtestkit)
[![docs.rs](https://docs.rs/envtestkit/badge.svg)](https://docs.rs/envtestkit)
[![semantic-release](https://img.shields.io/badge/%20%20%F0%9F%93%A6%F0%9F%9A%80-semantic--release-e10079.svg)](https://github.com/semantic-release/semantic-release)

Test kit for logics with environment variable involvements.

## Usage

See [the documentation](https://docs.rs/envtestkit) and [examples](examples/).

## Hacking

This repository uses [semantic-release](https://github.com/semantic-release/semantic-release) for automated release and crate publishing. Please look at [Angular commit convention](https://github.com/conventional-changelog/conventional-changelog/tree/master/packages/conventional-changelog-angular).

To test this crate:
```sh
cargo test
```

To see what's happening if environment tests aren't locked:
```sh
cargo test --examples -- --ignored
```

## License

[MIT](LICENSE).
