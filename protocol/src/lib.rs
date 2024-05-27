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
                use crate::serialize::SerializeItem;

                0
                $(+ self.$field.slen() )*
            }

            fn serialize<'b>(&self, mut buffer: &'b mut [u8]) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
                use crate::serialize::SerializeItem;
                $(
                    buffer = self.$field.serialize(buffer)?;
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

                use crate::serialize::SerializeItem;
                $(
                    let (i, $field) = <$field_ty>::parse(i)?;
                )*

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
