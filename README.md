# vec_mut_scan

Forward scan over a vector with mutation and item removal.

Provides an iterator like interface over a vector which allows mutation and
removal of items. Items are kept in order and every item is moved at most once,
even when items are removed. Dropping the `VecMutScan` mid-iteration keeps
remaining items in the vector.

## License

The vec_mut_scan source code is licensed under either of

  * Apache License, Version 2.0 (see [LICENSE-APACHE](LICENSE-APACHE))
  * MIT license (see [LICENSE-MIT](LICENSE-MIT))

at your option.

### Contribution

Unless You explicitly state otherwise, any Contribution intentionally submitted
for inclusion in the Work by You, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
