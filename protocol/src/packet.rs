use crate::{
    general::VarInt,
    serialize::{SerializeError, SerializeItem},
};

#[derive(Debug, PartialEq)]
pub struct Packet<D> {
    pub inner: D,
}

pub trait PacketContentParser<D>:
    FnMut(VarInt, &[u8]) -> nom::IResult<&[u8], D, crate::general::ParseError>
{
}

impl<T, D> PacketContentParser<D> for T where
    T: FnMut(VarInt, &[u8]) -> nom::IResult<&[u8], D, crate::general::ParseError>
{
}

pub trait PacketContent {
    const PACKETTRAIL: bool;
    const ID: i32;

    fn length(&self) -> usize;
    fn serialize<'b>(&self, buffer: &'b mut [u8]) -> Result<&'b mut [u8], SerializeError>;
}

pub enum LegacyPacket<D> {
    Ping,
    Actual(D),
}

impl<D> Packet<D> {
    pub fn parse<'i, F>(
        mut parser: F,
    ) -> impl FnMut(&'i [u8]) -> nom::IResult<&'i [u8], Self, crate::general::ParseError>
    where
        F: PacketContentParser<D>,
    {
        move |i| {
            let (i, size) = VarInt::parse(i)?;
            if size.0 < 0 {
                return Err(nom::Err::Error(crate::general::ParseError::NegativeLength));
            }

            let len = size.0 as usize;
            if i.len() < len {
                return Err(nom::Err::Incomplete(nom::Needed::new(len - i.len())));
            }

            let inner_i = &i[..len];
            let after_i = &i[len..];

            let (inner_i, packet_id) = VarInt::parse(inner_i)?;
            let (remaining, inner) = parser(packet_id, inner_i)?;
            if !remaining.is_empty() {
                dbg!(remaining);
                return Err(nom::Err::Error(
                    crate::general::ParseError::RemainingDataAfterParsing { packet_id },
                ));
            }

            Ok((after_i, Self { inner }))
        }
    }

    pub fn parse_bytes<F>(
        parser: F,
        bytes: &mut bytes::BytesMut,
    ) -> nom::IResult<(), Self, crate::general::ParseError>
    where
        F: PacketContentParser<D>,
    {
        let result = match Self::parse(parser)(&bytes) {
            Ok((rem, v)) => {
                let to_advance = bytes.len() - rem.len();

                Ok((to_advance, v))
            }
            Err(e) => Err(e),
        };

        match result {
            Ok((to_advance, v)) => {
                let _ = bytes.split_to(to_advance);
                Ok(((), v))
            }
            Err(e) => Err(e),
        }
    }

    pub fn serialize(&self) -> Vec<u8>
    where
        D: PacketContent,
    {
        let pid = VarInt(D::ID);
        let inner_length = self.inner.length();
        let length_varint =
            VarInt(inner_length as i32 + 5 + D::PACKETTRAIL.then_some(1).unwrap_or(0));

        let mut result = vec![
            0;
            pid.slen() + length_varint.slen() - 2
                + inner_length
                + D::PACKETTRAIL.then_some(1).unwrap_or(0)
        ];

        let mut buffer = &mut result[..];

        let mut tmp_buffer = [0, 0, 0, 0, 0];
        let written = length_varint.serialize(&mut tmp_buffer);
        (buffer[..3]).copy_from_slice(&tmp_buffer[..3]);
        buffer[2] &= 0x7f;
        buffer = &mut buffer[3..];

        buffer = pid.serialize(buffer).unwrap();

        let serialized = self.inner.serialize(buffer);
        if D::PACKETTRAIL {
            *buffer.last_mut().unwrap() = 0x01;
        }

        result
    }
}

pub struct RawPacket {
    pub id: VarInt,
    pub data: Vec<u8>,
}

