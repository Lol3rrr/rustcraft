//! All the Messages relating to the `Login` state of a connection

pub mod client {
    use crate::general::{PString, VarInt};

    #[derive(Debug, PartialEq)]
    pub struct EncryptionRequest {
        pub server_id: PString<'static>,
        pub pubkey: Vec<u8>,
        pub verifytoken: Vec<u8>,
    }

    impl EncryptionRequest {
        pub fn parse(
            id: VarInt,
            i: &[u8],
        ) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
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

    impl crate::packet::PacketContentSerializer for EncryptionRequest {
        const PACKETTRAIL: bool = true;
        const ID: i32 = 0x01;

        fn length(&self) -> usize {
            self.server_id.serialize_length()
                + VarInt(self.pubkey.len() as i32).serialize_length()
                + self.pubkey.len()
                + VarInt(self.verifytoken.len() as i32).serialize_length()
                + self.verifytoken.len() // + 1
        }

        fn serialize(&self, mut buffer: &mut [u8]) -> usize {
            let mut sum = 0;

            let written = self.server_id.serialize(buffer);
            sum += written;
            buffer = &mut buffer[written..];

            let pubkey_length = VarInt(self.pubkey.len() as i32);
            let written = pubkey_length.serialize(buffer);
            sum += written;
            buffer = &mut buffer[written..];

            (buffer[..self.pubkey.len()]).copy_from_slice(&self.pubkey);
            sum += self.pubkey.len();
            buffer = &mut buffer[self.pubkey.len()..];

            let verify_length = VarInt(self.verifytoken.len() as i32);
            let written = verify_length.serialize(buffer);
            sum += written;
            buffer = &mut buffer[written..];

            (buffer[..self.verifytoken.len()]).copy_from_slice(&self.verifytoken);
            sum += self.verifytoken.len();

            sum
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

    impl crate::packet::PacketContentSerializer for LoginSuccess {
        const PACKETTRAIL: bool = true;
        const ID: i32 = 0x02;

        fn length(&self) -> usize {
            16 + self.name.serialize_length()
                + VarInt(self.properites.len() as i32).serialize_length()
                + self.properites.iter().map(|p| p.s_length()).sum::<usize>()
        }

        fn serialize(&self, mut buffer: &mut [u8]) -> usize {
            (buffer[..16]).copy_from_slice(&self.uuid.to_be_bytes());
            buffer = &mut buffer[16..];

            let written = self.name.serialize(buffer);
            buffer = &mut buffer[written..];

            let prop_len_varint = VarInt(self.properites.len() as i32);
            prop_len_varint.serialize(buffer);

            for prop in self.properites.iter() {
                let written = prop.serialize(buffer);
                buffer = &mut buffer[written..];
            }

            self.length()
        }
    }

    impl Property {
        pub fn s_length(&self) -> usize {
            self.name.serialize_length()
                + self.value.serialize_length()
                + 1
                + self
                    .signature
                    .as_ref()
                    .map(|s| s.serialize_length())
                    .unwrap_or(0) // + 1
        }

        pub fn serialize(&self, mut buffer: &mut [u8]) -> usize {
            let mut sum = 0;

            let written = self.name.serialize(buffer);
            buffer = &mut buffer[written..];
            sum += written;

            let written = self.value.serialize(buffer);
            buffer = &mut buffer[written..];
            sum += written;

            match self.signature.as_ref() {
                Some(sig) => {
                    buffer[0] = 0;
                    sum += 1;
                    buffer = &mut buffer[1..];

                    let written = sig.serialize(buffer);
                    buffer = &mut buffer[written..];
                    sum += written;

                    // buffer[0] = 1;
                    // sum+= 1;
                }
                None => {
                    buffer[0] = 0;
                    sum += 1;
                    buffer = &mut buffer[1..];

                    // buffer[0] = 1;
                    // sum += 1;
                }
            };

            sum
        }
    }

    impl LoginSuccess {
        pub fn parse(
            id: VarInt,
            i: &[u8],
        ) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
            if id.0 != 0x02 {
                return Err(nom::Err::Error(crate::general::ParseError::Other));
            }

            let (i, uuid) = nom::number::streaming::be_u128(i)?;
            dbg!(i);

            let (i, name) = PString::parse(i)?;
            dbg!(i);

            let (i, prop_count) = VarInt::parse(i)?;
            dbg!(i);

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
        fn serialize_parse() {
            let msg = LoginSuccess {
                uuid: 123456789,
                name: PString("testing".into()),
                properites: Vec::new(),
            };
            let packet = Packet { inner: msg };

            let serialized = packet.serialize();

            let (rem, parsed_packet) = Packet::parse(LoginSuccess::parse)(&serialized).unwrap();
            assert_eq!(&[] as &[u8], rem);

            assert_eq!(packet, parsed_packet);
        }

        #[test]
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
}

pub mod server {
    use crate::general::{PString, VarInt};

    #[derive(Debug, PartialEq)]
    pub struct LoginStart {
        pub name: PString<'static>,
        pub uuid: u128,
    }

    impl LoginStart {
        pub fn parse(
            id: VarInt,
            i: &[u8],
        ) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
            if id.0 != 0x00 {
                return Err(nom::Err::Error(crate::general::ParseError::Other));
            }

            let (i, name) = PString::parse(i)?;
            let (i, uuid) = nom::number::streaming::be_u128(i)?;

            Ok((i, Self { name, uuid }))
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct EncryptionResponse {
        pub shared_secret: Vec<u8>,
        pub verify_token: Vec<u8>,
    }

    impl EncryptionResponse {
        pub fn parse(
            id: VarInt,
            i: &[u8],
        ) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
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
        pub fn parse(
            id: VarInt,
            i: &[u8],
        ) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
            Ok((i, Self {}))
        }
    }
}

/// https://gist.github.com/Lazersmoke/9947ada8acdd74a8b2e37d77cf1e0fdc
pub fn asn1_encode_key<M, E>(modulo: M, exponent: E) -> Vec<u8>
where
    M: core::ops::Deref<Target = openssl::bn::BigNumRef>,
    E: core::ops::Deref<Target = openssl::bn::BigNumRef>,
{
    let modulo: &openssl::bn::BigNumRef = modulo.deref();
    let exponent: &openssl::bn::BigNumRef = exponent.deref();

    let asnOIDForRSAKeys = [0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01, 0x01];

    let asnSequence = [0x30];
    let asnObjectId = [0x06];
    let asnTag = [0x05];
    let asnBitString = [0x03];
    let asnInt = [0x02];
    let nullForGoodLuck = [0x00];

    let algParams = join_vec(asnTag.to_vec(), nullForGoodLuck.to_vec());
    let algObjectId = join_vec(asnObjectId.to_vec(), with_length(asnOIDForRSAKeys.to_vec()));
    let algIdentifier = join_vec(
        asnSequence.to_vec(),
        with_length(join_vec(algObjectId.clone(), algParams.clone())),
    );

    let bytesOfModulus = int_bytes_raw(modulo);
    let bytesOfExponent = int_bytes_raw(exponent);

    let theModulus = join_vec(
        asnInt.to_vec(),
        with_length(join_vec(nullForGoodLuck.to_vec(), bytesOfModulus.clone())),
    );

    let theExponent = join_vec(asnInt.to_vec(), with_length(bytesOfExponent.clone()));

    let pubKeySequence = join_vec(
        asnSequence.to_vec(),
        with_length(join_vec(theModulus.clone(), theExponent.clone())),
    );

    let pubKeyBitString = join_vec(
        asnBitString.to_vec(),
        with_length(join_vec(nullForGoodLuck.to_vec(), pubKeySequence.clone())),
    );

    fn int_bytes_raw(v: &openssl::bn::BigNumRef) -> Vec<u8> {
        return v.to_vec();

        let mut result = Vec::new();
        let mut tmp = openssl::bn::BigNum::new().unwrap();
        let mut idx = 0;
        while tmp > openssl::bn::BigNum::new().unwrap() {
            // result.insert(0, (v & 0xff) as u8);
            tmp.rshift(&v, 8 * idx).unwrap();
            idx += 1;
        }

        result
    }

    fn join_vec(mut f: Vec<u8>, mut s: Vec<u8>) -> Vec<u8> {
        f.append(&mut s);
        f
    }

    fn len_of(v: &[u8]) -> Vec<u8> {
        if v.len() < 128 {
            return vec![v.len() as u8];
        } else {
            let len = v.len();

            let following_bytes = len.to_be_bytes();
            let first_byte =
                0b10000000 | (following_bytes.iter().skip_while(|v| **v == 0).count() as u8);

            core::iter::once(first_byte)
                .chain(following_bytes.iter().skip_while(|v| **v == 0).copied())
                .collect()
        }
    }
    fn with_length(mut v: Vec<u8>) -> Vec<u8> {
        let mut length = len_of(&v);
        length.append(&mut v);
        length
    }

    join_vec(
        asnSequence.to_vec(),
        with_length(join_vec(algIdentifier, pubKeyBitString)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode() {
        let mut modulo = openssl::bn::BigNum::new().unwrap();
        let mut exponent = openssl::bn::BigNum::new().unwrap();

        modulo.add_word(0xff);
        exponent.add_word(0x010001);

        let result = asn1_encode_key(modulo, exponent);

        assert_eq!(
            &[
                0x30, 0x1d, 0x30, 0x0d, 0x06, 0x09, 0x2a, 0x86, 0x48, 0x86, 0xf7, 0x0d, 0x01, 0x01,
                0x01, 0x05, 0x00, 0x03, 0x0c, 0x00, 0x30, 0x09, 0x02, 0x02, 0x00, 0xff, 0x02, 0x03,
                0x01, 0x00, 0x01
            ],
            result.as_slice()
        );
    }
}
