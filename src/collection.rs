//! This module contains iteration and enumeration implementations.

use core::iter::{Copied, Enumerate};
use core::ops::Deref;
use core::slice::Iter;
use core::str::{CharIndices, Chars};

use crate::span::Span;

/// Iterable slices of memory that represent contiguous slices of borrowed
/// memory.
///
/// The `Collection` trait defines the behavior of an allocated, readonly slice
/// of contiguous memory. Types that implement `Collection` often also implement
/// the [`Span`] trait.
///
/// Please note: The memory owned by the Iterable is always safe to read and
/// copy. However, these invariants do not hold if mutation occurs. Mutation, or
/// write operations, destroy the integrity of the data.
///
/// The `Collection` trait provides methods that are guaranteed to be both memory
/// safe and non-destructive. That is, no methods in `Collection` will cause or
/// introduce undefined behavior (UB).
///
/// It provides methods for extending a type with iteration and enumeration.
///
/// [`Span`]: crate::span::Span
pub trait Collection: PartialEq<Self> {
    /// The type of element that comprises the sequence.
    ///
    /// All Sequence types provide simple access to iteration, producing `Element` types
    /// produced when iterating over the sequence comprising the sequence. type being iterated over.
    ///
    /// The sequence is comprised of items, such as `u8`.
    type Item: ?Sized + Clone + Ord;

    /// An iterator that  iterator over the sequence, producing individual items.
    type Items: Iterator<Item = Self::Item>;

    /// An enumerating iterator, producing pairs of indices and items.
    ///
    /// The position always refers to a byte index.
    type EnumItems: Iterator<Item = (usize, Self::Item)>;

    /// Returns an iterator over the items of the sequence.
    ///
    /// This method performs a bitwise copy operation on the underlying data. This
    /// is typically not a problem as the data is cheap to copy.
    fn as_iter(&self) -> Self::Items;

    /// Returns an enumerating iterator. Each iteration contains a byte index and item.
    fn as_enum(&self) -> Self::EnumItems;
}

impl<'a> Collection for &'a [u8] {
    type Item = u8;

    type Items = Copied<Iter<'a, Self::Item>>;

    type EnumItems = Enumerate<Self::Items>;

    fn as_iter(&self) -> Self::Items {
        self.iter().copied()
    }

    fn as_enum(&self) -> Self::EnumItems {
        self.as_iter().enumerate()
    }
}

impl<'a> Collection for &'a str {
    type Item = char;

    type Items = Chars<'a>;

    type EnumItems = CharIndices<'a>;

    fn as_iter(&self) -> Self::Items {
        self.chars()
    }

    fn as_enum(&self) -> Self::EnumItems {
        self.char_indices()
    }
}

/// Extensions that help to unify the `&str` and `&[u8]` types.
///
/// The `Input` trait is used as the primary abstraction layer over input types.
/// If you are using a custom data type, you must implement `Input` for your
/// type.
///
/// Parser subroutines work on `&str` and `&[u8]` by default.
///
/// To extend the capabilites of `Input`, you may create a new type. The data
/// you are consuming must be compatible with the `&str` or `&[u8]` slice types.
///
/// Providing an abstraction over the `Input` type allows the crate's API to be
/// more minimal and allows library authors to make more powerful design decisions.
/// Recoil's goals are high-performance, memory-safety and no-std capable.
pub trait Input<I>: Span {
    /// Individual member of the input.
    type Token;

    type Slice: Collection + Span;

    fn as_slice(&self) -> Self::RefSlice;
}

impl<'a> Input<char> for &'a str {
    type Token = char;
    type Slice = &'a str;

    fn as_slice(&self) -> Self::Slice {
        let start = self.as_ptr() as usize;
        let end = self.len().max(self.as_ptr() as usize * self.len());
        self.over(start..end)
    }
}

impl<'slice> Input<u8> for &'slice [u8] {
    type Token = u8;
    type Slice = &'slice [u8];

    fn as_slice(&self) -> Self::Slice {
        let start = self.as_ptr() as usize;
        let end = self.len().max(self.as_ptr() as usize * self.len());
        self.over(start..end)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Bytes<'a> {
    inner: &'a [u8],
}

impl<'a> Collection for Bytes<'a> {
    type Item = u8;

    type Items = Copied<Iter<'a, Self::Item>>;

    type EnumItems = Enumerate<Self::Items>;

    fn as_iter(&self) -> Self::Items {
        self.inner.iter().copied()
    }

    fn as_enum(&self) -> Self::EnumItems {
        self.as_iter().enumerate()
    }
}

impl<'a> Bytes<'a> {
    pub fn new(inner: &'a [u8]) -> Self {
        Self { inner }
    }

    pub fn inner(&self) -> &'a [u8] {
        self.inner
    }

    pub fn clear(&mut self) {
        self.inner = &b""[..];
    }

    pub fn next_token(&mut self) -> Option<&[u8]> {
        let mut start = 0;
        let mut end = 0;

        if let Some((first, tail)) = self.split_first() {
            if first.is_ascii_whitespace() {
                start += 1;
                end = start + 1;
            }

            if start >= end {
                end += 1;
            }

            Some(tail)
        } else {
            None
        }
    }
}

impl<'a> Deref for Bytes<'a> {
    type Target = &'a [u8];

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<'a> Iterator for Bytes<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        self.split_first().map(|(first, tail)| {
            self.inner = tail;
            first
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn unified_input_type() {
        let old = "foo ☃☃☃ foo foo quux foo";

        let new = old.replace("foo", "hello");
        assert_eq!(new, "hello ☃☃☃ hello hello quux hello");
    }

    #[test]
    fn collection_tokens() {
        let mut input = Bytes::new(
            &br#"01 234   56\n\t789\r\nAaZ    z
        {
            \"age\":31,
            \"name\":\"unknown\"
        }"#[..],
        );

        let indices: Vec<usize> = vec![];

        assert_eq!(input.next_token(), Some(&b"01"[..]));

        assert_ne!(indices.len(), 0, "indices should not be equal to zero (0).");
    }
}
