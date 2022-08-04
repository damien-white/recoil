//! # [`RECOIL`][https://docs.rs/recoil/latest/recoil]
//!
//! Recoil offers users with a new approach to building and assembling parsers.

use prelude::Input;

mod error;
use crate::error::{Error, ErrorWithContext};

mod collection;
mod span;

pub type AResult<I, O = I, E = ErrorWithContext<I>> = core::result::Result<(I, O), E>;

pub trait Parser<I, O, E = ErrorWithContext<I>> {
    fn exec(&mut self, input: I) -> AResult<I, O, E>;

    fn map<F, B>(self, f: F) -> Map<Self, F, O>
    where
        F: Fn(O) -> B,
        Self: Sized,
    {
        Map {
            parser: self,
            f,
            phantom: core::marker::PhantomData,
        }
    }
}

impl<'parser, I, O, E, F> Parser<I, O, E> for F
where
    F: FnMut(I) -> AResult<I, O, E> + 'parser,
{
    fn exec(&mut self, input: I) -> AResult<I, O, E> {
        self(input)
    }
}

/// Parser subroutine that returns the result of the subparser. If the result is
/// Incomplete, an error is returned.
#[derive(Debug, Clone, Copy)]
pub struct Complete<P> {
    parser: P,
}

impl<I, O, E, P> Parser<I, O, E> for Complete<P>
where
    P: Parser<I, O, E>,
    E: Error<I>,
    I: Input<I>,
{
    fn exec(&mut self, input: I) -> AResult<I, O, E> {
        self.parser.exec(input)
    }
}

pub struct Map<P, F, B> {
    parser: P,
    f: F,
    phantom: core::marker::PhantomData<B>,
}

impl<I, O1, O2, E, P, F> Parser<I, O2, E> for Map<P, F, O1>
where
    P: Parser<I, O1, E>,
    F: Fn(O1) -> O2,
{
    fn exec(&mut self, input: I) -> AResult<I, O2, E> {
        match self.parser.exec(input) {
            Err(err) => Err(err),
            Ok((input, output)) => Ok((input, (self.f)(output))),
        }
    }
}

/// Create and return an `ErrorMessage` for a given `ErrorKind` and constant
/// message.
#[macro_export]
macro_rules! with_error {
    ($kind:expr, $message:expr $(,)?) => {
        $crate::error::ErrorMessage::from_static_message($kind, $message)
    };
}

pub mod prelude {
    pub use crate::collection::{Collection, Input};
    pub use crate::error::{Error, ErrorKind, ErrorMessage, ErrorSpan, ErrorWithContext};
    pub use crate::span::{ByteSpan, Span, StrSpan};
}

#[cfg(test)]
mod tests {
    use prelude::*;

    use crate::collection::Bytes;

    use super::*;

    #[test]
    fn spans_allow_input_inspection() {
        let buffer = include_bytes!("../mock/resume.pdf");
        let n = buffer.len();
        assert!(n > 0, "buffer must contain at least one byte.");

        let input = Bytes::new(unsafe { core::str::from_utf8_unchecked(&buffer[..]).as_ref() });

        let input = input.inner().over(0..1024);

        if input.len() % 512 == 0 && !input.is_empty() {
            let view1 = &input[..256];
            let view2 = &input[256..512];
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
    fn bytes_docker_compose_file() {
        let buffer = match core::str::from_utf8(include_bytes!("../mock/docker-compose.yml")) {
            Ok(inner) => inner,
            Err(error) => panic!("bytes should be valid UTF-8; error = {error:?}"),
        };

        // Create `Bytes` from valid UTF-8 data.
        let mut bytes = Bytes::new(buffer.as_bytes());
        assert!(
            bytes.len() > 0,
            "cursor inner value should be greater than 0"
        );

        assert_eq!(bytes.next(), Some(&b'v'));
        assert_eq!(bytes.next(), Some(&b"e"[0]));
        assert_eq!(bytes.next(), Some(&b'r'));
        assert_eq!(bytes.next(), Some(&b"s"[0]));
        assert_eq!(bytes.next(), Some(&b'i'));
    }

    #[test]
    fn bytes_to_text() {
        let buf = include_bytes!("../mock/docker-compose.yml");
        let len = buf.len();
        assert!(len > 0, "buffer must contain at least one byte.");

        let input: &[u8] = core::str::from_utf8(buf.as_slice()).unwrap().as_ref();
        assert!(
            !input.is_empty(),
            "The `Bytes` container should not be empty. Length should be greater than 0."
        );

        let bytes = unsafe { core::str::from_utf8_unchecked(input.to(7)) };

        assert_eq!(bytes, ": '3.9'");
    }
}
