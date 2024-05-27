use crate::{
    general::{PString, Position, VarInt},
    serialize::SerializeItem,
};

/// [Docs](https://wiki.vg/Pre-release_protocol#Entity_Metadata)
#[derive(Debug, PartialEq)]
pub struct EntityMetadata {
    pub metadata: Vec<EntityMetadataEntry>,
}

#[derive(Debug, PartialEq)]
pub struct EntityMetadataEntry {
    pub index: u8,
    pub ty: i32,
}

impl crate::serialize::SerializeItem for EntityMetadata {
    fn slen(&self) -> usize {
        todo!()
    }

    fn serialize<'b>(
        &self,
        buf: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        todo!()
    }

    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        let mut parts = Vec::new();

        let mut i = i;
        loop {
            let (n_i, index) = u8::parse(i)?;
            if index == 0xff {
                i = n_i;
                break;
            }

            let (n_i, raw_type) = VarInt::parse(n_i)?;

            let (n_i, value) = match raw_type.0 {
                0 => {
                    let (i, v) = i8::parse(n_i)?;
                    dbg!(v);
                    (i, ())
                }
                1 => {
                    let (i, v) = VarInt::parse(n_i)?;
                    dbg!(v);
                    (i, ())
                }
                2 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing VarLong EntityMetadata",
                    )))
                }
                3 => {
                    let (i, v) = f32::parse(n_i)?;
                    dbg!(v);
                    (i, ())
                }
                4 => {
                    let (i, v) = PString::<'static>::parse(n_i)?;
                    dbg!(v);
                    (i, ())
                }
                5 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Text Component EntityMetadata",
                    )))
                }
                6 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Optional Text Component EntityMetadata",
                    )))
                }
                7 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Slot EntityMetadata",
                    )))
                }
                8 => {
                    let (i, v) = bool::parse(n_i)?;
                    dbg!(v);
                    (i, ())
                }
                9 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Rotations EntityMetadata",
                    )))
                }
                10 => {
                    let (i, v) = Position::parse(n_i)?;
                    dbg!(v);
                    (i, ())
                }
                11 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Optional Position EntityMetadata",
                    )))
                }
                12 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Direction EntityMetadata",
                    )))
                }
                13 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Optional UUID EntityMetadata",
                    )))
                }
                14 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Block State EntityMetadata",
                    )))
                }
                15 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Optional Block State EntityMetadata",
                    )))
                }
                16 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing NBT EntityMetadata",
                    )))
                }
                17 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Particle EntityMetadata",
                    )))
                }
                18 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Particles EntityMetadata",
                    )))
                }
                19 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Villager Data EntityMetadata",
                    )))
                }
                20 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Optional VarInt EntityMetadata",
                    )))
                }
                21 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Pose EntityMetadata",
                    )))
                }
                22 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Cat Variant EntityMetadata",
                    )))
                }
                23 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Wolf Variant EntityMetadata",
                    )))
                }
                24 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Frog Variant EntityMetadata",
                    )))
                }
                25 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Optional Global Positon EntityMetadata",
                    )))
                }
                26 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Painting Variant EntityMetadata",
                    )))
                }
                27 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Sniffer State EntityMetadata",
                    )))
                }
                28 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Armadillo State EntityMetadata",
                    )))
                }
                29 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Vector3 EntityMetadata",
                    )))
                }
                30 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Quaternion EntityMetadata",
                    )))
                }

                other => {
                    dbg!(index, raw_type, other);
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Metadata Type",
                    )));
                }
            };

            parts.push(EntityMetadataEntry {
                index,
                ty: raw_type.0,
            });

            i = n_i;
        }

        Ok((i, Self { metadata: parts }))
    }
}
