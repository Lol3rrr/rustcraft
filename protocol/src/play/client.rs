use crate::{
    combined_packet, declare_packet,
    general::{BitSet, PString, Position, VarInt, VarLong},
    serialize::SerializeItem,
};

combined_packet!(
    Play,
    BundleDelimiter,
    SpawnEntity,
    AckBlockChange,
    BlockUpdate,
    ChangeDifficulty,
    ChunkBatchFinished,
    ChunkBatchStart,
    Commands,
    SetContainerContent,
    SetContainerSlot,
    DamageEvent,
    EntityEvent,
    UnloadChunk,
    GameEvent,
    HurtAnimation,
    InitializeWorldBorder,
    KeepAlive,
    ChunkDataAndUpdateLight,
    WorldEvent,
    Particle,
    UpdateLight,
    UpdateEntityPosition,
    UpdateEntityPositionAndRotation,
    UpdateEntityRotation,
    PlayerAbilities,
    PlayerInfoUpdate,
    SynchronizePlayerPosition,
    UpdateRecipeBook,
    RemoveEntities,
    SetHeadRotation,
    UpdateSectionBlocks,
    ServerData,
    SetHeldItem,
    SetCenterChunk,
    SetDefaultSpawnPosition,
    SetEntityMetadata,
    SetEntityVelocity,
    SetEquipment,
    SetExperience,
    SetHealth,
    UpdateTime,
    SoundEffect,
    TeleportEntity,
    PickupItem,
    SetTickingState,
    StepTick,
    UpdateAdvancements,
    UpdateAttributes,
    UpdateRecipes
);

declare_packet!(BundleDelimiter, 0x00, false,);
declare_packet!(
    SpawnEntity,
    0x01,
    false,
    (entity_id, VarInt),
    (entity_uuid, u128),
    (ty, VarInt),
    (x, f64),
    (y, f64),
    (z, f64),
    (pitch, u8),
    (yaw, u8),
    (head_yaw, u8),
    (data, VarInt),
    (velocity_x, i16),
    (velocity_y, i16),
    (velocity_z, i16)
);
declare_packet!(AckBlockChange, 0x05, false, (sequence_id, VarInt));
declare_packet!(
    BlockUpdate,
    0x09,
    false,
    (location, Position),
    (block_id, VarInt)
);
declare_packet!(
    ChangeDifficulty,
    0x0b,
    false,
    (difficulty, u8),
    (locked, bool)
);
declare_packet!(ChunkBatchFinished, 0x0c, false, (size, VarInt));
declare_packet!(ChunkBatchStart, 0x0d, false,);
declare_packet!(Commands, 0x11, false,); // TODO
declare_packet!(SetContainerContent, 0x13, false,); // TODO
declare_packet!(
    SetContainerSlot,
    0x15,
    false,
    (window_id, i8),
    (state_id, VarInt),
    (slot, i16),
    (slot_data, crate::general::Slot)
);
declare_packet!(
    DamageEvent,
    0x1a,
    false,
    (entity_id, VarInt),
    (source_type_id, VarInt),
    (source_cause_id, VarInt),
    (source_direct_id, VarInt),
    (source_position, Option<(f64, f64, f64)>)
);
declare_packet!(
    EntityEvent,
    0x1f,
    false,
    (entity_id, i32),
    (entity_status, i8)
);
declare_packet!(UnloadChunk, 0x21, false, (chunk_z, i32), (chunk_x, i32));
declare_packet!(GameEvent, 0x22, false,); // TODO
declare_packet!(HurtAnimation, 0x24, false, (entity_id, VarInt), (yaw, f32));
declare_packet!(InitializeWorldBorder, 0x25, false,); // TODO
declare_packet!(KeepAlive, 0x26, false,); // TODO
declare_packet!(WorldEvent, 0x28, false,); // TODO
declare_packet!(Particle, 0x29, false,); // TODO
declare_packet!(
    UpdateEntityPosition,
    0x2e,
    false,
    (entity_id, VarInt),
    (delta_x, i16),
    (delta_y, i16),
    (delta_z, i16),
    (on_ground, bool)
);
declare_packet!(
    UpdateEntityPositionAndRotation,
    0x2f,
    false,
    (entity_id, VarInt),
    (delta_x, i16),
    (delta_y, i16),
    (delta_z, i16),
    (yaw, u8),
    (pitch, u8),
    (on_ground, bool)
);
declare_packet!(
    UpdateEntityRotation,
    0x30,
    false,
    (entity_id, VarInt),
    (yaw, u8),
    (pitch, u8),
    (on_ground, bool)
);
declare_packet!(
    PlayerAbilities,
    0x38,
    false,
    (flags, i8),
    (flying_speed, f32),
    (fov_modifier, f32)
);
declare_packet!(PlayerInfoUpdate, 0x3e, false,); // TODO
declare_packet!(SynchronizePlayerPosition, 0x40, false,); // TODO
declare_packet!(UpdateRecipeBook, 0x41, false,); // TODO
declare_packet!(RemoveEntities, 0x42, false, (entity_ids, Vec<VarInt>));
declare_packet!(
    SetHeadRotation,
    0x48,
    false,
    (entity_id, VarInt),
    (angle, u8)
);
declare_packet!(
    UpdateSectionBlocks,
    0x49,
    false,
    (chunk_section_position, i64),
    (blocks, Vec<VarLong>)
);
declare_packet!(ServerData, 0x4b, false,); // TODO
declare_packet!(SetHeldItem, 0x53, false, (slot, i8));
declare_packet!(SetCenterChunk, 0x54, false,); // TODO
declare_packet!(SetDefaultSpawnPosition, 0x56, false,); // TODO
declare_packet!(
    SetEntityMetadata,
    0x58,
    false,
    (entity_id, VarInt),
    (metadata, crate::metadata::entity::EntityMetadata)
);
declare_packet!(
    SetEntityVelocity,
    0x5a,
    false,
    (entity_id, VarInt),
    (velocity_x, i16),
    (velocity_y, i16),
    (velocity_z, i16)
);
declare_packet!(
    SetEquipment,
    0x5b,
    false,
    (entity_id, VarInt),
    (equipment, Equipment)
);
declare_packet!(SetExperience, 0x5c, false,); // TODO
declare_packet!(SetHealth, 0x5d, false,); // TODO

