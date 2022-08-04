use crate::collection::Input;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct StatusCode(u16);

/// Error type with minimal contextual information.
///
/// This type should be preferred if the only thing that matters is performance.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct MinimalError<I: Input<I>> {
    /// Position of the error within the given input.
    input: I,
    /// Error code represented as a u16. Used to look up error by code.
    status_code: StatusCode,
}

impl<I: Input<I>> MinimalError<I> {
    pub fn new(input: I, status_code: StatusCode) -> Self {
        Self { input, status_code }
    }
}

/// Error type with rich contextual information.
///
/// This type should be used whenever additional information, such as the input
/// and error location(s) are needed. If detailed errors are not important, the
/// `MinimalError` type should be preferred.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ErrorSpan<'a> {
    /// Position of the error within the given input.
    input: &'a [u8],
    /// Error code represented as a u16. Used to look up error by code.
    offset: Option<(usize, usize)>,
}

impl<'a> ErrorSpan<'a> {
    pub fn new(input: &'a [u8], start: usize, end: usize) -> Self {
        debug_assert!(start <= end);
        ErrorSpan {
            input,
            offset: Some((start, end)),
        }
    }

    pub fn start(&self) -> usize {
        self.input.as_ptr() as usize
    }

    pub fn end(&self) -> usize {
        self.input.as_ref().len()
    }
}

/// Error type representing a parser subroutine failure.
///
/// `Error` contains contextual information, such as the error span and
/// code. This data is used to construct `ParserError` instances containing
/// useful information.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ErrorWithContext<I: Input<I>> {
    input: I,
    offset: Option<(usize, usize)>,
    code: u16,
}

impl<I: Input<I>> ErrorWithContext<I> {
    pub fn new(input: I, offset: Option<(usize, usize)>, code: u16) -> Self {
        if let Some((start, end)) = offset {
            debug_assert!(start <= end);
            Self {
                input,
                offset: Some((start, end)),
                code,
            }
        } else {
            Self {
                input,
                offset: None,
                code,
            }
        }
    }

    pub fn kind(&self) -> ErrorKind {
        use ErrorKind::*;
        // Use the error's `code` to match against `ErrorKind` discriminant.
        match self.code {
            0 => EndOfInput,
            1 => IncompatibleTypes,
            2 => MalformedData,
            3 => MissingData,
            4 => Unknown,
            _ => ErrorKind::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    /// Reached end of input, or EOF, while waiting on data.
    EndOfInput,
    /// Type conversion operation failed due to incompatible types.
    IncompatibleTypes,
    /// Unrecognized format or malformed data.
    MalformedData,
    /// Missing expected or required data.
    MissingData,
    /// An unknown or explicitly unspecified error has occurred.
    #[default]
    Unknown,
}

impl ErrorKind {
    /// Returns the string representation of the error kind.
    pub(crate) fn as_str(&self) -> &'static str {
        use ErrorKind::*;
        match *self {
            EndOfInput => "Reached end of input, or EOF, while waiting on data.",
            IncompatibleTypes => "Input and output types must be compatible.",
            MalformedData => "Received invalid or malformed data.",
            MissingData => "Received incomplete or missing data.",
            Unknown => "Failure caused by unknown or unexpected error.",
        }
    }

    /// Returns the status code of the error kind.
    pub(crate) fn as_code(&self) -> StatusCode {
        use ErrorKind::*;
        match *self {
            EndOfInput => StatusCode(0),
            IncompatibleTypes => StatusCode(1),
            MalformedData => StatusCode(2),
            MissingData => StatusCode(3),
            Unknown => StatusCode(4),
        }
    }
}

impl From<StatusCode> for ErrorKind {
    fn from(code: StatusCode) -> Self {
        match code {
            StatusCode(0) => ErrorKind::EndOfInput,
            StatusCode(1) => ErrorKind::IncompatibleTypes,
            StatusCode(2) => ErrorKind::MalformedData,
            StatusCode(3) => ErrorKind::MissingData,
            StatusCode(4) => ErrorKind::Unknown,
            _ => ErrorKind::default(),
        }
    }
}

impl From<ErrorKind> for StatusCode {
    fn from(kind: ErrorKind) -> Self {
        match kind {
            ErrorKind::EndOfInput => StatusCode(0),
            ErrorKind::IncompatibleTypes => StatusCode(1),
            ErrorKind::MalformedData => StatusCode(2),
            ErrorKind::MissingData => StatusCode(3),
            ErrorKind::Unknown => StatusCode(4),
        }
    }
}

pub trait Error<I>: Sized {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self;

    fn append(input: I, kind: ErrorKind, other: Self) -> Self;

    fn or(self, other: Self) -> Self {
        other
    }
}

#[derive(Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct ErrorMessage {
    kind: ErrorKind,
    message: &'static str,
}

impl ErrorMessage {
    pub const DEFAULT_MESSAGE: &str = "Unknown error caused by a parser failure has occurred.";

    pub const fn new(kind: ErrorKind, message: &'static str) -> Self {
        Self { kind, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ErrorKind::*;

    #[test]
    fn error_message_smoke_test() {
        let error = ErrorMessage::new(ErrorKind::MissingData, ErrorKind::MissingData.as_str());
        assert_eq!(
            error.kind, MissingData,
            "error message should contain a valid `kind`."
        );
        assert_eq!(
            ErrorKind::from(error.kind.as_code()),
            MissingData,
            "error message should contain a valid `kind`."
        );
    }
}
