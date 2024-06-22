//! Contains different Data that is needed, like RegistryData

/// Contains the registry related fixed data
pub mod registry {
    use protocol::{configuration::client::RegistryEntry, general::PString};

    pub fn all_registries() -> impl Iterator<Item = (PString<'static>, Vec<RegistryEntry>)> {
        [
            dimension_type(),
            worldgen_biome(),
            chat_type(),
            trim_pattern(),
            trim_material(),
            wolf_variant(),
            damage_type(),
            banner_pattern(),
        ]
        .into_iter()
    }

    pub fn dimension_type() -> (PString<'static>, Vec<RegistryEntry>) {
        (
            protocol::general::PString("minecraft:dimension_type".into()),
            vec![
                RegistryEntry {
                    id: PString("minecraft:overworld".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:overworld_caves".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:the_end".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:the_nether".into()),
                    data: None,
                },
            ],
        )
    }

    pub fn worldgen_biome() -> (PString<'static>, Vec<RegistryEntry>) {
        (
            protocol::general::PString("minecraft:worldgen/biome".into()),
            vec![
                RegistryEntry {
                    id: PString("minecraft:badlands".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:bamboo_jungle".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:basalt_deltas".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:beach".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:birch_forest".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:cherry_grove".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:cold_ocean".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:crimson_forest".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:dark_forest".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:deep_cold_ocean".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:deep_dark".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:deep_frozen_ocean".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:deep_lukewarm_ocean".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:deep_ocean".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:desert".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:dripstone_caves".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:end_barrens".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:end_highlands".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:end_midlands".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:eroded_badlands".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:flower_forest".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:forest".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:frozen_ocean".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:frozen_peaks".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:frozen_river".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:grove".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:ice_spikes".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:jagged_peaks".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:jungle".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:lukewarm_ocean".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:lush_caves".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:mangrove_swamp".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:meadow".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:mushroom_fields".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:nether_wastes".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:ocean".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:old_growth_birch_forest".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:old_growth_pine_taiga".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:old_growth_spruce_taiga".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:plains".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:river".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:savanna".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:savanna_plateau".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:small_end_islands".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:snowy_beach".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:snowy_plains".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:snowy_slopes".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:snowy_taiga".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:soul_sand_valley".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:sparse_jungle".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:stony_peaks".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:stony_shore".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:sunflower_plains".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:swamp".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:taiga".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:the_end".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:the_void".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:warm_ocean".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:warped_forest".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:windswept_forest".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:windswept_gravelly_hills".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:windswept_hills".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:windswept_savanna".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:wooded_badlands".into()),
                    data: None,
                },
            ],
        )
    }

    pub fn chat_type() -> (PString<'static>, Vec<RegistryEntry>) {
        (
            protocol::general::PString("minecraft:chat_type".into()),
            vec![
                RegistryEntry {
                    id: PString("minecraft:chat".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:emote_command".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:msg_command_incoming".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:msg_command_outgoing".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:say_command".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:team_msg_command_incoming".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:team_msg_command_outgoing".into()),
                    data: None,
                },
            ],
        )
    }

    pub fn trim_pattern() -> (PString<'static>, Vec<RegistryEntry>) {
        (
            protocol::general::PString("minecraft:trim_pattern".into()),
            vec![
                RegistryEntry {
                    id: PString("minecraft:coast".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:dune".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:eye".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:host".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:raiser".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:rib".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:sentry".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:shaper".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:silence".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:snout".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:spire".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:tide".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:vex".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:ward".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:wayfinder".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:wild".into()),
                    data: None,
                },
            ],
        )
    }

    pub fn trim_material() -> (PString<'static>, Vec<RegistryEntry>) {
        (
            protocol::general::PString("minecraft:trim_material".into()),
            vec![
                RegistryEntry {
                    id: PString("minecraft:amethyst".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:copper".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:diamond".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:emerald".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:gold".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:iron".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:lapis".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:netherite".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:quartz".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:redstone".into()),
                    data: None,
                },
            ],
        )
    }

    pub fn wolf_variant() -> (PString<'static>, Vec<RegistryEntry>) {
        (
            protocol::general::PString("minecraft:wolf_variant".into()),
            vec![
                RegistryEntry {
                    id: PString("minecraft:ashen".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:black".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:chestnut".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:pale".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:rusty".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:snowy".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:spotted".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:striped".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:woods".into()),
                    data: None,
                },
            ],
        )
    }

    pub fn damage_type() -> (PString<'static>, Vec<RegistryEntry>) {
        (
            protocol::general::PString("minecraft:damage_type".into()),
            vec![
                RegistryEntry {
                    id: PString("minecraft:arrow".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:bad_respawn_point".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:cactus".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:cramming".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:dragon_breath".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:drown".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:dry_out".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:explosion".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:fall".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:falling_anvil".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:falling_block".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:falling_stalactite".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:fireball".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:fireworks".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:fly_into_wall".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:freeze".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:generic".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:generic_kill".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:hot_floor".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:in_fire".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:in_wall".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:indirect_magic".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:lava".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:lightning_bolt".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:magic".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:mob_attack".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:mob_attack_no_aggro".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:mob_projectile".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:on_fire".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:out_of_world".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:outside_border".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:player_attack".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:player_explosion".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:sonic_boom".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:spit".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:stalagmite".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:starve".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:sting".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:sweet_berry_bush".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:thorns".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:thrown".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:trident".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:unattributed_fireball".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:wither".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:wither_skull".into()),
                    data: None,
                },
            ],
        )
    }

    pub fn banner_pattern() -> (PString<'static>, Vec<RegistryEntry>) {
        (
            protocol::general::PString("minecraft:banner_pattern".into()),
            vec![
                RegistryEntry {
                    id: PString("minecraft:base".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:border".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:bricks".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:circle".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:creeper".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:cross".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:curly_border".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:diagonal_left".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:diagonal_right".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:diagonal_up_left".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:diagonal_up_right".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:flower".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:globe".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:gradient".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:gradient_up".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:half_horizontal".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:half_horizontal_bottom".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:half_vertical".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:half_vertical_right".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:mojang".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:piglin".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:rhombus".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:skull".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:small_stripes".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:square_bottom_left".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:square_bottom_right".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:square_top_left".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:square_top_right".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:straight_cross".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:stripe_bottom".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:stripe_center".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:stripe_downleft".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:stripe_downright".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:stripe_left".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:stripe_middle".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:stripe_right".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:stripe_top".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:triangle_bottom".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:triangle_top".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:triangles_bottom".into()),
                    data: None,
                },
                RegistryEntry {
                    id: PString("minecraft:triangles_top".into()),
                    data: None,
                },
            ],
        )
    }
}