#[derive(Debug, PartialEq)]
pub struct Equipment {
    pub slots: Vec<(i8, crate::general::Slot)>,
}

impl crate::serialize::SerializeItem for Equipment {
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
        let mut slots = Vec::new();

        let mut i = i;
        loop {
            let (n_i, slot) = i8::parse(i)?;
            let (n_i, data) = crate::general::Slot::parse(n_i)?;

            slots.push((slot & (0x80_u8 as i8), data));

            i = n_i;

            if (slot as u8) & 0x80 == 0 {
                break;
            }
        }

        Ok((i, Self { slots }))
    }
}

#[derive(Debug, PartialEq)]
pub struct Login {
    pub entity_id: i32,
    pub is_hardcore: bool,
    pub dimensions: Vec<PString<'static>>,
    pub max_players: VarInt,
    pub view_distance: VarInt,
    pub simulation_distance: VarInt,
    pub reduced_debug_info: bool,
    pub enable_respawn_rule: bool,
    pub do_limited_crafting: bool,
    pub dimension_type: VarInt,
    pub dimension_name: PString<'static>,
    pub hashed_seed: i64,
    pub game_mode: u8,
    pub previous_game_mode: i8,
    pub is_debug: bool,
    pub is_flat: bool,
    pub death_location: Option<(PString<'static>, Position)>,
    pub portal_cooldown: VarInt,
}

impl crate::packet::PacketContent for Login {
    const ID: i32 = 0x2b;
    const PACKETTRAIL: bool = false;

    fn length(&self) -> usize {
        self.entity_id.slen()
            + self.is_hardcore.slen()
            + self.dimensions.slen()
            + self.max_players.slen()
            + self.view_distance.slen()
            + self.simulation_distance.slen()
            + self.reduced_debug_info.slen()
            + self.enable_respawn_rule.slen()
            + self.do_limited_crafting.slen()
            + self.dimension_type.slen()
            + self.dimension_name.slen()
            + self.hashed_seed.slen()
            + self.game_mode.slen()
            + self.previous_game_mode.slen()
            + self.is_debug.slen()
            + self.is_flat.slen()
            + self.death_location.slen()
            + self.portal_cooldown.slen()
    }

