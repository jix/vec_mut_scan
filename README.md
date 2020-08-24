# vec_mut_scan

[![github][github-badge]][github] [![crates.io][crate-badge]][crate] [![docs.rs][docs-badge]][docs]

[github]: https://github.com/jix/vec_mut_scan
[crate]: https://crates.io/crates/vec_mut_scan
[docs]: https://docs.rs/vec_mut_scan/*/vec_mut_scan

Forward scan over a vector with mutation and item removal.

Provides an iterator like interface over a vector which allows mutation and
removal of items. Items are kept in order and every item is moved at most once,
even when items are removed. Dropping the `VecMutScan` mid-iteration keeps
remaining items in the vector.

This can be seen as an extension of `Vec`'s `retain` and `drain`. It is also
very similar to the unstable `drain_filter` but slightly more flexible. Unlike
`drain_filter` this specifies the drop behavior (to keep all following
elements). It also doesn't require the filtering to be done within a closure,
which gives additional flexibilty at the cost of not being able to implement
the `Iterator` trait.

## License

The vec_mut_scan source code is licensed under either of

  * Apache License, Version 2.0 (see [LICENSE-APACHE](LICENSE-APACHE))
  * MIT license (see [LICENSE-MIT](LICENSE-MIT))

at your option.

### Contribution

Unless You explicitly state otherwise, any Contribution intentionally submitted
for inclusion in the Work by You, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[github-badge]: https://img.shields.io/badge/github-jix/vec_mut_scan-blueviolet?style=flat-square
[crate-badge]: https://img.shields.io/crates/v/vec_mut_scan?style=flat-square
[docs-badge]: https://img.shields.io/badge/docs.rs-vec_mut_scan-informational?style=flat-square
