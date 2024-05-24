use crate::general::VarInt;

#[derive(Debug, PartialEq, Eq)]
pub struct StatusRequest {}

#[derive(Debug, PartialEq, Eq)]
pub struct PingRequest {
    pub payload: i64,
}

#[derive(Debug, PartialEq)]
pub enum ServerBound {
    Status(StatusRequest),
    Ping(PingRequest),
}

impl StatusRequest {
    pub fn parse(id: VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        if id.0 != 0x00 {
            return Err(nom::Err::Error(crate::general::ParseError::Other));
        }

        Ok((i, Self {}))
    }
}

impl PingRequest {
    pub fn parse(id: VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        if id.0 != 0x01 {
            return Err(nom::Err::Error(crate::general::ParseError::Other));
        }

        let (rem, payload) = nom::number::streaming::be_i64(i)?;

        Ok((rem, Self { payload }))
    }
}

impl ServerBound {
    pub fn parse(id: VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        match id.0 {
            0x00 => StatusRequest::parse(id, i).map(|(r, v)| (r, Self::Status(v))),
            0x01 => PingRequest::parse(id, i).map(|(r, v)| (r, Self::Ping(v))),
            _ => {
                dbg!(id);
                return Err(nom::Err::Error(crate::general::ParseError::Other));
            }
        }
    }
}