    fn serialize<'b>(
        &self,
        mut buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        buffer = self.entity_id.serialize(buffer)?;
        buffer = self.is_hardcore.serialize(buffer)?;
        buffer = self.dimensions.serialize(buffer)?;
        buffer = self.max_players.serialize(buffer)?;
        buffer = self.view_distance.serialize(buffer)?;
        buffer = self.simulation_distance.serialize(buffer)?;
        buffer = self.reduced_debug_info.serialize(buffer)?;
        buffer = self.enable_respawn_rule.serialize(buffer)?;
        buffer = self.do_limited_crafting.serialize(buffer)?;
        buffer = self.dimension_type.serialize(buffer)?;
        buffer = self.dimension_name.serialize(buffer)?;
        buffer = self.hashed_seed.serialize(buffer)?;
        buffer = self.game_mode.serialize(buffer)?;
        buffer = self.previous_game_mode.serialize(buffer)?;
        buffer = self.is_debug.serialize(buffer)?;
        buffer = self.is_flat.serialize(buffer)?;
        buffer = self.death_location.serialize(buffer)?;
        buffer = self.portal_cooldown.serialize(buffer)?;

        Ok(buffer)
    }
}

declare_packet!(
    UpdateTime,
    0x64,
    false,
    (world_age, i64),
    (time_of_day, i64)
);
declare_packet!(
    TeleportEntity,
    0x70,
    false,
    (entity_id, VarInt),
    (x, f64),
    (y, f64),
    (z, f64),
    (yaw, u8),
    (pitch, u8),
    (on_ground, bool)
);

#[derive(Debug, PartialEq)]
pub struct BlockEntity {
    pub x: u8,
    pub z: u8,
    pub y: i16,
    pub ty: VarInt,
    pub data: nbt::Tag,
}

impl SerializeItem for BlockEntity {
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
        let (i, packed_xz) = nom::number::streaming::be_u8(i)?;
        let (i, y) = nom::number::streaming::be_i16(i)?;
        let (i, ty) = VarInt::parse(i)?;

        let (i, (n_name, data)) = nbt::Tag::parse(false, true)(i)
            .map_err(|e| nom::Err::Error(crate::general::ParseError::Other))?;

        Ok((
            i,
            Self {
                x: packed_xz >> 4,
                z: packed_xz & 0x0f,
                y,
                ty,
                data,
            },
        ))
    }
}

#[derive(Debug, PartialEq)]
pub struct HeightMap(pub nbt::Tag);

impl SerializeItem for HeightMap {
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
        let (i, (_, value)) = nbt::Tag::parse(false, true)(i)
            .map_err(|e| nom::Err::Error(crate::general::ParseError::Other))?;
        Ok((i, Self(value)))
    }
}

declare_packet!(
    ChunkDataAndUpdateLight,
    0x27,
    false,
    (chunk_x, i32),
    (chunk_z, i32),
    (height_maps, HeightMap),
    (data, Vec<i8>),
    (block_entities, Vec<BlockEntity>),
    (sky_light_mask, BitSet),
    (block_light_mask, BitSet),
    (empty_sky_light_mask, BitSet),
    (sky_light_arrays, Vec<Vec<i8>>),
    (block_light_arrays, Vec<Vec<i8>>)
);

declare_packet!(
    UpdateLight,
    0x2a,
    false,
    (chunk_x, VarInt),
    (chunk_z, VarInt),
    (sky_light_mask, BitSet),
    (block_light_mask, BitSet),
    (empty_sky_light_mask, BitSet),
    (sky_light_arrays, Vec<Vec<i8>>),
    (block_light_arrays, Vec<Vec<i8>>)
);

#[derive(Debug, PartialEq)]
pub enum SoundID {
    Id(VarInt),
    NamedId {
        name: PString<'static>,
        fixed_range: Option<f32>,
    },
}

