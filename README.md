# vec_mut_scan

[![ci][ci-badge]](https://github.com/jix/vec_mut_scan/actions?query=workflow%3A%22Check+Last+Release%22)
[![github][github-badge]](https://github.com/jix/vec_mut_scan)
[![crates.io][crate-badge]](https://crates.io/crates/vec_mut_scan)
[![docs.rs][docs-badge]](https://docs.rs/vec_mut_scan/*/vec_mut_scan)

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

This software is available under the Zero-Clause BSD license, see
[COPYRIGHT](COPYRIGHT) for full licensing information and exceptions to this.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this software by you shall be licensed as defined in
[COPYRIGHT](COPYRIGHT).


[ci-badge]: https://img.shields.io/github/workflow/status/jix/vec_mut_scan/Check%20Last%20Release?style=flat-square
[github-badge]: https://img.shields.io/badge/github-jix/vec_mut_scan-blueviolet?style=flat-square
[crate-badge]: https://img.shields.io/crates/v/vec_mut_scan?style=flat-square
[docs-badge]: https://img.shields.io/badge/docs.rs-vec_mut_scan-informational?style=flat-square
