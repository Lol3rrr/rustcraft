use crate::{
    general::{PString, VarInt},
    serialize::SerializeItem,
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

#[derive(Debug, PartialEq)]
pub struct RegistryData {
    pub id: PString<'static>,
    pub entries: Vec<RegistryEntry>,
}

#[derive(Debug, PartialEq)]
pub struct RegistryEntry {
    pub id: PString<'static>,
    pub data: Option<nbt::Tag>,
}

impl RegistryEntry {
    pub fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
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

impl RegistryData {
    pub fn parse(id: VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        if id.0 != 0x07 {
            return Err(nom::Err::Error(crate::general::ParseError::WrongPacketId {
                expected: 0x07,
                received: id.0,
            }));
        }

        let (i, registry_id) = PString::parse(i)?;
        let (i, entry_count) = VarInt::parse(i)?;

        let mut entries = Vec::with_capacity(entry_count.0 as usize);
        let mut i = i;
        for _ in 0..entry_count.0 {
            let (n_i, entry) = RegistryEntry::parse(i)?;
            i = n_i;
            entries.push(entry);
        }

        Ok((
            &[],
            Self {
                id: registry_id,
                entries,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct Finish {}

impl Finish {
    pub fn parse(id: VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        if id.0 != 0x03 {
            return Err(nom::Err::Error(crate::general::ParseError::Other));
        }

        Ok((i, Self {}))
    }
}

impl crate::packet::PacketContent for Finish {
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
