mod cursor;
mod error;
mod sequence;
mod view;

// FIXME: Consolidate / unify trait implementations to avoid duplicating logic.
pub mod prelude {
    pub use crate::{cursor::Cursor, sequence::Sequence, view::View};
}

#[cfg(test)]
mod tests {
    use prelude::*;

    use super::*;

    #[test]
    fn views_can_inspect_input() {
        let buffer = include_bytes!("../mock/apt-listing");
        let n = buffer.len();
        assert!(n > 0, "buffer must contain at least one byte.");

        let mut cursor = Cursor::new(&buffer[..1024]);

        if cursor.has_remaining() {
            let (before, after) = cursor.split_at(256);
            assert_eq!(
                before.len(),
                256,
                "before view should be 256 bytes in length"
            );
            assert_eq!(
                after.len(),
                1024 - 256,
                "after view should be [1024 - 256] bytes in length"
            );

            let view = cursor.peek_to(12).expect("peek_to should contain bytes");
            assert_eq!(view.len(), 12, "view should be: `(..12)`");
            assert_eq!(
                cursor.remaining(),
                1024 - 256,
                "view 1 should be equal to range length (256..512)"
            );
            assert!(cursor.has_remaining());
        }

        assert!(!buffer.is_empty());
    }

    #[test]
    fn cursor_iteration() {
        let buffer = include_bytes!("../mock/apt-listing");
        let len = buffer.len();
        assert!(len > 0, "buffer must contain at least one byte.");

        let mut cursor = Cursor::new(buffer);
        assert!(
            cursor.remaining() > 0,
            "cursor inner value should be greater than 0"
        );

        assert_eq!(cursor.next_item(), Some((1, &b'P')));
        assert_eq!(cursor.next_item(), Some((2, &b'a')));
        assert_eq!(cursor.next_item(), Some((3, &b'c')));
        assert_eq!(cursor.next_item(), Some((4, &b'k')));
        assert_eq!(cursor.next_item(), Some((5, &b'a')));
    }

    #[test]
    fn bytes_to_text() {
        let buffer = include_bytes!("../mock/apt-listing");
        let len = buffer.len();
        assert!(len > 0, "buffer must contain at least one byte.");

        let cursor = Cursor::new(buffer);
        assert!(
            cursor.remaining() > 0,
            "cursor inner value should be greater than 0"
        );

        let bytes = String::from_utf8_lossy(&cursor.get_ref()[..7]).to_string();

        assert_eq!(bytes, "Package");
    }
}
