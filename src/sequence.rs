//! This module contains iteration and enumeration implementations.

use core::iter::{Copied, Enumerate};
use core::slice::Iter;

/// Sequences represent contiguous slices of memory.
pub trait Sequence {
    /// The current type that comprises a sequence.
    type Item;

    /// An iterator over the sequence, producing individual items.
    type Iter: Iterator<Item = Self::Item>;

    /// An enumerating iterator, producing pairs of indices and items.
    ///
    /// The position always refers to a byte index.
    type Enum: Iterator<Item = (usize, Self::Item)>;

    /// Returns an iterator. Each iteration contains an item.
    fn iterate(&self) -> Self::Iter;

    /// Returns an enumerating iterator. Each iteration contains a byte index and item.
    fn enumerate(&self) -> Self::Enum;
}

impl<'items> Sequence for &'items [u8] {
    type Item = u8;

    type Iter = Copied<Iter<'items, u8>>;

    type Enum = Enumerate<Self::Iter>;

    fn iterate(&self) -> Self::Iter {
        self.iter().copied()
    }

    fn enumerate(&self) -> Self::Enum {
        self.iterate().enumerate()
    }
}
