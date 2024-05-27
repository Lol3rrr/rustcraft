use crate::{
    declare_packet,
    general::{PString, Position, VarInt},
    serialize::SerializeItem,
};

#[derive(Debug, PartialEq)]
pub enum Play {
    BundleDelimiter(BundleDelimiter),
    SpawnEntity(SpawnEntity),
    BlockUpdate(BlockUpdate),
    ChunkBatchFinished(ChunkBatchFinished),
    ChunkBatchStart(ChunkBatchStart),
    EntityEvent(EntityEvent),
    UnloadChunk(UnloadChunk),
    Login(Login),
    UpdateEntityPosition(UpdateEntityPosition),
    UpdateEntityPositionAndRotation(UpdateEntityPositionAndRotation),
    UpdateEntityRotation(UpdateEntityRotation),
    RemoveEntities(RemoveEntities),
    SetHeadRotation(SetHeadRotation),
    SetEntityMetadata(SetEntityMetadata),
    SetEntityVelocity(SetEntityVelocity),
    UpdateTime(UpdateTime),
    TeleportEntity(TeleportEntity),
}

impl Play {
    pub fn parse(id: VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        match id.0 {
            0x00 => BundleDelimiter::parse(id, i).map(|(i, v)| (i, Self::BundleDelimiter(v))),
            0x01 => SpawnEntity::parse(id, i).map(|(i, v)| (i, Self::SpawnEntity(v))),
            0x02 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "SpawnExperienceOrb",
            ))),
            0x03 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "EntityAnimation",
            ))),
            0x04 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "AwardStatistics",
            ))),
            0x05 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "AcknowledgeBlockChange",
            ))),
            0x06 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "SetBlockDestroyStage",
            ))),
            0x07 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "BlockEntityData",
            ))),
            0x08 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "BlockAction",
            ))),
            0x09 => BlockUpdate::parse(id, i).map(|(i, v)| (i, Self::BlockUpdate(v))),
            0x0a => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "BossBar",
            ))),
            0x0b => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "ChangeDifficulty",
            ))),
            0x0c => ChunkBatchFinished::parse(id, i).map(|(i, v)| (i, Self::ChunkBatchFinished(v))),
            0x0d => ChunkBatchStart::parse(id, i).map(|(i, v)| (i, Self::ChunkBatchStart(v))),
            0x1a => Err(nom::Err::Error(crate::general::ParseError::NotImplemented("Damage Event"))),
            0x1f => EntityEvent::parse(id, i).map(|(i, v)| (i, Self::EntityEvent(v))),
            0x20 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "Explosion",
            ))),
            0x21 => UnloadChunk::parse(id, i).map(|(i, v)| (i, Self::UnloadChunk(v))),
            0x22 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "GameEvent",
            ))),
            0x23 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "OpenHorseScreen",
            ))),
            0x24 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "HurtAnimation",
            ))),
            0x25 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "InitializeWorldBorder",
            ))),
            0x26 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "ClientBound-KeepAlive",
            ))),
            0x27 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "ChunkDataAndUpdateLight",
            ))),
            0x28 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "WorldEvent",
            ))),
            0x29 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "Particle",
            ))),
            0x2a => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "Update Light",
            ))),
            0x2b => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "Login",
            ))),
            0x2c => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "MapData",
            ))),
            0x2d => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "MerchantOffers",
            ))),
            0x2e => {
                UpdateEntityPosition::parse(id, i).map(|(i, v)| (i, Self::UpdateEntityPosition(v)))
            }
            0x2f => UpdateEntityPositionAndRotation::parse(id, i)
                .map(|(i, v)| (i, Self::UpdateEntityPositionAndRotation(v))),
            0x30 => {
                UpdateEntityRotation::parse(id, i).map(|(i, v)| (i, Self::UpdateEntityRotation(v)))
            }
            0x42 => RemoveEntities::parse(id, i).map(|(i, v)| (i, Self::RemoveEntities(v))),
            0x48 => SetHeadRotation::parse(id, i).map(|(i, v)| (i, Self::SetHeadRotation(v))),
            0x49 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "UpdateSectionBlocks",
            ))),
            0x54 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "SetCenterChunk",
            ))),
            0x58 => SetEntityMetadata::parse(id, i).map(|(i, v)| (i, Self::SetEntityMetadata(v))),
            0x5a => SetEntityVelocity::parse(id, i).map(|(i, v)| (i, Self::SetEntityVelocity(v))),
            0x5b => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "SetEquipment",
            ))),
            0x64 => UpdateTime::parse(id, i).map(|(i, v)| (i, Self::UpdateTime(v))),
            0x68 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "SoundEffect",
            ))),
            0x70 => TeleportEntity::parse(id, i).map(|(i, v)| (i, Self::TeleportEntity(v))),
            0x75 => Err(nom::Err::Error(crate::general::ParseError::NotImplemented(
                "UpdateAttributes",
            ))),
            other => Err(nom::Err::Error(crate::general::ParseError::Other)),
        }
    }
}

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
declare_packet!(
    BlockUpdate,
    0x09,
    false,
    (location, Position),
    (block_id, VarInt)
);
declare_packet!(ChunkBatchFinished, 0x0c, false, (size, VarInt));
declare_packet!(ChunkBatchStart, 0x0d, false,);
declare_packet!(
    EntityEvent,
    0x1f,
    false,
    (entity_id, i32),
    (entity_status, i8)
);
declare_packet!(UnloadChunk, 0x21, false, (chunk_z, i32), (chunk_x, i32));
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
declare_packet!(RemoveEntities, 0x42, false, (entity_ids, Vec<VarInt>));
declare_packet!(
    SetHeadRotation,
    0x48,
    false,
    (entity_id, VarInt),
    (angle, u8)
);
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
