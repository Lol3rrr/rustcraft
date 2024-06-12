use crate::{
    general::{PString, VarInt},
    serialize::SerializeItem,
    declare_packet,
};

#[derive(Debug)]
pub enum Configuration {
    RegistryData(RegistryData),
    Finish(Finish),
    UpdateTags(UpdateTags),
}

impl Configuration {
    pub fn parse(id: VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        match id.0 {
            0x03 => Finish::parse(id, i).map(|(i, v)| (i, Self::Finish(v))),
            0x07 => RegistryData::parse(id, i).map(|(i, v)| (i, Self::RegistryData(v))),
            0x0d => UpdateTags::parse(id, i).map(|(i, v)| (i, Self::UpdateTags(v))),
            _ => Err(nom::Err::Error(crate::general::ParseError::Other)),
        }
    }
}

declare_packet!(Finish, 0x03, false,);

#[derive(Debug, PartialEq)]
pub struct RegistryEntry {
    pub id: PString<'static>,
    pub data: Option<nbt::Tag>,
}

declare_packet!(RegistryData, 0x07, false, (id, PString<'static>), (entries, Vec<RegistryEntry>));

impl crate::serialize::SerializeItem for RegistryEntry {
    fn slen(&self) -> usize {
        todo!()
    }

    fn serialize<'b>(&self, buf: &'b mut [u8]) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        todo!()
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
