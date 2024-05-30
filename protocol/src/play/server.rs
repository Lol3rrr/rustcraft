use crate::{
    combined_packet, declare_packet,
    general::{Position, VarInt},
};

combined_packet!(
    Play,
    ConfirmTeleportation,
    ChunkBatchReceived,
    Interact,
    KeepAlive,
    SetPlayerPosition,
    SetPlayerPositionAndRotation,
    SetPlayerRotation,
    SetPlayerOnGround,
    PlayerAction,
    PlayerCommand,
    SetHeldItem,
    SwingArm
);

declare_packet!(ConfirmTeleportation, 0x00, false, (teleport_id, VarInt));
declare_packet!(ChunkBatchReceived, 0x08, false, (chunks_per_tick, f32));

declare_packet!(Interact, 0x16, false,); // TODO
declare_packet!(KeepAlive, 0x18, false,); // TODO
declare_packet!(
    SetPlayerPosition,
    0x1a,
    false,
    (x, f64),
    (feet_y, f64),
    (z, f64),
    (on_ground, bool)
);
declare_packet!(
    SetPlayerPositionAndRotation,
    0x1b,
    false,
    (x, f64),
    (feet_y, f64),
    (z, f64),
    (yaw, f32),
    (pitch, f32),
    (on_ground, bool)
);
declare_packet!(
    SetPlayerRotation,
    0x1c,
    false,
    (yaw, f32),
    (pitch, f32),
    (on_ground, bool)
);
declare_packet!(SetPlayerOnGround, 0x1d, false,); // TODO

declare_packet!(
    PlayerAction,
    0x24,
    false,
    (status, VarInt),
    (location, Position),
    (face, i8),
    (sequence, VarInt)
);
declare_packet!(
    PlayerCommand,
    0x25,
    false,
    (entity_id, VarInt),
    (action_id, VarInt),
    (jump_boost, VarInt)
);
declare_packet!(SetHeldItem, 0x2f, false,); // TODO

declare_packet!(SwingArm, 0x36, false, (hand, VarInt));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn implemented_x00() {
        let result = Play::parse(VarInt(0x00), &[]);
        match result {
            Err(nom::Err::Error(crate::general::ParseError::UnknownPacketId(_))) => {
                panic!("Unimplemented Packet ID")
            }
            _ => {}
        };
    }

    #[test]
    fn implemented_x16() {
        let result = Play::parse(VarInt(0x16), &[]);
        match result {
            Err(nom::Err::Error(crate::general::ParseError::UnknownPacketId(_))) => {
                panic!("Unimplemented Packet ID")
            }
            _ => {}
        };
    }

    #[test]
    fn implemented_x18() {
        let result = Play::parse(VarInt(0x18), &[]);
        match result {
            Err(nom::Err::Error(crate::general::ParseError::UnknownPacketId(_))) => {
                panic!("Unimplemented Packet ID")
            }
            _ => {}
        };
    }

    #[test]
    fn implemented_x1d() {
        let result = Play::parse(VarInt(0x1d), &[]);
        match result {
            Err(nom::Err::Error(crate::general::ParseError::UnknownPacketId(_))) => {
                panic!("Unimplemented Packet ID")
            }
            _ => {}
        };
    }

    #[test]
    fn implemented_x24() {
        let result = Play::parse(VarInt(0x24), &[]);
        match result {
            Err(nom::Err::Error(crate::general::ParseError::UnknownPacketId(_))) => {
                panic!("Unimplemented Packet ID")
            }
            _ => {}
        };
    }

    #[test]
    fn implemented_x2f() {
        let result = Play::parse(VarInt(0x2f), &[]);
        match result {
            Err(nom::Err::Error(crate::general::ParseError::UnknownPacketId(_))) => {
                panic!("Unimplemented Packet ID")
            }
            _ => {}
        };
    }

    #[test]
    fn implemented_x36() {
        let result = Play::parse(VarInt(0x36), &[]);
        match result {
            Err(nom::Err::Error(crate::general::ParseError::UnknownPacketId(_))) => {
                panic!("Unimplemented Packet ID")
            }
            _ => {}
        };
    }
}
