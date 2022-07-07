use crate::slice::Slice;
use core::ops::{Range, RangeFrom, RangeTo};

/// Views use byte-indexing operations to view into "subslices" of memory.
///
/// Please note that a `View` uses use non-inclusive ranges.
pub trait View {
    type Subslice: ?Sized;

    /// Returns a view into the subslice, given a lower and upper bound.
    fn view(&self, range: Range<usize>) -> Self::Subslice;

    /// Returns a view into the subslice, given a lower bound.
    fn view_from(&self, from: RangeFrom<usize>) -> Self::Subslice;

    /// Returns a view into the subslice, given an upper bound.
    fn view_to(&self, to: RangeTo<usize>) -> Self::Subslice;

    /// Returns the number of bytes remaining in the view.
    fn remaining(&self) -> usize;

    /// Returns true if and only if the view contains more bytes.
    fn has_remaining(&self) -> bool;
}

impl<'items> View for &'items [u8] {
    type Subslice = &'items [u8];

    fn view(&self, range: Range<usize>) -> Self::Subslice {
        self.slice(range)
    }

    fn view_from(&self, from: RangeFrom<usize>) -> Self::Subslice {
        self.slice(from)
    }

    fn view_to(&self, to: RangeTo<usize>) -> Self::Subslice {
        self.slice(to)
    }

    fn remaining(&self) -> usize {
        let pos = self.as_ptr() as usize;
        let len = self.len();
        pos - len
    }

    fn has_remaining(&self) -> bool {
        self.remaining() > 0
    }
}
