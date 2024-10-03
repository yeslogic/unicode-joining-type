unicode-joining-type
====================

<div align="center">
  <a href="https://github.com/yeslogic/unicode-joining-type/actions/workflows/ci.yml">
    <img src="https://github.com/yeslogic/unicode-joining-type/actions/workflows/ci.yml/badge.svg" alt="Build Status"></a>
  <a href="https://docs.rs/unicode-joining-type">
    <img src="https://docs.rs/unicode-joining-type/badge.svg" alt="Documentation">
  </a>
  <a href="https://crates.io/crates/unicode-joining-type">
    <img src="https://img.shields.io/crates/v/unicode-joining-type.svg" alt="Version">
  </a>
  <img src="https://img.shields.io/badge/unicode-16.0-informational" alt="Unicode Version">
  <a href="https://github.com/yeslogic/unicode-joining-type/blob/master/LICENSE">
    <img src="https://img.shields.io/crates/l/unicode-joining-type.svg" alt="License">
  </a>
</div>

<br>

Fast lookup of the Unicode Joining Type and Joining Group properties for `char`
in Rust using Unicode 16.0 data. This crate is no-std compatible.

Usage
-----

```rust
use unicode_joining_type::{get_joining_type, JoiningType};
use unicode_joining_type::{get_joining_group, JoiningGroup};

fn main() {
    assert_eq!(get_joining_type('A'), JoiningType::NonJoining);
    assert_eq!(get_joining_group('ھ'), JoiningGroup::KnottedHeh);
}
```

Performance & Implementation Notes
----------------------------------

[ucd-generate] is used to generate `joining_type_tables.rs` and
`joining_group_tables.rs`. A build script (`build.rs`) compiles each of these
into a two level look up tables. The look up time is constant as it is just
indexing into two arrays.

The two level approach maps a code point to a block, then to a position within
a block. The allows the second level of block to be deduplicated, saving space.
The code is parameterised over the block size, which must be a power of 2. The
value in the build script is optimal for the data set.

This approach trades off some space for faster lookups. The joining type tables
take up about 26KiB, the joining group tables take up about 6.75KiB. Benchmarks
showed this approach to be ~5–10× faster than the typical binary search
approach.

There is still room for further size reduction. For example, by eliminating
repeated block mappings at the end of the first level block array.

[ucd-generate]: https://github.com/yeslogic/ucd-generate
