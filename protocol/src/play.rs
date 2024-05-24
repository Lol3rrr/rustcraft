pub mod client {
    use crate::{
        general::{PString, Position, VarInt},
        serialize::SerializeItem,
    };

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
}
