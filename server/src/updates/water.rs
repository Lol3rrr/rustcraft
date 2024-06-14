use crate::world::{coordinates::ChunkCoordinate, Block, BlockData, Chunk};

pub struct WaterUpdates {
    speed: u64,
}

impl WaterUpdates {
    pub fn new(speed: u64) -> Self {
        Self { speed }
    }

    pub fn update<const L: i16, const U: i16>(
        &self,
        previous: &Chunk<L, U>,
        chunk: &mut Chunk<L, U>,
        tick: u64,
    ) {
        for y in (L..U).rev() {
            for x in 0..16 {
                for z in 0..16 {
                    let block = previous
                        .get_block(ChunkCoordinate(x), y, ChunkCoordinate(z))
                        .unwrap(); // TODO

                    // If the block is solid, water will never flow into that block itself
                    if block.is_solid() {
                        continue;
                    }

                    // If the block is already a water source, skip it
                    if matches!(block.block(), BlockData::WaterSource) {
                        continue;
                    }

                    if self
                        .flow_from_above(
                            block,
                            previous,
                            chunk,
                            ChunkCoordinate(x),
                            y,
                            ChunkCoordinate(z),
                            tick,
                        )
                        .unwrap_or(false)
                    {
                        continue;
                    }

                    if self
                        .flow_from_source(
                            block,
                            previous,
                            chunk,
                            ChunkCoordinate(x),
                            y,
                            ChunkCoordinate(z),
                            tick,
                        )
                        .unwrap_or(false)
                    {
                        continue;
                    }

                    if self
                        .flow_from_water(
                            block,
                            previous,
                            chunk,
                            ChunkCoordinate(x),
                            y,
                            ChunkCoordinate(z),
                            tick,
                        )
                        .unwrap_or(false)
                    {
                        continue;
                    }
                }
            }
        }
    }

