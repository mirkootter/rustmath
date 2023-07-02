pub enum ErrorKind {
    FromNom(nom::error::ErrorKind),
    InvalidDelimiter,
}

impl From<nom::error::ErrorKind> for ErrorKind {
    fn from(error_kind: nom::error::ErrorKind) -> Self {
        Self::FromNom(error_kind)
    }
}

#[allow(dead_code)] // we may need them for debugging
pub struct Error<I> {
    input: I,
    kind: ErrorKind,
}

impl<I> nom::error::ParseError<I> for Error<I> {
    fn from_error_kind(input: I, kind: nom::error::ErrorKind) -> Self {
        Self {
            input,
            kind: kind.into(),
        }
    }

    fn append(_input: I, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

pub fn make_recoverable_error<T>(input: &str, kind: ErrorKind) -> super::ParseResult<T> {
    Err(nom::Err::Error(Error { input, kind }))
}
