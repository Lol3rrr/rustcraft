//! All the Messages relating to the `Status` state of a connection

pub mod client {
    use crate::general::VarInt;
    use serde::Serialize;

    pub struct StatusResponse {
        content: String,
    }

    impl StatusResponse {
        pub fn new(content: &StatusResponseContent) -> Self {
            let content = serde_json::to_string(content).unwrap();

            Self {
                content,
            }
        }
    }

    impl crate::packet::PacketContentSerializer for StatusResponse {
        const PACKETTRAIL: bool = false;
        const ID: i32 = 0x00;

        fn length(&self) -> usize {
            5 + self.content.len()
        }

        fn serialize(&self, mut buffer: &mut [u8]) -> usize {

            let written = VarInt(self.content.len() as i32).serialize(&mut buffer);
            buffer = &mut buffer[written..];

            (&mut buffer[..self.content.len()]).copy_from_slice(self.content.as_bytes());

            written + self.content.len()
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

    pub struct PingResponse {
        pub payload: i64,
    }

    impl crate::packet::PacketContentSerializer  for PingResponse {
        const PACKETTRAIL: bool = false;
        const ID: i32 = 0x01;

        fn length(&self) -> usize {
            8
        }

        fn serialize(&self, buffer: &mut [u8]) -> usize {
            (buffer[..8]).copy_from_slice(&self.payload.to_be_bytes());
            8
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

            todo!()
        }
    }
}

pub mod server {
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
        pub fn parse(
            id: VarInt,
            i: &[u8],
        ) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
            if id.0 != 0x00 {
                return Err(nom::Err::Error(crate::general::ParseError::Other));
            }

            Ok((i, Self {}))
        }
    }

    impl PingRequest {
        pub fn parse(
            id: VarInt,
            i: &[u8],
        ) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
            if id.0 != 0x01 {
                return Err(nom::Err::Error(crate::general::ParseError::Other));
            }

            let (rem, payload) = nom::number::streaming::be_i64(i)?;

            Ok((rem, Self { payload }))
        }
    }

    impl ServerBound {
        pub fn parse(
            id: VarInt,
            i: &[u8],
        ) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
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
}
