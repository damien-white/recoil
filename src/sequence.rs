//! This module contains iteration and enumeration implementations.

use core::{
    iter::{Copied, Enumerate},
    slice::Iter,
};

/// Sequences represent contiguous slices of memory.
pub trait Sequence {
    /// The type being iterated over.
    ///
    /// The sequence is comprised of items, such as `u8`.
    type Item;

    /// An iterator that  iterator over the sequence, producing individual items.
    type Iter: Iterator<Item = Self::Item>;

    /// An enumerating iterator, producing pairs of indices and items.
    ///
    /// The position always refers to a byte index.
    type Enum: Iterator<Item = (usize, Self::Item)>;

    /// Returns an iterator over the items of the sequence.
    ///
    /// Note: This iterator uses `.copied()` to return an owned iterator.
    fn iter_copied(&self) -> Self::Iter;

    /// Returns an enumerating iterator. Each iteration contains a byte index and item.
    fn iter_indices(&self) -> Self::Enum;
}

impl<'items> Sequence for &'items [u8] {
    type Item = u8;

    type Iter = Copied<Iter<'items, Self::Item>>;

    type Enum = Enumerate<Self::Iter>;

    fn iter_copied(&self) -> Self::Iter {
        self.iter().copied()
    }

    fn iter_indices(&self) -> Self::Enum {
        self.iter_copied().enumerate()
    }
}

// Helper macro for quickly printing a byte slice to the console in various formats.
macro_rules! print_bytes {
    ($encoded:ident) => {{
        let mut dest = String::new();
        writeln!(&mut dest, "\n[bytes]: {:?}", $encoded)
            .expect("failed to write formatted output to buffer.");
        write!(&mut dest, "[chars]: ").expect("failed to write formatted output to buffer.");

        ::std::string::String::from_utf8_lossy($encoded)
            .to_string()
            .chars()
            .for_each(|c| {
                write!(&mut dest, "{:^5}", c).expect("failed to write formatted output to buffer.");
            });

        writeln!(&mut dest, "\n").expect("failed to write newline to end of dest buffer.");
        println!("{}", dest);
    }};
}

#[cfg(test)]
mod tests {
    use core::fmt::Write;

    use crate::prelude::{Cursor, View};

    use super::*;

    #[test]
    fn sequence_remaining_length() {
        let input = &br#"In the previous section we tried running our unsafe
singly-linked queue under miri, and it said we had broken the rules of
stacked borrows, and linked us some documentation.
"#[..];
        // let offset = 0;
        let len = input.len();
        let mut cursor = Cursor::new(input);

        cursor.advance(7);
        let end = cursor
            .take_while(|&(_, b)| *b != b' ')
            .map(|(pos, _)| pos)
            .last()
            .unwrap();
        let actual = cursor.view(0..end);
        let expected = input.view(7..7 + end);

        print_bytes!(actual);

        assert_eq!(
            actual, expected,
            "Expected `{:?}` bytes, but got: `{:?}`",
            expected, actual
        );
    }
}
