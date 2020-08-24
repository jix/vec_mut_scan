//! Forward scan over a vector with mutation and item removal.
use std::ops::{Deref, DerefMut};
use std::{mem, ptr};

/// Forward scan over a vector with mutation and item removal.
///
/// Provides an iterator like interface over a vector which allows mutation and removal of items.
/// Items are kept in order and every item is moved at most once, even when items are removed.
/// Dropping the `VecMutScan` mid-iteration keeps remaining items in the vector.
///
/// This does not implement the iterator trait, as the returned items borrow from this (i.e. this is
/// a streaming iterator).
///
/// The [`next`](VecMutScan::next) method returns [`VecMutScanItem`] values, which auto dereference
/// to the vector's item type but also provide a [`remove`](VecMutScanItem::remove) and
/// [`replace`](VecMutScanItem::replace) method.
pub struct VecMutScan<'a, T: 'a> {
    vec: &'a mut Vec<T>,
    base: *mut T,
    write: usize,
    read: usize,
    end: usize,
}

// Here is a small overview of how this is implemented, which should aid in auditing this library's
// use of unsafe:
//
// The initial state after taking ownership of the data from `vec` looks like this:
//
//   |0 = write = read          |end
//   [ ][ ][ ][ ][ ][ ][ ][ ][ ]
//
// Calling next without deleting items progresses like this:
//
//   |0 |write = read           |end
//   [ ][ ][ ][ ][ ][ ][ ][ ][ ]
//
//   |0    |write = read        |end
//   [ ][ ][ ][ ][ ][ ][ ][ ][ ]
//                .
//                :
//                           |write = read
//   |0                      |  |end
//   [ ][ ][ ][ ][ ][ ][ ][ ][ ]
//
//   |0                         |end = write = read
//   [ ][ ][ ][ ][ ][ ][ ][ ][ ]
//
// If we are in a state like this and delete an item, we introduce a gap of uninitialized data (as
// we moved it elsewere or dropped it) between write and read:
//
//   |0    |write = read        |end
//   [ ][A][B][C][D][E][ ][ ][ ]
//
//         |write
//   |0    |  |read             |end
//   [ ][A] u [C][D][E][ ][ ][ ]
//
// Calling next in that situation moves items over the gap
//
//            |write
//   |0       |  |read          |end
//   [ ][A][C] u [D][E][ ][ ][ ]
//
// Removing more items widens the gap
//
//            |write
//   |0       |     |read       |end
//   [ ][A][C] u  u [E][ ][ ][ ]
//
// Dropping the `VecMutScan` at that point must move the items in the suffix to close the gap before
// passing ownership back to `vec`.

// TODO replace indices with pointers when pointer offset computation is stabilized should
// benchmarks show an improvement.

impl<'a, T: 'a> VecMutScan<'a, T> {
    /// Begin a scan over a vector with mutation and item removal.
    pub fn new(vec: &mut Vec<T>) -> VecMutScan<T> {
        let base = vec.as_mut_ptr();
        let write = 0;
        let read = 0;
        let end = vec.len();

        // Make sure `vec` is in a consistent state should this `VecMutScan` be leaked. In that case
        // all items within `vec` are also leaked, which is safe. This strategy is also called leak
        // amplification. This can be seen as the `VecMustScan` taking ownership over `vec`'s items,
        // while still keeping them in `vec`'s buffer. As we keep a mutable reference to the `vec`
        // we stop others from messing with its items.
        unsafe {
            vec.set_len(0);
        }

        VecMutScan {
            vec,
            base,
            write,
            read,
            end,
        }
    }

    /// Advance to the next item of the vector.
    ///
    /// This returns a reference wrapper that enables item removal (see [`VecMutScanItem`]).
    #[allow(clippy::should_implement_trait)] // can't be an iteratore due to lifetimes
    pub fn next<'s>(&'s mut self) -> Option<VecMutScanItem<'s, 'a, T>> {
        // This just constructs a VecMutScanItem without updating any state. The read and write
        // offsets are adjusted by `VecMutScanItem` whenever it is dropped or one of its
        // self-consuming methods are called.
        if self.read != self.end {
            Some(VecMutScanItem { scan: self })
        } else {
            None
        }
    }
}

impl<'a, T: 'a> Drop for VecMutScan<'a, T> {
    fn drop(&mut self) {
        // When we are dropped, there might be a gap of uninitialized (after dropping) memory
        // between a prefix of non-removed items we iterated over and a suffix of items we did not
        // iterate over. We need to move the suffix to close the gap, so we have a consecutive
        // buffer of items. Then we can safely set `vec`'s length to the total number of remaining
        // items.

        unsafe {
            // The read performed by copy is safe as `self.read..self.end` contains valid data and
            // is within `vec`'s buffer.

            // The write performed by copy is safe as `self.write <= self.read` so
            // `self.write..self.write + suffix_len` also stays within `vec`'s buffer.
            let suffix_len = self.end - self.read;
            // This is required to handle overlapping copies.
            ptr::copy(
                self.base.add(self.read),
                self.base.add(self.write),
                suffix_len,
            );
            // `0..self.write` contained valid data before the copy and the copy also moved valid
            // data to `self.write..self.write + suffix_len`. We took ownership of that data and can
            // safely pass that ownership to `vec` here.
            self.vec.set_len(self.write + suffix_len);
        }
    }
}

