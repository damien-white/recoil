//! This module contains a cursor type used to maintain state.

use crate::{sequence::Sequence, view::View};
use core::{
    iter::{Copied, Enumerate},
    ops::Range,
    slice::Iter,
};

/// Cursor used to wrap a slice of bytes and give it additional capabilities.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Cursor<'inner> {
    inner: &'inner [u8],
    position: usize,
}

impl<'inner> Cursor<'inner> {
    pub fn new(inner: &'inner [u8]) -> Self {
        debug_assert!(inner.len() < isize::MAX as usize);
        Self { inner, position: 0 }
    }

    /// Returns a reference to the inner byte slice.
    pub fn get_ref(&self) -> &'inner [u8] {
        self.inner
    }

    /// Returns the current position of the cursor.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Consumes the cursor, returning the inner value as a vector of bytes.
    pub fn into_vec(self) -> Vec<(usize, u8)> {
        self.iter_indices().collect()
    }

    /// Advance the internal cursor by the specified number of bytes.
    pub fn advance(&mut self, count: usize) {
        if self.remaining() > count {
            self.position += count;
            self.inner = self.view_from(self.position);
        }
    }

    /// Returns a slice of `count` bytes, panicking if count > len.
    pub fn peek_to(&mut self, count: usize) -> Option<&'inner [u8]> {
        (self.remaining() >= count)
            .then_some(self.view_to(count))
            .filter(|v| v.len() == count)
    }

    /// Divides the inner slice into two at the given `index` value.
    pub fn split_at(&mut self, mid: usize) -> (&'inner [u8], &'inner [u8]) {
        if self.remaining() >= mid {
            let (before, after) = self.inner.split_at(mid);

            assert_eq!(before, &self.inner[self.position..mid]);
            assert_eq!(after, &self.inner[mid..]);

            self.position += mid;

            assert_eq!(after, &self.inner[self.position..]);
            (before, after)
        } else {
            (&[], self.inner)
        }
    }

    /// Returns the next item from the cursor's bytes, advancing the position.
    pub fn next_item(&mut self) -> Option<(usize, &u8)> {
        self.next()
    }

    /// Returns the item at `offset` bytes, advancing the cursor by the same amount.
    ///
    /// This is similar to calling `advance(offset)`, then `next_item()`.
    pub fn offset(&mut self, offset: usize) -> Option<(usize, &u8)> {
        if self.remaining() >= offset {
            self.advance(offset);
            self.next_item()
        } else {
            None
        }
    }

    pub fn item_at(&mut self, count: usize) -> Option<(usize, &u8)> {
        if self.remaining() >= count {
            self.position += count;
            self.next()
        } else {
            None
        }
    }

    /// Returns the number of remaining bytes inside the cursor.
    pub fn remaining(&self) -> usize {
        if self.position() >= self.len() {
            return 0;
        }

        self.len() - self.position()
    }

    /// Returns true if and only if the cursor contains one or more bytes.
    pub fn has_remaining(&self) -> bool {
        self.remaining() > 0
    }

    /// Returns the number of bytes contained in the cursor.
    fn len(&self) -> usize {
        self.inner.len()
    }
}

impl<'inner> Iterator for Cursor<'inner> {
    type Item = (usize, &'inner u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_remaining() {
            // SAFETY: enough bytes remain to advance the cursor at least once.
            let item = unsafe { self.inner.get_unchecked(self.position) };
            self.position += 1;
            Some((self.position, item))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining();
        (remaining, Some(remaining))
    }
}

impl<'view> View for Cursor<'view> {
    type Slice = &'view [u8];

    fn view(&self, range: Range<usize>) -> Self::Slice {
        &self.inner[range]
    }

    fn view_from(&self, from: usize) -> Self::Slice {
        &self.inner[from..]
    }

    fn view_to(&self, to: usize) -> Self::Slice {
        &self.inner[..to]
    }
}

impl<'sequence> Sequence for Cursor<'sequence> {
    type Item = u8;

    type Iter = Copied<Iter<'sequence, u8>>;

    type Enum = Enumerate<Self::Iter>;

    fn iter_copied(&self) -> Self::Iter {
        self.inner.iter().copied()
    }

    fn iter_indices(&self) -> Self::Enum {
        self.iter_copied().enumerate()
    }
}
