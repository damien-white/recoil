//! This module contains type and trait extensions for slices, or spans.

use core::fmt::Debug;
use core::mem;
use core::ops::{Deref, Range};

use crate::prelude::Collection;

/// Wrapper type for working directly with `&[u8]` slices.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ByteSpan<'a> {
    slice: &'a [u8],
    start: usize,
    end: usize,
}

impl<'a> ByteSpan<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self {
            start: 0,
            end: slice.len(),
            slice,
        }
    }

    /// Returns the inner slice of the Span as a `&[u8]`, or byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        self.slice
    }

    /// Returns the inner slice of the Span as a `&str`.
    pub fn as_str(&self) -> &str {
        match core::str::from_utf8(self.as_bytes()) {
            Ok(str_slice) => str_slice,
            Err(err) => panic!("failed to convert inner slice to `&str` type: {err:?}"),
        }
    }

    /// Returns the start offset of the inner slice.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the end offset value of the inner slice.
    pub fn end(&self) -> usize {
        self.end
    }
}

impl<'a> Deref for ByteSpan<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.slice
    }
}

impl<'a> From<&'a [u8]> for ByteSpan<'a> {
    fn from(slice: &'a [u8]) -> Self {
        Self {
            slice,
            start: 0,
            end: slice.len(),
        }
    }
}

impl<'a> Iterator for ByteSpan<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((first, rest)) = self.slice.split_first() {
            self.slice = rest;
            Some(first)
        } else {
            None
        }
    }
}

/// Wrapper type for working directly with `&str` slices.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct StrSpan<'a> {
    slice: &'a str,
    start: usize,
    end: usize,
}

impl<'a> Deref for StrSpan<'a> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.slice
    }
}

impl<'a> From<&'a str> for StrSpan<'a> {
    fn from(slice: &'a str) -> Self {
        Self {
            slice,
            start: 0,
            end: slice.len(),
        }
    }
}

impl<'a> StrSpan<'a> {
    pub fn new(slice: &'a str) -> Self {
        Self {
            start: 0,
            end: slice.len(),
            slice,
        }
    }

    /// Returns the inner slice of the Span as a `&[u8]`, or byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        self.slice.as_bytes()
    }

    /// Returns the inner slice of the Span as a `&str`.
    pub fn as_str(&self) -> &str {
        self.slice
    }

    /// Returns the start offset of the inner slice.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the end offset value of the inner slice.
    pub fn end(&self) -> usize {
        self.end
    }
}

impl<'a> Iterator for StrSpan<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(item) = self.slice.chars().next() {
            self.slice = mem::take(&mut self.slice);
            Some(item)
        } else {
            None
        }
    }
}

/// Fundamental trait for interacting with slices of borrowed memory.
///
/// Spans are slices of contiguous memory with well-defined start and end
/// offsets, or pointers. Spans automatically inherit much of the behavior of
/// Rust's slice type while combining and extending the built-in [`Index`] trait
/// and [`Range`] type.
///
/// The explict lower and upper bounds, coupled with the fact they represent
/// readonly, immutable data means they are **always** safe to construct, read
/// and copy from. These properties also make it easy to create fast, efficient
/// containers for inspecting memory that has already been allocated.
///
/// For convenience, the [`Span`] trait uses Rust's [`Range`] syntax and
/// provides additional capabilities on top of this.
///
/// [`Range`]: https://doc.rust-lang.org/core/ops/struct.Range.html
/// [`Index`]: https://doc.rust-lang.org/core/ops/trait.Index.html
/// [`Span`]: https://docs.rs/recoil/latest/src/recoil/trait.View.html
/// [`Collection`]: crate::collection::Collection;
pub trait Span: Collection {
    /// Individual token type that comprises a `Slice`.
    ///
    /// For a `&str` slice, this is `char`. For `&[u8]`, this is `u8`.
    type Member: ?Sized + Clone + Copy;

    /// The `Slice` represents the type of the internal memory slice.
    ///
    /// This type is typically `&str` for strings, or `&[u8]` for bytes.
    type RefSlice: ?Sized + Clone + Copy + Collection;

    /// Constructs and returns a view into memory over a given `range`.
    fn over(&self, range: Range<usize>) -> Self::RefSlice;

    /// Returns a view into a slice of memory up to `index`.
    fn to(&self, index: usize) -> Self::RefSlice;

    /// Searches for an element in the iterable, returning the two halves split
    /// at the found element.
    ///
    /// `until()` takes a closure that returns `true` or `false`. The closure
    /// is applied to each element of the span until the closure resolves to
    /// `true`. Upon success, `until()` returns `Some(head, tail)`. If no element
    /// is found that matches the predicate, returns `None`.
    fn split_when<W>(&self, when: W) -> Option<(Self::RefSlice, Self::RefSlice)>
    where
        W: Fn(Self::Member) -> bool;
}

impl<'a> Span for &'a str {
    type RefSlice = &'a str;
    type Member = char;

    fn over(&self, range: Range<usize>) -> Self::RefSlice {
        &self[range.start..range.end]
    }

    fn to(&self, index: usize) -> Self::RefSlice {
        let end = index.min(self.len());
        let start = 0.max(self.as_ptr() as usize);
        debug_assert!(start <= end, "Out of bounds! `start` must be <= `end`");
        debug_assert!(index == end, "Out of bounds! `to` must be == `end`");
        &self[0..end]
    }

    fn split_when<W>(&self, when: W) -> Option<(Self::RefSlice, Self::RefSlice)>
    where
        W: Fn(char) -> bool,
    {
        self.as_iter()
            .position(when)
            .map(|index| self.split_at(index))
        // self.as_iter().position(f).map(|index| self.over(0..index))
    }
}

impl<'a> Span for &'a [u8] {
    type RefSlice = &'a [u8];

    type Member = u8;

    fn over(&self, range: Range<usize>) -> Self::RefSlice {
        &self[range.start..range.end]
    }

    fn to(&self, index: usize) -> Self::RefSlice {
        let end = index.min(self.len());
        &self[0..end]
    }

    fn split_when<W>(&self, when: W) -> Option<(Self::RefSlice, Self::RefSlice)>
    where
        W: Fn(Self::Member) -> bool,
    {
        self.iter()
            .position(|b| when(*b))
            .map(|index| self.split_at(index))
    }
}
