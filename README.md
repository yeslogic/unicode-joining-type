unicode-joining-type
====================

<div align="center">
  <a href="https://travis-ci.com/yeslogic/unicode-joining-type">
    <img src="https://travis-ci.com/yeslogic/unicode-joining-type.svg?branch=master" alt="Build Status"></a>
  <a href="https://docs.rs/unicode-joining-type">
    <img src="https://docs.rs/unicode-joining-type/badge.svg" alt="Documentation">
  </a>
  <a href="https://crates.io/crates/unicode-joining-type">
    <img src="https://img.shields.io/crates/v/unicode-joining-type.svg" alt="Version">
  </a>
  <a href="https://github.com/yeslogic/unicode-joining-type/blob/master/LICENSE">
    <img src="https://img.shields.io/crates/l/unicode-joining-type.svg" alt="License">
  </a>
</div>

<br>

Fast lookup of the Unicode Joining Type property for `char` in Rust using
Unicode 12.1 data.

Usage
-----

`Cargo.toml`:

```toml
[dependencies]
unicode-joining-type = "0.1.0"
```

`main.rs`:

```rust
use unicode_joining_type::{get_joining_type, JoiningType};

fn main() {
    assert_eq!(get_joining_type('A'), JoiningType::NonJoining);
}
```

Performance & Implementation Notes
----------------------------------

[ucd-generate] is used to generate `tables.rs`. A build script (`build.rs`)
compiles this into a two level look up table. The look up time is constant as
it is just indexing into two arrays.

The two level approach maps a code point to a block, then to a position within
a block. The allows the second level of block to be duplicated, saving space.
The code is parameterised over the block size, which must be a power of 1. The
value in the build script is optimal for the data set.

This approach trades off some space for faster lookups. The tables take up
about 26KiB. Benchmarks showed this approach to be ~5–10× faster than the
typical binary search approach.

There is still room for further size reduction. For example, by eliminating
repeated block mappings at the end of the first level block array.

[ucd-generate]: https://github.com/BurntSushi/ucd-generate