/// Reference wrapper that enables item removal for [`VecMutScan`].
pub struct VecMutScanItem<'s, 'a, T: 'a> {
    scan: &'s mut VecMutScan<'a, T>,
}

// When a `VecMutScanItem` is created, there must be valid data at `scan.read` i.e. `scan.read` must
// not have reached `scan.end` yet.

impl<'s, 'a, T: 'a> VecMutScanItem<'s, 'a, T> {
    /// Removes and returns this item from the vector.
    pub fn remove(self) -> T {
        unsafe {
            // Read the next item, taking local ownership of the data to return it.
            let result = ptr::read(self.scan.base.add(self.scan.read));
            // Adjust the read pointer but keep the write pointer to create or widen the gap (see
            // diagrams above).
            self.scan.read += 1;
            // Do not run the `VecMutScanItem`'s drop, as it handles the case for a non-removed item
            // and would perform a now invalid update of the `VecMutScan`.
            mem::forget(self);
            result
        }
    }

    /// Replaces this item with a new value, returns the old value.
    ///
    /// This is equivalent to assigning a new value or calling [`std::mem::replace`] on the mutable
    /// reference obtained by using [`DerefMut`], but can avoid an intermediate move within the
    /// vector's buffer.
    pub fn replace(self, value: T) -> T {
        unsafe {
            // Read the next item, taking local ownership of the data to return it.
            let result = ptr::read(self.scan.base.add(self.scan.read));

            // Write the replacement in place of the removed item, adjusted for the gap between
            // write and read (see diagrams above).
            ptr::write(self.scan.base.add(self.scan.write), value);
            // Advance the position without changing the width of the gap.
            self.scan.read += 1;
            self.scan.write += 1;
            // Do not run the `VecMutScanItem`'s drop, as it handles the case for a non-replaced
            // item and would perform a now invalid update of the `VecMutScan`.
            mem::forget(self);
            result
        }
    }
}

impl<'s, 'a, T: 'a> Deref for VecMutScanItem<'s, 'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Within a `VecMutScanItem` the offset `scan.read` contains valid data owned by the
        // `VecMutScan` on which we have a mutable borrow, thus we are allowed to reference it.
        unsafe { &*self.scan.base.add(self.scan.read) }
    }
}

impl<'s, 'a, T: 'a> DerefMut for VecMutScanItem<'s, 'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Within a `VecMutScanItem` the offset `scan.read` contains valid data owned by the
        // `VecMutScan` on which we have a mutable borrow, thus we are allowed to mutably reference
        // it.
        unsafe { &mut *self.scan.base.add(self.scan.read) }
    }
}

impl<'s, 'a, T: 'a> Drop for VecMutScanItem<'s, 'a, T> {
    fn drop(&mut self) {
        unsafe {
            // Move the item at `scan.read` to `scan.write` i.e. move it over the gap (see diagrams
            // above).
            ptr::copy(
                self.scan.base.add(self.scan.read),
                self.scan.base.add(self.scan.write),
                1,
            );
            // Advance the position without changing the width of the gap.
            self.scan.read += 1;
            self.scan.write += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::rc::Rc;

    #[test]
    fn check_item_drops() {
        let mut input: Vec<_> = vec![0, 1, 2, 3, 4, 5, 6, 7].into_iter().map(Rc::new).collect();
        let input_copy = input.clone();

        let mut scan = VecMutScan::new(&mut input);

        let mut keep = None;
        let mut also_keep = None;

        while let Some(item) = scan.next() {
            if **item == 2 {
                item.replace(Rc::new(10));
            } else if **item == 3 {
                keep = Some(item.remove());
            } else if **item == 4 {
                item.remove();
            } else if **item == 5 {
                also_keep = Some(item.replace(Rc::new(20)));
            } else if **item == 6 {
                break;
            }
        }

        let _keep_copy = keep.clone();
        let _also_keep_copy_1 = also_keep.clone();
        let _also_keep_copy_2 = also_keep.clone();

        let ref_counts: Vec<_> = input_copy.iter().map(|rc| Rc::strong_count(rc)).collect();

        assert_eq!(ref_counts, vec![2, 2, 1, 3, 1, 4, 2, 2]);
        assert_eq!(keep.map(|rc| Rc::strong_count(&rc)), Some(3));
        assert_eq!(also_keep.map(|rc| Rc::strong_count(&rc)), Some(4));
    }
}
