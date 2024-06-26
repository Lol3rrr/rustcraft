use crate::{
    general::{PString, VarInt},
    serialize::SerializeItem,
};
use serde::Serialize;

#[derive(Debug)]
pub struct StatusResponse {
    content: PString<'static>,
}

impl StatusResponse {
    pub fn new(content: &StatusResponseContent) -> Self {
        let content = serde_json::to_string(content).unwrap();

        Self {
            content: PString(std::borrow::Cow::Owned(content)),
        }
    }

    pub fn parse(id: VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        if id.0 != 0x00 {
            return Err(nom::Err::Error(crate::general::ParseError::Other));
        }

        let (i, content) = PString::parse(i)?;

        Ok((i, Self { content }))
    }
}

impl crate::packet::PacketContent for StatusResponse {
    const PACKETTRAIL: bool = false;
    const ID: i32 = 0x00;

    fn length(&self) -> usize {
        self.content.slen()
    }

    fn serialize<'b>(
        &self,
        buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        self.content.serialize(buffer)
    }
}

#[derive(Debug, Serialize)]
pub struct StatusResponseContent {
    pub version: StatusVersion,
    pub players: StatusPlayers,
    pub description: StatusDescription,
    #[serde(rename = "enforcesSecureChat")]
    pub enforces_secure_chat: bool,
    #[serde(rename = "previewsChat")]
    pub previews_chat: bool,
}

#[derive(Debug, Serialize)]
pub struct StatusVersion {
    pub name: String,
    pub protocol: u16,
}

#[derive(Debug, Serialize)]
pub struct StatusPlayers {
    pub max: usize,
    pub online: usize,
    pub sample: Vec<()>,
}

#[derive(Debug, Serialize)]
pub struct StatusDescription {
    pub text: String,
}

#[derive(Debug)]
pub struct PingResponse {
    pub payload: i64,
}

impl PingResponse {
    pub fn parse(id: VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        if id.0 != 0x01 {
            return Err(nom::Err::Error(crate::general::ParseError::Other));
        }

        let (i, payload) = nom::number::streaming::be_i64(i)?;

        Ok((i, Self { payload }))
    }
}

impl crate::packet::PacketContent for PingResponse {
    const PACKETTRAIL: bool = false;
    const ID: i32 = 0x01;

    fn length(&self) -> usize {
        8
    }

    fn serialize<'b>(
        &self,
        buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        crate::serialize::SerializeItem::serialize(&self.payload, buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_response() {
        let content = StatusResponseContent {
            version: StatusVersion {
                name: "1.20.6".into(),
                protocol: 766,
            },
            players: StatusPlayers {
                max: 5,
                online: 0,
                sample: Vec::new(),
            },
            description: StatusDescription {
                text: "testing".into(),
            },
            enforces_secure_chat: false,
            previews_chat: false,
        };
        let packet = crate::packet::Packet {
            inner: StatusResponse::new(&content),
        };

        let serialized = packet.serialize();
        dbg!(&serialized);
    }
}
