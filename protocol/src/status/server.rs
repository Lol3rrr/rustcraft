use crate::{general::VarInt, serialize::SerializeItem};

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
impl crate::packet::PacketContent for StatusRequest {
    const ID: i32 = 0x00;
    const PACKETTRAIL: bool = false;

    fn length(&self) -> usize {
        0
    }

    fn serialize<'b>(
        &self,
        buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        Ok(buffer)
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

impl crate::packet::PacketContent for PingRequest {
    const ID: i32 = 0x01;
    const PACKETTRAIL: bool = false;

    fn length(&self) -> usize {
        self.payload.slen()
    }

    fn serialize<'b>(
        &self,
        buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        self.payload.serialize(buffer)
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
