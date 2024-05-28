use crate::{declare_packet, general::VarInt};

#[derive(Debug, PartialEq)]
pub enum Play {
    ChunkBatchReceived(ChunkBatchReceived),
    SetPlayerPosition(SetPlayerPosition),
    SetPlayerPositionAndRotation(SetPlayerPositionAndRotation),
    SetPlayerRotation(SetPlayerRotation),
}

impl Play {
    pub fn parse(id: VarInt, i: &[u8]) -> nom::IResult<&[u8], Self, crate::general::ParseError> {
        match id.0 {
            0x08 => ChunkBatchReceived::parse(id, i).map(|(i, v)| (i, Self::ChunkBatchReceived(v))),
            0x1a => SetPlayerPosition::parse(id, i).map(|(i, v)| (i, Self::SetPlayerPosition(v))),
            0x1b => SetPlayerPositionAndRotation::parse(id, i).map(|(i, v)| (i, Self::SetPlayerPositionAndRotation(v))),
            0x1c => SetPlayerRotation::parse(id, i).map(|(i, v)| (i, Self::SetPlayerRotation(v))),
            other => Err(nom::Err::Error(crate::general::ParseError::Other)),
        }
    }
}

declare_packet!(ChunkBatchReceived, 0x08, false, (chunks_per_tick, f32));
declare_packet!(SetPlayerPosition, 0x1a, false, (x, f64), (feet_y, f64), (z, f64), (on_ground, bool));
declare_packet!(SetPlayerPositionAndRotation, 0x1b, false, (x, f64), (feet_y, f64), (z, f64), (yaw, f32), (pitch, f32), (on_ground, bool));
declare_packet!(SetPlayerRotation, 0x1c, false, (yaw, f32), (pitch, f32), (on_ground, bool));
