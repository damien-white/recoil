//! This module contains trait extensions for `slice` types.

use core::ops::Range;

/// Views use byte-indexing operations to view into "subslices" of memory.
///
/// Please note that a `View` uses use non-inclusive ranges.
pub trait View {
    type Slice: ?Sized;

    /// Returns a view into the subslice, given a lower and upper bound.
    fn view(&self, range: Range<usize>) -> Self::Slice;

    /// Returns a view into the subslice, given a lower bound.
    fn view_from(&self, from: usize) -> Self::Slice;

    /// Returns a view into the subslice, given an upper bound.
    fn view_to(&self, to: usize) -> Self::Slice;
}

impl<'items> View for &'items [u8] {
    type Slice = &'items [u8];

    fn view(&self, range: Range<usize>) -> Self::Slice {
        &self[range]
    }

    fn view_from(&self, from: usize) -> Self::Slice {
        &self[from..]
    }

    fn view_to(&self, to: usize) -> Self::Slice {
        &self[..to]
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct BytesView {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl BytesView {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.start
    }
}
