use crate::world::{
    coordinates::{ChunkCoordinate, WorldChunkCoordinate, WorldCoordinate},
    Block, BlockData,
};

/// A `16x16xY` large collection of blocks
#[derive(PartialEq)]
pub struct Chunk<const L: i16, const U: i16> {
    /// The Chunk-X coordinate
    x: WorldChunkCoordinate,
    /// The Chunk-Z coordinate
    z: WorldChunkCoordinate,
    blocks: Vec<Block>,
}

impl<const L: i16, const U: i16> core::fmt::Debug for Chunk<L, U> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Chunk")
            .field("x", &self.x)
            .field("z", &self.z)
            .field_with("blocks", |f| {
                let mut list = f.debug_list();
                for y in L..U {
                    let start_index = (y + Self::OFFSET) as usize * 16 * 16;
                    let end_index = start_index + 16 * 16;
                    let layer_blocks = &self.blocks[start_index..end_index];

                    list.entry_with(|f| {
                        let mut l_struct = f.debug_struct("Layer");
                        l_struct.field("y", &y);

                        if layer_blocks
                            .iter()
                            .all(|b| matches!(b.block(), BlockData::Air))
                        {
                            l_struct.field("blocks", &"Air");
                        } else {
                            l_struct.field("blocks", &layer_blocks);
                        }
                        l_struct.finish()
                    });
                }
                list.finish()
            })
            .finish()
    }
}

impl<const L: i16, const U: i16> Chunk<L, U> {
    const OFFSET: i16 = -L;

    /// Generates a chunk filled with only air
    pub fn air(x: WorldChunkCoordinate, z: WorldChunkCoordinate) -> Self {
        let mut blocks = Vec::with_capacity(U.abs_diff(L) as usize * 16 * 16);
        for y in L..U {
            for x in 0..16 {
                for z in 0..16 {
                    blocks.push(Block {
                        last_modified: 0,
                        x,
                        z,
                        y,
                        data: BlockData::Air,
                    })
                }
            }
        }

        Self { x, z, blocks }
    }

    pub fn from_fn<F>(x: WorldChunkCoordinate, z: WorldChunkCoordinate, mut func: F) -> Self
    where
        F: FnMut(u8, u8, i16) -> BlockData,
    {
        let mut blocks = Vec::with_capacity(U.abs_diff(L) as usize * 16 * 16);
        for y in L..U {
            for x in 0..16 {
                for z in 0..16 {
                    blocks.push(Block {
                        last_modified: 0,
                        x,
                        z,
                        y,
                        data: func(x, z, y),
                    })
                }
            }
        }

        Self { x, z, blocks }
    }

    pub fn world_coordinates(&self) -> (WorldCoordinate, WorldCoordinate) {
        (self.x.into(), self.z.into())
    }

    pub fn chunk_coordinates(&self) -> (WorldChunkCoordinate, WorldChunkCoordinate) {
        (self.x, self.z)
    }

    pub fn get_block(&self, x: ChunkCoordinate, y: i16, z: ChunkCoordinate) -> Option<&Block> {
        if x.0 >= 16 || z.0 >= 16 || y < L || y > U {
            return None;
        }

        let idx = ((y + Self::OFFSET) as usize) * 16 * 16 + (z.0 as usize) * 16 + (x.0 as usize);

        self.blocks.get(idx)
    }

    pub fn set_block(
        &mut self,
        x: ChunkCoordinate,
        y: i16,
        z: ChunkCoordinate,
        tick: u64,
        value: BlockData,
    ) {
        if x.0 >= 16 || z.0 >= 16 || y < L || y > U {
            return; // TODO
        }

        let idx = ((y + Self::OFFSET) as usize) * 16 * 16 + (z.0 as usize) * 16 + (x.0 as usize);

        let block = self.blocks.get_mut(idx).unwrap(); // TODO
        block.last_modified = tick;
        block.data = value;
    }

    pub fn update_block<F, T>(
        &mut self,
        x: ChunkCoordinate,
        y: i16,
        z: ChunkCoordinate,
        update: F,
    ) -> Result<T, ()>
    where
        F: FnOnce(&mut Block) -> Result<T, ()>,
    {
        if x.0 >= 16 || z.0 >= 16 || y < L || y > U {
            return Err(());
        }

        let idx = ((y + Self::OFFSET) as usize) * 16 * 16 + (z.0 as usize) * 16 + (x.0 as usize);

        let block = self.blocks.get_mut(idx).ok_or(())?;

        let result = update(block)?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_air_chunk() {
        let _chunk = Chunk::<0, 100>::air(WorldChunkCoordinate(0), WorldChunkCoordinate(0));
    }

    #[test]
    fn get_block_in_chunk() {
        let chunk = Chunk::<0, 100>::air(WorldChunkCoordinate(0), WorldChunkCoordinate(0));

        let block = chunk
            .get_block(ChunkCoordinate(0), 0, ChunkCoordinate(0))
            .unwrap();
        assert_eq!(BlockData::Air, block.data);
    }

    #[test]
    fn update_value_in_chunk() {
        let mut chunk = Chunk::<0, 100>::air(WorldChunkCoordinate(0), WorldChunkCoordinate(0));

        let block = chunk
            .get_block(ChunkCoordinate(0), 0, ChunkCoordinate(0))
            .unwrap();
        assert_eq!(BlockData::Air, block.data);
        assert_eq!(0, block.last_modified());

        chunk.set_block(
            ChunkCoordinate(0),
            0,
            ChunkCoordinate(0),
            12,
            BlockData::Stone,
        );
        let block = chunk
            .get_block(ChunkCoordinate(0), 0, ChunkCoordinate(0))
            .unwrap();
        assert_eq!(BlockData::Stone, block.data);
        assert_eq!(12, block.last_modified());
    }
}
