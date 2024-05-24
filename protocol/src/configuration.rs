//! [Docs](https://wiki.vg/Protocol#Configuration)

pub mod client {
    #[derive(Debug, PartialEq)]
    pub struct RegistryData {
        pub data: (),
    }

    #[derive(Debug, PartialEq)]
    pub struct Finish {}

    impl crate::packet::PacketContentSerializer for Finish {
        const ID: i32 = 0x03;
        const PACKETTRAIL: bool = false;

        fn length(&self) -> usize {
            0
        }

        fn serialize<'b>(
            &self,
            buf: &'b mut [u8],
        ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
            Ok(buf)
        }
    }
}

pub mod server {
    use crate::general::{PString, VarInt};

    #[derive(Debug, PartialEq)]
    pub enum ConfigurationMessage {
        ClientInformation(ClientInformation),
        PluginMessage(PluginMessage),
        AckFinish(AckFinish),
    }

    impl ConfigurationMessage {
        pub fn parse(
            id: VarInt,
            i: &[u8],
        ) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
            match id.0 {
                0x00 => {
                    ClientInformation::parse(id, i).map(|(i, v)| (i, Self::ClientInformation(v)))
                }
                0x02 => PluginMessage::parse(id, i).map(|(i, v)| (i, Self::PluginMessage(v))),
                0x03 => AckFinish::parse(id, i).map(|(i, v)| (i, Self::AckFinish(v))),
                other => {
                    dbg!(other);
                    Err(nom::Err::Error(crate::general::ParseError::Other))
                }
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct ClientInformation {
        pub locale: PString<'static>,
        pub view_distance: i8,
        pub chat_mode: VarInt,
        pub chat_colors: bool,
        pub displayed_skin_parts: u8,
        pub main_hand: VarInt,
        pub enable_text_filtering: bool,
        pub allow_server_listings: bool,
    }

    impl ClientInformation {
        pub fn parse(
            id: VarInt,
            i: &[u8],
        ) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
            if id.0 != 0x00 {
                return Err(nom::Err::Error(crate::general::ParseError::Other));
            }

            let (i, locale) = PString::parse(i)?;
            let (i, view_distance) = nom::number::streaming::i8(i)?;
            let (i, chat_mode) = VarInt::parse(i)?;
            let (i, chat_colors) = nom::number::streaming::u8(i)?;
            let (i, displayed_skin_parts) = nom::number::streaming::u8(i)?;
            let (i, main_hand) = VarInt::parse(i)?;
            let (i, enable_text_filtering) = nom::number::streaming::u8(i)?;
            let (i, allow_server_listings) = nom::number::streaming::u8(i)?;

            Ok((
                i,
                Self {
                    locale,
                    view_distance,
                    chat_mode,
                    chat_colors: chat_colors == 0x01,
                    displayed_skin_parts,
                    main_hand,
                    enable_text_filtering: enable_text_filtering == 0x01,
                    allow_server_listings: allow_server_listings == 0x01,
                },
            ))
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct PluginMessage {
        pub channel: PString<'static>,
        pub data: Vec<u8>,
    }

    impl PluginMessage {
        pub fn parse(
            id: VarInt,
            i: &[u8],
        ) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
            if id.0 != 0x02 {
                return Err(nom::Err::Error(crate::general::ParseError::Other));
            }

            let (i, channel) = PString::parse(i)?;
            let (i, length) = VarInt::parse(i)?;

            let data = i[..length.0 as usize].to_vec();
            let i = &i[length.0 as usize..];

            Ok((i, Self { channel, data }))
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct AckFinish {}

    impl AckFinish {
        pub fn parse(
            id: VarInt,
            i: &[u8],
        ) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
            if id.0 != 0x03 {
                return Err(nom::Err::Error(crate::general::ParseError::Other));
            }

            Ok((i, Self {}))
        }
    }
}