impl RawPacket {
    pub fn parse<'i>(
    ) -> impl FnMut(&'i [u8]) -> nom::IResult<&'i [u8], Self, crate::general::ParseError> {
        move |i| {
            let (i, size) = VarInt::parse(i)?;
            if size.0 < 0 {
                return Err(nom::Err::Error(crate::general::ParseError::NegativeLength));
            }

            let len = size.0 as usize;
            if i.len() < len {
                return Err(nom::Err::Incomplete(nom::Needed::new(len - i.len())));
            }

            let inner_i = &i[..len];
            let after_i = &i[len..];

            let (inner_i, packet_id) = VarInt::parse(inner_i)?;
            let content = inner_i.to_vec();

            Ok((
                after_i,
                Self {
                    id: packet_id,
                    data: content,
                },
            ))
        }
    }

    pub fn parse_bytes(
        bytes: &mut bytes::BytesMut,
    ) -> nom::IResult<(), Self, crate::general::ParseError> {
        let result = match Self::parse()(&bytes) {
            Ok((rem, v)) => {
                let to_advance = bytes.len() - rem.len();

                Ok((to_advance, v))
            }
            Err(e) => Err(e),
        };

        match result {
            Ok((to_advance, v)) => {
                let _ = bytes.split_to(to_advance);
                Ok(((), v))
            }
            Err(e) => Err(e),
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let pid = self.id;
        let inner_length = self.data.len();
        let length_varint = VarInt(inner_length as i32 + 5);

        let mut result = vec![0; pid.slen() + length_varint.slen() - 2 + inner_length];

        let mut buffer = &mut result[..];

        let mut tmp_buffer = [0, 0, 0, 0, 0];
        let written = length_varint.serialize(&mut tmp_buffer);
        (buffer[..3]).copy_from_slice(&tmp_buffer[..3]);
        buffer[2] &= 0x7f;
        buffer = &mut buffer[3..];

        buffer = pid.serialize(buffer).unwrap();
        (buffer[..self.data.len()]).copy_from_slice(&self.data);

        result
    }
}

impl<D> LegacyPacket<D> {
    pub fn parse<'i, F>(
        mut parser: F,
    ) -> impl FnMut(&'i [u8]) -> nom::IResult<&'i [u8], Self, crate::general::ParseError>
    where
        F: PacketContentParser<D>,
    {
        move |i| {
            if i.is_empty() {
                return Err(nom::Err::Incomplete(nom::Needed::new(1)));
            }
            if i[0] == 0xfe {
                dbg!("Parse Legacy Ping");
                return Err(nom::Err::Error(crate::general::ParseError::Other));
            }

            let (i, size) = VarInt::parse(i)?;
            if size.0 < 0 {
                return Err(nom::Err::Error(crate::general::ParseError::Other));
            }

            let len = size.0 as usize;
            if i.len() < len {
                return Err(nom::Err::Incomplete(nom::Needed::new(len - i.len())));
            }

            let inner_i = &i[..len];
            let after_i = &i[len..];

            let (inner_i, packet_id) = VarInt::parse(inner_i)?;
            let (_, inner) = parser(packet_id, inner_i)?;

            Ok((after_i, Self::Actual(inner)))
        }
    }

    pub fn parse_bytes<F>(
        parser: F,
        bytes: &mut bytes::BytesMut,
    ) -> nom::IResult<(), Self, crate::general::ParseError>
    where
        F: PacketContentParser<D>,
    {
        let result = match Self::parse(parser)(&bytes) {
            Ok((rem, v)) => {
                let to_advance = bytes.len() - rem.len();

                Ok((to_advance, v))
            }
            Err(e) => Err(e),
        };

        match result {
            Ok((to_advance, v)) => {
                let _ = bytes.split_to(to_advance);
                Ok(((), v))
            }
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packet() {
        let (rem, pack) = Packet::parse(|id, i| {
            assert_eq!(VarInt(0x00), id);
            assert_eq!(8, i.len());
            Ok((&[], ()))
        })(&[
            0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff,
        ])
        .unwrap();
        assert_eq!(&[0xff], rem);
        assert_eq!(Packet { inner: () }, pack);
    }
}
