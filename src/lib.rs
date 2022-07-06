use core::iter::{Copied, Enumerate};
use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};
use core::slice::Iter;
use std::path::{Path, PathBuf};

pub mod error;

/// Abstracts away slicing and indexing operations using ranges.
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

/// Spans use indexing and pointers to view into subslices of memory.
pub trait Span {
    type Slice: ?Sized;

    /// Provides the ability to take a view into the slice by range.
    fn view(&self, range: Range<usize>) -> Self::Slice;

    /// Returns the remaining byte slice the span represents.
    fn remaining(&self) -> Self::Slice;

    /// Returns true if and only if the span contains more bytes.
    fn has_remaining(&self) -> bool;
}

impl<'items> Span for &'items [u8] {
    type Slice = &'items [u8];

    fn view(&self, range: Range<usize>) -> Self::Slice {
        self.slice(..range.end)
    }

    fn remaining(&self) -> Self::Slice {
        self.slice(..self.len())
    }

    fn has_remaining(&self) -> bool {
        !self.remaining().is_empty()
    }
}

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

/// A type that allows a handle to the input type.
///
/// The `Cursor` mostly contains some pointers used to maintain "state" during
/// the execution of your program. It can be seen as the main type of data that
/// you're working with while interating with [`recoil`].
///
/// It is created to fulfill the role of recoil's main type. It contains various
/// metadata about the input and methods to manipulate it efficiently.
///
/// Cursor must wrap the `I` type and satisfies its constraints. (Byte offsets)
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd)]
pub struct Cursor<'input> {
    pub input: &'input [u8],
    pub position: usize,
}

impl<'input> Cursor<'input> {
    pub fn new(input: &'input [u8]) -> Self {
        Self { input, position: 0 }
    }

    pub fn input(&self) -> &'input [u8] {
        self.input
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn bump(&mut self) -> Option<&u8> {
        self.next()
    }
}

impl<'input> Iterator for Cursor<'input> {
    type Item = &'input u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.input.len() {
            None
        } else {
            // We have enough room to advance at least once.
            self.position += 1;
            self.input.iter().next()
        }
    }
}

/// Helper function for resolving relative file paths as absolute paths.
pub(crate) fn resolved_path<P: AsRef<Path>>(path: P) -> Result<PathBuf, std::io::Error> {
    let resolved = std::env::current_dir()?.join(path.as_ref());
    if !resolved.exists() {
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "failed to find file or directory",
        ))
    } else {
        Ok(resolved)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;

    use super::*;

    #[test]
    fn it_works() {
        let resolved = match resolved_path("mock/resume.pdf") {
            Ok(p) => p,
            Err(err) => panic!("error: {err:?}"),
        };

        let mut file = File::open(&resolved).expect("failed to open file");

        let mut buf = vec![];
        let nread = match file.read_to_end(&mut buf) {
            Ok(n) => n,
            Err(err) => panic!("failed to read file into buffer: {err}"),
        };

        let input = &buf[..nread];

        let mut iter = input.iterate();

        let input = input.view(0..1024);

        if input.has_remaining() {
            let span = iter.next();
            let view1 = input.slice(..256);
            let view2 = input.slice(256..512);
            println!("view 1 is {} bytes long", view1.len());
            assert!(!view1.is_empty());
            assert_eq!(
                view1.len(),
                256,
                "view 1 should be equal to range length (..256)"
            );
            assert_eq!(
                view2.len(),
                256,
                "view 1 should be equal to range length (256..512)"
            );
            assert!(!view2.is_empty());
            println!("span: {:?}", span);
            println!("view1: {:?}", view1);
            println!("view2: {:?}", view2);
        }

        assert!(!input.is_empty());
        println!("[input]:\n{:?}\n\n[input.len()]:\n{:?}", input, input.len());
        assert_eq!(input.len(), 1024);
    }
}
