use crate::{
    general::{PString, VarInt},
    serialize::SerializeItem,
};

#[derive(Debug, PartialEq)]
pub struct LoginStart {
    pub name: PString<'static>,
    pub uuid: u128,
}

impl LoginStart {
    pub fn parse(id: VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        if id.0 != 0x00 {
            return Err(nom::Err::Error(crate::general::ParseError::Other));
        }

        let (i, name) = PString::parse(i)?;
        let (i, uuid) = nom::number::streaming::be_u128(i)?;

        Ok((i, Self { name, uuid }))
    }
}

impl crate::packet::PacketContent for LoginStart {
    const ID: i32 = 0x00;
    const PACKETTRAIL: bool = false;

    fn length(&self) -> usize {
        self.name.slen() + self.uuid.slen()
    }

    fn serialize<'b>(
        &self,
        mut buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        buffer = self.name.serialize(buffer)?;
        buffer = self.uuid.serialize(buffer)?;
        Ok(buffer)
    }
}

#[derive(Debug, PartialEq)]
pub struct EncryptionResponse {
    pub shared_secret: Vec<u8>,
    pub verify_token: Vec<u8>,
}

impl EncryptionResponse {
    pub fn parse(id: VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        if id.0 != 0x01 {
            return Err(nom::Err::Error(crate::general::ParseError::Other));
        }

        let (i, secret_length) = VarInt::parse(i)?;
        let secret_length = secret_length.0 as usize;
        let shared_secret = i[..secret_length].to_vec();
        let i = &i[secret_length..];

        let (i, token_length) = VarInt::parse(i)?;
        let token_length = token_length.0 as usize;
        let verify_token = i[..token_length].to_vec();
        let i = &i[token_length..];

        Ok((
            i,
            Self {
                shared_secret,
                verify_token,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct LoginAck {}

impl LoginAck {
    pub fn parse(id: VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        if id.0 != 0x03 {
            return Err(nom::Err::Error(crate::general::ParseError::Other));
        }

        Ok((i, Self {}))
    }
}

impl crate::packet::PacketContent for LoginAck {
    const ID: i32 = 0x03;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::Packet;

    #[test]
    fn serialize_loginstart() {
        let packet = Packet {
            inner: LoginStart {
                name: PString("testing".into()),
                uuid: 123456789,
            },
        };

        let serialized = packet.serialize();
        dbg!(&serialized);

        todo!();
    }
}
