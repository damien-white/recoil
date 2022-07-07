//! This module contains a cursor type used to maintain state.
use crate::{sequence::Sequence, slice::Slice, view::View};
use core::{
    iter::{Copied, Enumerate},
    ops::{Range, RangeFrom, RangeTo},
    slice::Iter,
};

/// Cursor used to wrap byte slices and track the current position in the input.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Cursor<'inner> {
    inner: &'inner [u8],
    position: usize,
}

impl<'inner> Cursor<'inner> {
    pub const fn new(inner: &'inner [u8]) -> Self {
        Self { inner, position: 0 }
    }

    /// Consumes the cursor, returning the inner value as a vector of bytes.
    pub fn into_vec(self) -> Vec<u8> {
        self.inner.to_vec()
    }

    pub fn inner(&self) -> &'inner [u8] {
        self.inner
    }

    /// Returns the current position of the cursor.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Advance the internal cursor.
    pub fn bump(&mut self) {
        if self.has_remaining() {
            self.position += 1;
            self.inner = self.view_from(self.position..);
        }
    }

    /// Advance the internal cursor by the given number of bytes.
    pub fn bump_to(&mut self, count: usize) {
        for _ in 0..count {
            self.bump()
        }
    }

    /// Returns the next item from the inner input type, advancing the position.
    pub fn next_item(&mut self) -> Option<(usize, &u8)> {
        self.next()
    }

    /// Returns the number of bytes contained in the cursor.
    fn len(&self) -> usize {
        self.inner().len()
    }
}

impl<'inner> Iterator for Cursor<'inner> {
    type Item = (usize, &'inner u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.has_remaining() {
            // SAFETY: we have enough remaining to advance at least once.
            let item = unsafe { self.inner.get_unchecked(self.position) };
            self.position += 1;
            Some((self.position, item))
        } else {
            None
        }
    }
}

impl<'view> View for Cursor<'view> {
    type Subslice = &'view [u8];

    fn view(&self, range: Range<usize>) -> Self::Subslice {
        self.inner().slice(range)
    }

    fn remaining(&self) -> usize {
        if self.position() >= self.len() {
            return 0;
        }

        self.len() - self.position()
    }

    fn has_remaining(&self) -> bool {
        self.remaining() > 0
    }

    fn view_from(&self, from: RangeFrom<usize>) -> Self::Subslice {
        self.inner().slice(from)
    }

    fn view_to(&self, to: RangeTo<usize>) -> Self::Subslice {
        self.inner().slice(to)
    }
}

impl<'sequence> Sequence for Cursor<'sequence> {
    type Item = u8;

    type Iter = Copied<Iter<'sequence, u8>>;

    type Enum = Enumerate<Self::Iter>;

    fn iterate(&self) -> Self::Iter {
        self.inner().iter().copied()
    }

    fn enumerate(&self) -> Self::Enum {
        self.iterate().enumerate()
    }
}
