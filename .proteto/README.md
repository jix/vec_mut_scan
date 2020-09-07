%% extends "README_base.md"
%% block body
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
%% endblock