impl SerializeItem for SoundID {
    fn slen(&self) -> usize {
        match self {
            Self::Id(v) => VarInt(v.0 + 1).slen(),
            Self::NamedId { name, fixed_range } => {
                VarInt(0).slen() + name.slen() + fixed_range.slen()
            }
        }
    }

    fn serialize<'b>(
        &self,
        mut buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        match self {
            Self::Id(v) => VarInt(v.0 + 1).serialize(buffer),
            Self::NamedId { name, fixed_range } => {
                buffer = VarInt(0).serialize(buffer)?;
                buffer = name.serialize(buffer)?;
                buffer = fixed_range.serialize(buffer)?;
                Ok(buffer)
            }
        }
    }

    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        let (i, id) = VarInt::parse(i)?;

        if id.0 == 0 {
            let (i, name) = PString::parse(i)?;
            let (i, fixed_range) = Option::<f32>::parse(i)?;

            Ok((i, Self::NamedId { name, fixed_range }))
        } else {
            Ok((i, Self::Id(VarInt(id.0 - 1))))
        }
    }
}

declare_packet!(
    SoundEffect,
    0x68,
    false,
    (id, SoundID),
    (sound_category, VarInt),
    (effect_position_x, i32),
    (effect_position_y, i32),
    (effect_position_z, i32),
    (volume, f32),
    (pitch, f32),
    (seed, i64)
);

#[derive(Debug, PartialEq)]
pub struct AttributeModifier {
    pub id: u128,
    pub amount: f64,
    pub operation: i8,
}

impl SerializeItem for AttributeModifier {
    fn slen(&self) -> usize {
        self.id.slen() + self.amount.slen() + self.operation.slen()
    }

    fn serialize<'b>(
        &self,
        mut buffer: &'b mut [u8],
    ) -> Result<&'b mut [u8], crate::serialize::SerializeError> {
        buffer = self.id.serialize(buffer)?;
        buffer = self.amount.serialize(buffer)?;
        buffer = self.operation.serialize(buffer)?;
        Ok(buffer)
    }

    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        let (i, id) = u128::parse(i)?;
        let (i, amount) = f64::parse(i)?;
        let (i, operation) = i8::parse(i)?;

        Ok((
            i,
            Self {
                id,
                amount,
                operation,
            },
        ))
    }
}

declare_packet!(PickupItem, 0x6f, false,); // TODO
declare_packet!(SetTickingState, 0x71, false,); // TODO
declare_packet!(StepTick, 0x72, false,); // TODO
declare_packet!(UpdateAdvancements, 0x74, false,); // TODO
declare_packet!(
    UpdateAttributes,
    0x75,
    false,
    (entity_id, VarInt),
    (properties, Vec<(VarInt, f64, Vec<AttributeModifier>)>)
);
declare_packet!(UpdateRecipes, 0x77, false,); // TODO

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_packet_impled {
        ($test_name:ident, $id:literal) => {
            #[test]
            fn $test_name() {
                let result = Play::parse(VarInt($id), &[]);
                match result {
                    Err(nom::Err::Error(crate::general::ParseError::UnknownPacketId(_))) => {
                        panic!("Packet is not implemented");
                    }
                    _ => {}
                };
            }
        };
    }

    test_packet_impled!(id_0x13, 0x13);
    test_packet_impled!(id_0x15, 0x15);
    test_packet_impled!(id_0x22, 0x22);
    test_packet_impled!(id_0x25, 0x25);
    test_packet_impled!(id_0x26, 0x26);
    test_packet_impled!(id_0x28, 0x28);
    test_packet_impled!(id_0x29, 0x29);
    test_packet_impled!(id_0x3e, 0x3e);
    test_packet_impled!(id_0x54, 0x54);
    test_packet_impled!(id_0x56, 0x56);
    test_packet_impled!(id_0x58, 0x58);
    test_packet_impled!(id_0x5c, 0x5c);
    test_packet_impled!(id_0x5d, 0x5d);
    test_packet_impled!(id_0x6f, 0x6f);
    test_packet_impled!(id_0x71, 0x71);
    test_packet_impled!(id_0x72, 0x72);
}
