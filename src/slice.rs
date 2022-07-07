//! This module contains trait extensions for `slice` types.

use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

/// Trait for performing slicing operations via [`Range`] types.
///
/// [`Range`]: https://doc.rust-lang.org/core/ops/struct.Range.html
pub trait Slice<R> {
    /// Slices self according to the range argument
    fn slice(&self, range: R) -> Self;
}

// This base implementation for `Slice<R>` was borrowed from `nom`.
macro_rules! slice_range_impl {
    ( [ $for_type:ident ], $ty:ty ) => {
        impl<'a, $for_type> Slice<$ty> for &'a [$for_type] {
            fn slice(&self, range: $ty) -> Self {
                &self[range]
            }
        }
    };
    ( $for_type:ty, $ty:ty ) => {
        impl<'a> Slice<$ty> for &'a $for_type {
            fn slice(&self, range: $ty) -> Self {
                &self[range]
            }
        }
    };
}

macro_rules! slice_ranges_impl {
    ( [ $for_type:ident ] ) => {
        slice_range_impl! {[$for_type], Range<usize>}
        slice_range_impl! {[$for_type], RangeInclusive<usize>}
        slice_range_impl! {[$for_type], RangeTo<usize>}
        slice_range_impl! {[$for_type], RangeToInclusive<usize>}
        slice_range_impl! {[$for_type], RangeFrom<usize>}
        slice_range_impl! {[$for_type], RangeFull}
    };
    ( $for_type:ty ) => {
        slice_range_impl! {$for_type, Range<usize>}
        slice_range_impl! {$for_type, RangeInclusive<usize>}
        slice_range_impl! {$for_type, RangeTo<usize>}
        slice_range_impl! {$for_type, RangeToInclusive<usize>}
        slice_range_impl! {$for_type, RangeFrom<usize>}
        slice_range_impl! {$for_type, RangeFull}
    };
}

slice_ranges_impl! {str}
slice_ranges_impl! {[T]}
