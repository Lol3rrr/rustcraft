mod varint;
pub use varint::VarInt;

mod pstring;
pub use pstring::PString;

mod position;
pub use position::Position;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    NegativeLength,
    ParseString,
    Other,
}

impl nom::error::ParseError<&[u8]> for ParseError {
    fn from_error_kind(input: &[u8], kind: nom::error::ErrorKind) -> Self {
        Self::Other
    }
    fn append(input: &[u8], kind: nom::error::ErrorKind, other: Self) -> Self {
        Self::Other
    }

    fn from_char(input: &[u8], _: char) -> Self {
        Self::Other
    }
    fn or(self, other: Self) -> Self {
        Self::Other
    }
}
