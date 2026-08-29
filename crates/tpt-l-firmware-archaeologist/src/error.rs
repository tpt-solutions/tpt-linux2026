//! Error types for firmware parsing.

use core::fmt;

/// Failure modes encountered while parsing firmware blobs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub offset: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    /// The blob (or a sub-structure) was shorter than required.
    TooShort,
    /// An expected magic / signature did not match.
    BadSignature { expected: [u8; 4], found: [u8; 4] },
    /// A reserved / unexpected value was encountered.
    InvalidValue(&'static str),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ParseErrorKind::TooShort => {
                write!(f, "unexpected end of data at offset {}", self.offset)
            }
            ParseErrorKind::BadSignature { expected, found } => write!(
                f,
                "bad signature at offset {}: expected {:?}, found {:?}",
                self.offset, expected, found
            ),
            ParseErrorKind::InvalidValue(msg) => {
                write!(f, "invalid value at offset {}: {}", self.offset, msg)
            }
        }
    }
}

impl core::error::Error for ParseError {}

impl ParseError {
    pub(crate) fn too_short(offset: usize) -> Self {
        ParseError { kind: ParseErrorKind::TooShort, offset }
    }
    pub(crate) fn bad_signature(offset: usize, expected: [u8; 4], found: [u8; 4]) -> Self {
        ParseError { kind: ParseErrorKind::BadSignature { expected, found }, offset }
    }
    pub(crate) fn invalid(offset: usize, msg: &'static str) -> Self {
        ParseError { kind: ParseErrorKind::InvalidValue(msg), offset }
    }
}

/// Convenience `Result` alias used across parser modules.
pub type Result<T> = core::result::Result<T, ParseError>;
