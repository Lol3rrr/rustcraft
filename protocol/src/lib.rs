//! # Protocol
//! Entirely based on [these docs](https://wiki.vg/index.php)

pub mod configuration;
pub mod general;
pub mod handshake;
pub mod login;
pub mod metadata;
pub mod packet;
pub mod play;
pub mod status;

pub mod serialize;

macro_rules! declare_packet {
    ($name:ident, $pid:literal, $ptrail:literal, $(($field:ident, $field_ty:ty)),*) => {
        #[derive(Debug, PartialEq)]
        pub struct $name {
            $(
                pub $field: $field_ty,
            )*
        }

        impl crate::packet::PacketContent for $name {
            const ID: i32 = $pid;
            const PACKETTRAIL: bool = $ptrail;

            fn length(&self) -> usize {
                0
                $(+ crate::serialize::SerializeItem::slen(&self.$field) )*
            }

            fn serialize<'b>(&self, buffer: &'b mut [u8]) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
                $(
                    let buffer = crate::serialize::SerializeItem::serialize(&self.$field, buffer)?;
                )*

                Ok(buffer)
            }
        }

        impl $name {
            pub fn parse(id: crate::general::VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
                if id.0 != $pid {
                    return Err(nom::Err::Error(crate::general::ParseError::WrongPacketId {
                        expected: $pid,
                        received: id.0,
        }));
                }

                $(
                    let (i, $field) = <$field_ty as crate::serialize::SerializeItem>::parse(i)?;
                )*

                let (i, _) = nom::combinator::cond($ptrail, nom::bytes::streaming::tag(&[0x01]))(i)?;

                Ok((i, Self {
                    $(
                        $field,
                    )*
                }))
            }
        }
    }
}
pub(crate) use declare_packet;

macro_rules! combined_packet {
    ($name:ident, $($parts:ident),*) => {
        #[derive(Debug, PartialEq)]
        pub enum $name {
            $(
                $parts($parts),
            )*
        }

        impl $name {
            pub fn parse(id: crate::general::VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
                use crate::packet::PacketContent;
                match id.0 {
                    $(
                    $parts::ID => $parts::parse(id, i).map(|(i, v)| (i, Self::$parts(v))),
                    )*
                    other => {
                        Err(nom::Err::Error(crate::general::ParseError::UnknownPacketId(other)))
                    }
                }
            }
        }
    }
}
pub(crate) use combined_packet;
