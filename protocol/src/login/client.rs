use crate::{
    general::{PString, VarInt},
    serialize::SerializeItem,
};

#[derive(Debug, PartialEq)]
pub struct EncryptionRequest {
    pub server_id: PString<'static>,
    pub pubkey: Vec<u8>,
    pub verifytoken: Vec<u8>,
}

impl EncryptionRequest {
    pub fn parse(id: VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        if id.0 != 0x01 {
            return Err(nom::Err::Error(crate::general::ParseError::Other));
        }

        let (i, server_id) = PString::parse(i)?;

        let (i, pubkey_len) = VarInt::parse(i)?;

        let pubkey = &i[..pubkey_len.0 as usize];
        let i = &i[pubkey_len.0 as usize..];

        let (i, verify_len) = VarInt::parse(i)?;

        let verifytoken = &i[..verify_len.0 as usize];
        let i = &i[verify_len.0 as usize..];

        let i = &i[1..];

        Ok((
            i,
            Self {
                server_id,
                pubkey: pubkey.to_vec(),
                verifytoken: verifytoken.to_vec(),
            },
        ))
    }
}

impl crate::packet::PacketContent for EncryptionRequest {
    const PACKETTRAIL: bool = true;
    const ID: i32 = 0x01;

    fn length(&self) -> usize {
        self.server_id.slen()
            + VarInt(self.pubkey.len() as i32).slen()
            + self.pubkey.len()
            + VarInt(self.verifytoken.len() as i32).slen()
            + self.verifytoken.len() // + 1
    }

    fn serialize<'b>(
        &self,
        mut buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        buffer = self.server_id.serialize(buffer)?;
        buffer = self.pubkey.serialize(buffer)?;
        buffer = self.verifytoken.serialize(buffer)?;
        Ok(buffer)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LoginSuccess {
    pub uuid: u128,
    pub name: PString<'static>,
    pub properites: Vec<Property>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Property {
    pub name: PString<'static>,
    pub value: PString<'static>,
    pub signature: Option<PString<'static>>,
}

impl crate::packet::PacketContent for LoginSuccess {
    const PACKETTRAIL: bool = true;
    const ID: i32 = 0x02;

    fn length(&self) -> usize {
        16 + self.name.slen() + self.properites.slen()
    }

    fn serialize<'b>(
        &self,
        mut buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        (buffer[..16]).copy_from_slice(&self.uuid.to_be_bytes());
        buffer = &mut buffer[16..];

        buffer = self.name.serialize(buffer)?;
        buffer = self.properites.serialize(buffer)?;

        Ok(buffer)
    }
}

impl crate::serialize::SerializeItem for Property {
    fn slen(&self) -> usize {
        self.name.slen()
            + self.value.slen()
            + 1
            + self.signature.as_ref().map(|s| s.slen()).unwrap_or(0)
    }

    fn serialize<'b>(
        &self,
        mut buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        buffer = self.name.serialize(buffer)?;
        buffer = self.value.serialize(buffer)?;

        match self.signature.as_ref() {
            Some(sig) => {
                buffer = true.serialize(buffer)?;
                buffer = sig.serialize(buffer)?;
            }
            None => {
                buffer = false.serialize(buffer)?;
            }
        };

        Ok(buffer)
    }

    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        todo!()
    }
}

impl LoginSuccess {
    pub fn parse(id: VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        if id.0 != 0x02 {
            return Err(nom::Err::Error(crate::general::ParseError::Other));
        }

        let (i, uuid) = nom::number::streaming::be_u128(i)?;
        let (i, name) = PString::parse(i)?;
        let (i, prop_count) = VarInt::parse(i)?;

        let i = &i[1..];

        Ok((
            i,
            Self {
                uuid,
                name,
                properites: Vec::new(),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use base64::prelude::*;

    use super::*;
    use crate::packet::Packet;

    #[test]
    #[ignore = "How do we deal with the trailing 1 at the end?"]
    fn serialize_parse() {
        let msg = LoginSuccess {
            uuid: 123456789,
            name: PString("testing".into()),
            properites: Vec::new(),
        };
        let packet = Packet { inner: msg };

        let mut serialized = packet.serialize();

        let (rem, parsed_packet) = Packet::parse(LoginSuccess::parse)(&serialized).unwrap();
        assert_eq!(&[] as &[u8], rem);

        assert_eq!(packet, parsed_packet);
    }

    #[test]
    #[ignore = "How do we deal with the trailing 1 at the end?"]
    fn testing() {
        let data = [
            0xac, 0x01, 0x01, 0x00, 0xa2, 0x01, 0x30, 0x81, 0x9f, 0x30, 0x0d, 0x06, 0x09, 0x2a,
            0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01, 0x05, 0x00, 0x03, 0x81, 0x8d, 0x00,
            0x30, 0x81, 0x89, 0x02, 0x81, 0x81, 0x00, 0xbb, 0xfc, 0xaf, 0x20, 0xed, 0xb2, 0x17,
            0x97, 0xda, 0xdd, 0xfd, 0xf4, 0x91, 0x17, 0x19, 0x55, 0x52, 0x21, 0x89, 0x84, 0x6c,
            0x77, 0x02, 0x49, 0x95, 0x2d, 0x2d, 0xaf, 0x50, 0xd7, 0xc7, 0x48, 0x66, 0x4d, 0xae,
            0xcd, 0xb7, 0x14, 0x9e, 0xe7, 0xe9, 0xef, 0x6d, 0xaa, 0xd2, 0x27, 0x9d, 0xf7, 0x52,
            0x40, 0xab, 0x19, 0x50, 0x2f, 0x0d, 0x16, 0xe1, 0x51, 0x20, 0xce, 0x74, 0x03, 0x68,
            0xe5, 0xa8, 0x76, 0x20, 0x72, 0xcc, 0x7b, 0x40, 0x3b, 0xe4, 0x2b, 0x07, 0x81, 0x3f,
            0x0c, 0x2c, 0xd6, 0x22, 0x57, 0x2b, 0xfa, 0x8d, 0x70, 0xe2, 0xa6, 0x5a, 0x59, 0x06,
            0x32, 0x7c, 0xdf, 0x74, 0xd5, 0x7f, 0xf0, 0xae, 0x5e, 0x1e, 0xb2, 0xb2, 0x2a, 0x73,
            0x4c, 0x0d, 0xe8, 0x96, 0x0f, 0xf7, 0xc2, 0x84, 0x14, 0x3c, 0x12, 0xbf, 0x3c, 0xc1,
            0x23, 0x20, 0x6d, 0x40, 0x29, 0xab, 0xac, 0xc7, 0xad, 0x02, 0x03, 0x01, 0x00, 0x01,
            0x04, 0x85, 0xea, 0x8c, 0xab, 0x01,
        ];

        let (rem, parsed) = Packet::parse(EncryptionRequest::parse)(&data).unwrap();
        dbg!(rem, &parsed);

        dbg!(BASE64_STANDARD.encode(&parsed.inner.pubkey));

        // todo!();
    }
}
