//! This module contains a cursor type used to maintain state.
use crate::slice::View;
use crate::{sequence::Sequence, slice::Slice};
use core::{
    iter::{Copied, Enumerate},
    ops::{Range, RangeFrom, RangeTo},
    slice::Iter,
};

// NOTE: Add second, separate lifetime to represent `Cursor` and `inner` flows.

/// Cursor used to wrap a slice of bytes and give it additional capabilities.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Cursor<'inner> {
    inner: &'inner [u8],
    position: usize,
}

impl<'inner> Cursor<'inner> {
    pub const fn new(inner: &'inner [u8]) -> Self {
        debug_assert!(inner.len() < isize::MAX as usize);
        Self { inner, position: 0 }
    }

    pub fn inner(&self) -> &'inner [u8] {
        &self.inner
    }

    /// Returns the current position of the cursor.
    pub fn position(&self) -> usize {
        self.position
    }

    /// Consumes the cursor, returning the inner value as a vector of bytes.
    pub fn into_vec(self) -> Vec<u8> {
        self.inner.to_vec()
    }

    /// Advance the internal cursor.
    pub fn jump(&mut self) {
        if self.has_remaining() {
            self.position += 1;
            self.inner = self.view_from(self.position..);
        }
    }

    /// Advance the internal cursor by the given number of bytes.
    pub fn jump_to(&mut self, count: usize) {
        for _ in 0..count {
            self.jump()
        }
    }

    /// Returns the next item from the inner type, advancing the position.
    pub fn next_item(&mut self) -> Option<(usize, &u8)> {
        self.next()
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
        let remaining = self.inner.remaining();
        (remaining, Some(remaining))
    }
}

impl<'view> View for Cursor<'view> {
    type Subslice = &'view [u8];

    fn view(&self, range: Range<usize>) -> Self::Subslice {
        self.inner.slice(range)
    }

    fn view_from(&self, from: RangeFrom<usize>) -> Self::Subslice {
        self.inner.slice(from)
    }

    fn view_to(&self, to: RangeTo<usize>) -> Self::Subslice {
        self.inner.slice(to)
    }
}

impl<'sequence> Sequence for Cursor<'sequence> {
    type Item = u8;

    type Iter = Copied<Iter<'sequence, u8>>;

    type Enum = Enumerate<Self::Iter>;

    fn iter_copied(&self) -> Self::Iter {
        self.inner.iter().copied()
    }

    fn enumerate(&self) -> Self::Enum {
        self.iter_copied().enumerate()
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
}
