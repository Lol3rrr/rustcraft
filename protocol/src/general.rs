mod varint;
pub use varint::VarInt;

mod varlong;
pub use varlong::VarLong;

mod pstring;
pub use pstring::PString;

mod position;
pub use position::Position;

mod bitset;
pub use bitset::BitSet;

mod slot;
pub use slot::Slot;

mod text;
pub use text::TextComponent;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    WrongPacketId { expected: i32, received: i32 },
    UnknownPacketId(i32),
    NegativeLength,
    ParseString,
    RemainingDataAfterParsing { packet_id: VarInt },
    Other,
    NotImplemented(&'static str),
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
