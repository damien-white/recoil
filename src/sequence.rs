//! This module contains iteration and enumeration implementations.

use core::{
    iter::{Copied, Enumerate},
    slice::Iter,
};

/// Sequences represent contiguous slices of memory.
pub trait Sequence {
    /// The type being iterated over.
    ///
    /// The sequence is comprised of items, such as `u8`.
    type Item;

    /// An iterator that  iterator over the sequence, producing individual items.
    type Iter: Iterator<Item = Self::Item>;

    /// An enumerating iterator, producing pairs of indices and items.
    ///
    /// The position always refers to a byte index.
    type Enum: Iterator<Item = (usize, Self::Item)>;

    /// Returns an iterator over the items of the sequence.
    ///
    /// Note: This iterator uses `.copied()` to return an owned iterator.
    fn iter_copied(&self) -> Self::Iter;

    /// Returns an enumerating iterator. Each iteration contains a byte index and item.
    fn enumerate(&self) -> Self::Enum;

    /// Returns the number of items remaining in the sequence.
    fn remaining(&self) -> usize;

    /// Returns true if and only if one or more items remain in the sequence.
    fn has_remaining(&self) -> bool;
}

impl<'items> Sequence for &'items [u8] {
    type Item = u8;

    type Iter = Copied<Iter<'items, Self::Item>>;

    type Enum = Enumerate<Self::Iter>;

    fn iter_copied(&self) -> Self::Iter {
        self.iter().copied()
    }

    fn enumerate(&self) -> Self::Enum {
        self.iter_copied().enumerate()
    }

    // TODO: Test this and consider adding more pointer-based methods.
    fn remaining(&self) -> usize {
        let start = self.as_ptr() as usize;
        let size = core::mem::size_of::<&[u8]>();
        let end = size * self.len();

        let remaining = end - start;
        println!("\n[start]: {start}");
        println!("\n[end]: {end}");
        println!("\n[remaining]: {remaining}");
        remaining
    }

    fn has_remaining(&self) -> bool {
        self.remaining() > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_remaining_length() {
        let input = &br#"In the previous section we tried running our unsafe
singly-linked queue under miri, and it said we had broken the rules of
stacked borrows, and linked us some documentation.
"#[..];
        // let offset = 0;
        let len = input.len();

        let expected = len;
        let actual = input.remaining();
        assert_eq!(
            actual, expected,
            "Expected `{}` bytes, but got: `{}`",
            expected, actual
        );
    }
}
