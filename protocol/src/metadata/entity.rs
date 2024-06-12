use crate::general::{PString, Position, Slot, VarInt, VarLong};

/// [Docs](https://wiki.vg/Pre-release_protocol#Entity_Metadata)
#[derive(Debug, PartialEq)]
pub struct EntityMetadata {
    pub metadata: Vec<EntityMetadataEntry>,
}

#[derive(Debug, PartialEq)]
pub struct EntityMetadataEntry {
    pub index: u8,
    pub ty: i32,
    pub value: EntityMetadataValue,
}

#[derive(Debug, PartialEq)]
pub enum EntityMetadataValue {
    Byte(i8),
    VarInt(VarInt),
    VarLong(VarLong),
    Float(f32),
    String(PString<'static>),
    TextComponent,
    OptionalTextComponent,
    Slot(Slot),
    Boolean(bool),
    Rotations,
    Position(Position),
    OptionalPosition(Option<Position>),
    Direction,
    OptionalUUID(Option<u128>),
    BlockState(VarInt),
    OptionalBlockState(Option<VarInt>),
    NBT(nbt::Tag),
    Particle,
    Particles,
    VillagerData,
    OptionalVarInt(Option<VarInt>),
    Pose(VarInt),
    CatVariant(VarInt),
    WolfVariant(VarInt),
    FrogVariant(VarInt),
    OptionalGlobalPosition,
    PaintingVariant(VarInt),
    SnifferState(VarInt),
    ArmadilloState(VarInt),
    Vector3(f32, f32, f32),
    Quaternion(f32, f32, f32, f32),
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
                    (i, EntityMetadataValue::Byte(v))
                }
                1 => {
                    let (i, v) = VarInt::parse(n_i)?;
                    (i, EntityMetadataValue::VarInt(v))
                }
                2 => {
                    let (i, v) = VarLong::parse(n_i)?;
                    (i, EntityMetadataValue::VarLong(v))
                }
                3 => {
                    let (i, v) = f32::parse(n_i)?;
                    (i, EntityMetadataValue::Float(v))
                }
                4 => {
                    let (i, v) = PString::<'static>::parse(n_i)?;
                    (i, EntityMetadataValue::String(v))
                }
                5 => {
                    let (i, v) = nbt::Tag::parse(false, true)(n_i).map_err(|e| nom::Err::Error(crate::general::ParseError::Other))?;
                    dbg!(v);

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
                    let (i, v) = crate::general::Slot::parse(n_i)?;
                    (i, EntityMetadataValue::Slot(v))
                }
                8 => {
                    let (i, v) = bool::parse(n_i)?;
                    (i, EntityMetadataValue::Boolean(v))
                }
                9 => {
                    return Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                        "Parsing Rotations EntityMetadata",
                    )))
                }
                10 => {
                    let (i, v) = Position::parse(n_i)?;
                    (i, EntityMetadataValue::Position(v))
                }
                11 => {
                    let (i, v) = Option::<Position>::parse(n_i)?;
                    (i, EntityMetadataValue::OptionalPosition(v))
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
                    let (i, pose_id) = VarInt::parse(n_i)?;
                    (i, EntityMetadataValue::Pose(pose_id))
                }
                22 => {
                    let (i, cat_id) = VarInt::parse(n_i)?;
                    (i, EntityMetadataValue::CatVariant(cat_id))
                }
                23 => {
                    let (i, wolf_id) = VarInt::parse(n_i)?;
                    (i, EntityMetadataValue::WolfVariant(wolf_id))
                }
                24 => {
                    let (i, frog_id) = VarInt::parse(n_i)?;
                    (i, EntityMetadataValue::FrogVariant(frog_id))
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
                value,
            });

            i = n_i;
        }

        Ok((i, Self { metadata: parts }))
    }
}