    /// Updates a block with water, if there is water above it
    fn flow_from_above<const L: i16, const U: i16>(
        &self,
        block: &Block,
        previous: &Chunk<L, U>,
        chunk: &mut Chunk<L, U>,
        x: ChunkCoordinate,
        y: i16,
        z: ChunkCoordinate,
        tick: u64,
    ) -> Result<bool, ()> {
        let above = match previous.get_block(x, y + 1, z) {
            Some(a) => a,
            None => return Ok(false),
        };

        if matches!(
            above.block(),
            BlockData::Water { .. } | BlockData::WaterSource
        ) && !matches!(
            block.block(),
            BlockData::Water { height: 8 } | BlockData::WaterSource
        ) && above.last_modified() + self.speed <= tick
        {
            chunk.set_block(x, y, z, tick, BlockData::Water { height: 8 });
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn flow_from_source<const L: i16, const U: i16>(
        &self,
        block: &Block,
        previous: &Chunk<L, U>,
        chunk: &mut Chunk<L, U>,
        x: ChunkCoordinate,
        y: i16,
        z: ChunkCoordinate,
        tick: u64,
    ) -> Result<bool, ()> {
        let offsets = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        let coordinates = core::iter::repeat((x, z))
            .zip(offsets.iter().copied())
            .map(|((x, z), (x_off, z_off))| (x.0 as i16 - x_off, z.0 as i16 - z_off));

        let mut found = 0;
        for (n_x, n_z) in coordinates {
            if n_x < 0 || n_x > 15 || n_z < 0 || n_z > 15 {
                // TODO
                // How do we fetch data from outside this chunk

                continue;
            }

            let block = previous
                .get_block(ChunkCoordinate(n_x as u8), y, ChunkCoordinate(n_z as u8))
                .ok_or(())?;

            if matches!(block.block(), BlockData::WaterSource)
                && block.last_modified() + self.speed <= tick
            {
                dbg!((y, x, z, n_x, n_z));
                found += 1;
            }
        }

        if found == 1 {
            chunk.set_block(x, y, z, tick, BlockData::Water { height: 7 });
        } else if found > 1 {
            chunk.set_block(x, y, z, tick, BlockData::WaterSource);
        }

        Ok(found > 0)
    }

    fn flow_from_water<const L: i16, const U: i16>(
        &self,
        block: &Block,
        previous: &Chunk<L, U>,
        chunk: &mut Chunk<L, U>,
        x: ChunkCoordinate,
        y: i16,
        z: ChunkCoordinate,
        tick: u64,
    ) -> Result<bool, ()> {
        let offsets = [(0, -1), (1, 0), (0, 1), (-1, 0)];
        let coordinates = core::iter::repeat((x, z))
            .zip(offsets.iter().copied())
            .map(|((x, z), (x_off, z_off))| (x.0 as i16 - x_off, z.0 as i16 - z_off));

        let mut height: u8 = 0;
        for (n_x, n_z) in coordinates {
            if n_x < 0 || n_x > 15 || n_z < 0 || n_z > 15 {
                // TODO
                // Load data from outside the chunk

                continue;
            }

            let block = previous
                .get_block(ChunkCoordinate(n_x as u8), y, ChunkCoordinate(n_z as u8))
                .ok_or(())?;

            let b_height = match block.block() {
                BlockData::Water { height } => height,
                _ => continue,
            };

            // TODO
            height = height.max(*b_height);
        }

        if height > 1 {
            chunk.set_block(x, y, z, tick, BlockData::Water { height: height - 1 });
        }

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::coordinates::*;
    use pretty_assertions::assert_eq;

    macro_rules! place_blocks {
        ($chunk:ident, $tick:literal, $(($x:literal, $y:literal, $z:literal, $data:expr),)*) => {{
            $(
                $chunk.set_block(ChunkCoordinate($x), $y, ChunkCoordinate($z), $tick, $data);
            )*
        }}
    }

    #[test]
    fn empty_world() {
        let air_chunk = Chunk::<0, 10>::air(WorldChunkCoordinate(0), WorldChunkCoordinate(0));

        let mut chunk = Chunk::air(WorldChunkCoordinate(0), WorldChunkCoordinate(0));

        let updater = WaterUpdates::new(1);

        updater.update(&air_chunk, &mut chunk, 1);

        // Make sure that nothing has changed and both are still just empty air chunks
        assert_eq!(air_chunk, chunk);
    }

    #[test]
    fn one_source_spreading_over_air() {
        let mut og_chunk = Chunk::<0, 20>::air(WorldChunkCoordinate(0), WorldChunkCoordinate(0));

        let mut chunk = Chunk::air(WorldChunkCoordinate(0), WorldChunkCoordinate(0));

        og_chunk.set_block(
            ChunkCoordinate(7),
            10,
            ChunkCoordinate(7),
            0,
            BlockData::WaterSource,
        );
        chunk.set_block(
            ChunkCoordinate(7),
            10,
            ChunkCoordinate(7),
            0,
            BlockData::WaterSource,
        );
        assert_eq!(og_chunk, chunk);

        let updater = WaterUpdates::new(1);

        updater.update(&og_chunk, &mut chunk, 10);

        assert_ne!(og_chunk, chunk);

        let mut expected = og_chunk;
        place_blocks!(
            expected,
            10,
            (7, 9, 7, BlockData::Water { height: 8 }),
            (6, 10, 7, BlockData::Water { height: 7 }),
            (8, 10, 7, BlockData::Water { height: 7 }),
            (7, 10, 6, BlockData::Water { height: 7 }),
            (7, 10, 8, BlockData::Water { height: 7 }),
        );

        assert_eq!(expected, chunk);
    }

    #[test]
    fn normal_water_spread() {
        let mut og_chunk = Chunk::<0, 20>::air(WorldChunkCoordinate(0), WorldChunkCoordinate(0));

        let mut chunk = Chunk::air(WorldChunkCoordinate(0), WorldChunkCoordinate(0));

        place_blocks!(
            og_chunk,
            0,
            (7, 10, 7, BlockData::WaterSource),
            (7, 9, 7, BlockData::Stone),
            (6, 9, 7, BlockData::Stone),
            (8, 9, 7, BlockData::Stone),
            (7, 9, 6, BlockData::Stone),
            (7, 9, 8, BlockData::Stone),
        );
        place_blocks!(
            chunk,
            0,
            (7, 10, 7, BlockData::WaterSource),
            (7, 9, 7, BlockData::Stone),
            (6, 9, 7, BlockData::Stone),
            (8, 9, 7, BlockData::Stone),
            (7, 9, 6, BlockData::Stone),
            (7, 9, 8, BlockData::Stone),
        );
        assert_eq!(og_chunk, chunk, "Starting point");

        let updater = WaterUpdates::new(1);

        updater.update(&og_chunk, &mut chunk, 10);

        assert_ne!(og_chunk, chunk, "Testing");

        let mut expected = og_chunk;
        place_blocks!(
            expected,
            10,
            (6, 10, 7, BlockData::Water { height: 7 }),
            (8, 10, 7, BlockData::Water { height: 7 }),
            (7, 10, 6, BlockData::Water { height: 7 }),
            (7, 10, 8, BlockData::Water { height: 7 }),
        );

        assert_eq!(expected, chunk, "Other");

        updater.update(&expected, &mut chunk, 20);

        // The same as above, just with the latest tick
        place_blocks!(
            expected,
            20,
            (6, 10, 7, BlockData::Water { height: 7 }),
            (8, 10, 7, BlockData::Water { height: 7 }),
            (7, 10, 6, BlockData::Water { height: 7 }),
            (7, 10, 8, BlockData::Water { height: 7 }),
        );

        // New Flow
        place_blocks!(
            expected,
            20,
            // Surrinding 6, 10, 7
            (5, 10, 7, BlockData::Water { height: 6 }),
            (6, 10, 8, BlockData::Water { height: 6 }),
            (6, 10, 6, BlockData::Water { height: 6 }),
            // Sourrinding 8, 10, 7
            (9, 10, 7, BlockData::Water { height: 6 }),
            (8, 10, 6, BlockData::Water { height: 6 }),
            (8, 10, 8, BlockData::Water { height: 6 }),
            // Sourrinding 7, 10, 8
            (7, 10, 9, BlockData::Water { height: 6 }),
            (6, 10, 8, BlockData::Water { height: 6 }),
            (8, 10, 8, BlockData::Water { height: 6 }),
            // Sourrinding 7, 10, 6
            (7, 10, 5, BlockData::Water { height: 6 }),
            (6, 10, 6, BlockData::Water { height: 6 }),
            (8, 10, 6, BlockData::Water { height: 6 }),
        );

        assert_eq!(expected, chunk);
    }
}
