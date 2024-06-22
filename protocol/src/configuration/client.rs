use crate::{
    combined_packet, declare_packet,
    general::{PString, VarInt},
};

combined_packet!(
    Configuration,
    PluginMessage,
    RegistryData,
    FeatureFlags,
    Finish,
    UpdateTags,
    KnownPacks
);

// declare_packet!(PluginMessage, 0x01, false, (channel, PString<'static>), ); // TODO
declare_packet!(Finish, 0x03, false,);
declare_packet!(FeatureFlags, 0x0c, false, (flags, Vec<PString<'static>>));
declare_packet!(
    KnownPacks,
    0x0e,
    false,
    (
        packs,
        Vec<(PString<'static>, PString<'static>, PString<'static>)>
    )
);

#[derive(Debug, PartialEq)]
pub struct PluginMessage {
    pub channel: PString<'static>,
    pub data: Vec<u8>,
}

impl PluginMessage {
    pub fn parse(id: VarInt, buf: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        use crate::serialize::SerializeItem;

        if id.0 != 0x01 {
            return Err(nom::Err::Error(crate::general::ParseError::WrongPacketId {
                expected: 0x01,
                received: id.0,
            }));
        }

        let (buf, channel) = PString::parse(buf)?;

        Ok((
            &[],
            Self {
                channel,
                data: buf.to_vec(),
            },
        ))
    }
}

impl crate::packet::PacketContent for PluginMessage {
    const ID: i32 = 0x01;
    const PACKETTRAIL: bool = false;

    fn length(&self) -> usize {
        use crate::serialize::SerializeItem;

        self.channel.slen() + self.data.len()
    }

    fn serialize<'b>(
        &self,
        buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        use crate::serialize::SerializeItem;

        let buffer = self.channel.serialize(buffer)?;

        let content_len = self.data.len();
        (buffer[..content_len]).copy_from_slice(&self.data);

        Ok(&mut buffer[content_len..])
    }
}

#[derive(Debug, PartialEq)]
pub struct RegistryEntry {
    pub id: PString<'static>,
    pub data: Option<nbt::Tag>,
}

declare_packet!(
    RegistryData,
    0x07,
    false,
    (id, PString<'static>),
    (entries, Vec<RegistryEntry>)
);

impl crate::serialize::SerializeItem for RegistryEntry {
    fn slen(&self) -> usize {
        self.id.slen() + self.data.as_ref().map(|t| todo!()).unwrap_or(1)
    }

    fn serialize<'b>(
        &self,
        buf: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        let buf = self.id.serialize(buf)?;
        match self.data.as_ref() {
            None => 0x00u8.serialize(buf),
            Some(data) => {
                todo!()
            }
        }
    }

    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        let (i, id) = PString::parse(i)?;

        let (i, has_data) = nom::number::streaming::be_u8(i)?;
        let data = match has_data {
            0x00 => None,
            0x01 => {
                return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                    "Parsing Registry Data",
                )));
            }
            other => {
                dbg!(other);
                return Err(nom::Err::Error(crate::general::ParseError::Other));
            }
        };

        Ok((i, Self { id, data }))
    }
}

declare_packet!(
    UpdateTags,
    0x0d,
    false,
    (
        tags,
        Vec<(PString<'static>, Vec<(PString<'static>, Vec<VarInt>)>)>
    )
);
/*
#[derive(Debug, PartialEq)]
pub struct UpdateTags {
    pub tags: Vec<PString<'static>>,
}

#[derive(Debug, PartialEq)]
pub struct TagArray {
    pub tags: Vec<()>
}

impl UpdateTags {
    pub fn parse(id: VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        if id.0 != 0x0d {
            return Err(nom::Err::Error(crate::general::ParseError::Other));
        }

        let (mut i, count) = VarInt::parse(i)?;
        if count.0 < 0 {
            return Err(nom::Err::Error(crate::general::ParseError::NegativeLength));
        }

        let mut tags = Vec::with_capacity(count.0 as usize);
        for _ in 0..count.0 {
            let (n_i, name) = PString::parse(i)?;
            dbg!(&name);

            let (n_i, tag_array) = TagArray::parse(n_i)?;
            dbg!(&tag_array);

            i = n_i;

            tags.push(name);
        }

        Ok((i, Self { tags }))
    }
}

impl TagArray {
    pub fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        let (i, length) = VarInt::parse(i)?;
        if length.0 < 0 {
            return Err(nom::Err::Error(crate::general::ParseError::NegativeLength));
        }

        // TODO
        let mut tags = Vec::with_capacity(length.0 as usize);
        for _ in 0..length.0 {
            todo!()
        }

        Ok((i, Self { tags }))
    }
}

impl crate::packet::PacketContent for UpdateTags {
    const ID: i32 = 0x0d;
    const PACKETTRAIL: bool = true;

    fn length(&self) -> usize {
        self.tags.slen()
    }

    fn serialize<'b>(
        &self,
        buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        self.tags.serialize(buffer)
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::{Packet, PacketContent};

    #[test]
    fn registry_data() {
        let expected = RegistryData {
            id: PString("testing".into()),
            entries: vec![RegistryEntry {
                id: PString("first".into()),
                data: None,
            }],
        };

        let buffer = Packet { inner: expected }.serialize();

        dbg!(&buffer);

        let (rem, parsed) = Packet::parse(RegistryData::parse)(&buffer).unwrap();
        dbg!(parsed);

        // todo!()
    }
}
