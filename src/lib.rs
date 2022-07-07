mod cursor;
pub mod error;
pub mod sequence;
mod slice;
mod view;

// FIXME: Consolidate / unify trait implementations to avoid duplicating logic.

pub use cursor::Cursor;
pub use slice::Slice;
pub use view::View;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn views_can_inspect_input() {
        let buffer = include_bytes!("../mock/resume.pdf");
        let n = buffer.len();
        assert!(n > 0, "buffer must contain at least one byte.");

        let cursor = Cursor::new(buffer);

        let input = cursor.view(0..1024);

        if input.has_remaining() {
            let view1 = input.slice(..256);
            let view2 = input.slice(256..512);
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
        }

        assert!(!input.is_empty());
        assert_eq!(input.len(), 1024);
    }

    #[test]
    fn cursor_iteration() {
        let buffer = include_bytes!("../mock/docker-compose.yml");
        let len = buffer.len();
        assert!(len > 0, "buffer must contain at least one byte.");

        let mut cursor = Cursor::new(buffer);
        assert!(
            cursor.remaining() > 0,
            "cursor inner value should be greater than 0"
        );

        assert_eq!(cursor.next_item(), Some((1, &b'v')));
        assert_eq!(cursor.next_item(), Some((2, &b'e')));
        assert_eq!(cursor.next_item(), Some((3, &b'r')));
        assert_eq!(cursor.next_item(), Some((4, &b's')));
        assert_eq!(cursor.next_item(), Some((5, &b'i')));
    }

    #[test]
    fn bytes_to_text() {
        let buffer = include_bytes!("../mock/docker-compose.yml");
        let len = buffer.len();
        assert!(len > 0, "buffer must contain at least one byte.");

        let cursor = Cursor::new(buffer);
        assert!(
            cursor.remaining() > 0,
            "cursor inner value should be greater than 0"
        );

        let bytes = String::from_utf8_lossy(&cursor.inner()[..7]).to_string();

        assert_eq!(bytes, "version");
    }
}
