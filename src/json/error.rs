use std::error;
use std::fmt;
use std::io;

use de::{Token, TokenKind};

/// The errors that can arise while parsing a JSON stream.
#[derive(Clone, PartialEq)]
pub enum ErrorCode {
    ConversionError(Token),
    EOFWhileParsingList,
    EOFWhileParsingObject,
    EOFWhileParsingString,
    EOFWhileParsingValue,
    ExpectedColon,
    ExpectedConversion,
    ExpectedEnumEnd,
    ExpectedEnumEndToken,
    ExpectedEnumMapStart,
    ExpectedEnumToken,
    ExpectedEnumVariantString,
    ExpectedListCommaOrEnd,
    ExpectedName,
    ExpectedObjectCommaOrEnd,
    ExpectedSomeIdent,
    ExpectedSomeValue,
    ExpectedTokens(Token, &'static [TokenKind]),
    InvalidEscape,
    InvalidNumber,
    InvalidUnicodeCodePoint,
    KeyMustBeAString,
    LoneLeadingSurrogateInHexEscape,
    MissingField(&'static str),
    NotFourDigit,
    NotUtf8,
    TrailingCharacters,
    UnexpectedEndOfHexEscape,
    UnexpectedName(Token),
    UnknownVariant,
    UnrecognizedHex,
}

impl fmt::Show for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ErrorCode::ConversionError(ref token) => write!(f, "failed to convert {:?}", token),
            ErrorCode::EOFWhileParsingList => "EOF While parsing list".fmt(f),
            ErrorCode::EOFWhileParsingObject => "EOF While parsing object".fmt(f),
            ErrorCode::EOFWhileParsingString => "EOF While parsing string".fmt(f),
            ErrorCode::EOFWhileParsingValue => "EOF While parsing value".fmt(f),
            ErrorCode::ExpectedColon => "expected `:`".fmt(f),
            ErrorCode::ExpectedConversion => "expected conversion".fmt(f),
            ErrorCode::ExpectedEnumEnd => "expected enum end".fmt(f),
            ErrorCode::ExpectedEnumEndToken => "expected enum map end".fmt(f),
            ErrorCode::ExpectedEnumMapStart => "expected enum map start".fmt(f),
            ErrorCode::ExpectedEnumToken => "expected enum token".fmt(f),
            ErrorCode::ExpectedEnumVariantString => "expected variant".fmt(f),
            ErrorCode::ExpectedListCommaOrEnd => "expected `,` or `]`".fmt(f),
            ErrorCode::ExpectedName => "expected name".fmt(f),
            ErrorCode::ExpectedObjectCommaOrEnd => "expected `,` or `}`".fmt(f),
            ErrorCode::ExpectedSomeIdent => "expected ident".fmt(f),
            ErrorCode::ExpectedSomeValue => "expected value".fmt(f),
            ErrorCode::ExpectedTokens(ref token, tokens) => write!(f, "expected {:?}, found {:?}", tokens, token),
            ErrorCode::InvalidEscape => "invalid escape".fmt(f),
            ErrorCode::InvalidNumber => "invalid number".fmt(f),
            ErrorCode::InvalidUnicodeCodePoint => "invalid unicode code point".fmt(f),
            ErrorCode::KeyMustBeAString => "key must be a string".fmt(f),
            ErrorCode::LoneLeadingSurrogateInHexEscape => "lone leading surrogate in hex escape".fmt(f),
            ErrorCode::MissingField(ref field) => write!(f, "missing field \"{}\"", field),
            ErrorCode::NotFourDigit => "invalid \\u escape (not four digits)".fmt(f),
            ErrorCode::NotUtf8 => "contents not utf-8".fmt(f),
            ErrorCode::TrailingCharacters => "trailing characters".fmt(f),
            ErrorCode::UnexpectedEndOfHexEscape => "unexpected end of hex escape".fmt(f),
            ErrorCode::UnexpectedName(ref name) => write!(f, "unexpected name {:?}", name),
            ErrorCode::UnknownVariant => "unknown variant".fmt(f),
            ErrorCode::UnrecognizedHex => "invalid \\u escape (unrecognized hex)".fmt(f),
        }
    }
}

#[derive(Clone, PartialEq, Show)]
pub enum Error {
    /// msg, line, col
    SyntaxError(ErrorCode, uint, uint),
    IoError(io::IoError),
    ExpectedError(String, String),
    MissingFieldError(String),
    UnknownVariantError(String),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::SyntaxError(..) => "syntax error",
            Error::IoError(ref error) => error.description(),
            Error::ExpectedError(ref expected, _) => expected.as_slice(),
            Error::MissingFieldError(_) => "missing field",
            Error::UnknownVariantError(_) => "unknown variant",
        }
    }

    fn detail(&self) -> Option<String> {
        match *self {
            Error::SyntaxError(ref code, line, col) => {
                Some(format!("{:?} at line {:?} column {:?}", code, line, col))
            }
            Error::IoError(ref error) => error.detail(),
            Error::ExpectedError(ref expected, ref found) => {
                Some(format!("expected {:?}, found {:?}", expected, found))
            }
            Error::MissingFieldError(ref field) => {
                Some(format!("missing field {:?}", field))
            }
            Error::UnknownVariantError(ref variant) => {
                Some(format!("unknown variant {:?}", variant))
            }
        }
    }
}

impl error::FromError<io::IoError> for Error {
    fn from_error(error: io::IoError) -> Error {
        Error::IoError(error)
    }
}
