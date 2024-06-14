use crate::{
    combined_packet, declare_packet,
    general::{PString, VarInt},
    serialize::SerializeItem,
};

combined_packet!(
    Configuration,
    PluginMessage,
    RegistryData,
    FeatureFlags,
    Finish,
    UpdateTags,
    KnownPackets
);

declare_packet!(PluginMessage, 0x01, false, (channel, PString<'static>)); // TODO
declare_packet!(Finish, 0x03, false,);
declare_packet!(FeatureFlags, 0x0c, false, (flags, Vec<PString<'static>>));
declare_packet!(KnownPackets, 0x0e, false,); // TODO

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
            None => 0x00.serialize(buf),
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

#[derive(Debug, PartialEq)]
pub struct UpdateTags {
    pub tags: Vec<PString<'static>>,
}

impl UpdateTags {
    pub fn parse(id: VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        if id.0 != 0x0d {
            return Err(nom::Err::Error(crate::general::ParseError::Other));
        }

        // TODO

        Ok((&[], Self { tags: Vec::new() }))
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

        todo!()
    }
}
