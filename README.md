# arXiv

[![License](https://img.shields.io/badge/License-MIT%20%26%20Apache%202.0-blue)](#license)
[![docs.rs](https://img.shields.io/docsrs/arxiv/latest)](https://docs.rs/arxiv/)
[![CI](https://github.com/acmuta-research/arxiv-rs/actions/workflows/main.yml/badge.svg)](https://github.com/acmuta-research/arxiv-rs/actions/workflows/main.yml)
[![Security audit](https://github.com/acmuta-research/arxiv-rs/actions/workflows/security-audit.yml/badge.svg)](https://github.com/acmuta-research/arxiv-rs/actions/workflows/security-audit.yml)
[![codecov](https://codecov.io/gh/acmuta-research/arxiv-rs/branch/main/graph/badge.svg?token=6ZSIWAQTHU)](https://codecov.io/gh/acmuta-research/arxiv-rs)

A Rust library for parsing `arXiv` identifiers and references.

## Install

Run the following command in the terminal:

```shell
cargo add arxiv
```

Or, add this to `Cargo.toml`:

```shell
[dependencies]
arxiv = "0.1"
```

## Usage

```rust
use std::str::FromStr;
use arxiv::{ArxivId, ArxivStamp};

// Parse an arXiv identifier
let id = ArxivId::from_str("arXiv:9912.12345v2").unwrap();
assert_eq!(id.month, 12);
assert_eq!(id.year, 2099);
assert_eq!(id.number, "12345");
assert_eq!(id.version, Some(2));

// Parse an arXiv stamp
let stamp = ArxivStamp::from_str("arXiv:0706.0001v1 [q-bio.CB] 1 Jun 2007").unwrap();
assert_eq!(stamp.category, "q-bio.CB");
assert_eq!(stamp.submitted.year(), 2007);
```

## License

Licensed under either of

* Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([`LICENSE-MIT`](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
