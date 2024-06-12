//! All the world related information

pub mod coordinates;

mod chunk;
pub use chunk::Chunk;

pub struct Region {
    chunks: Vec<Chunk<-64, 320>>,
}

/// An individual Block
#[derive(Debug, PartialEq)]
pub struct Block {
    /// The Tick in which the block was last updated
    last_modified: u64,
    /// The X cooridnate relative to a chunks position, NOT the world position
    x: u8,
    /// The Z coordinate relative to a chunks position, NOT the world position
    z: u8,
    /// The Y coordinate, which is also the world position
    y: i16,
    /// The actual data for this block
    data: BlockData,
}

impl Block {
    pub fn last_modified(&self) -> u64 {
        self.last_modified
    }

    pub fn is_solid(&self) -> bool {
        matches!(self.data, BlockData::Stone | BlockData::Dirt)
    }

    pub fn block(&self) -> &BlockData {
        &self.data
    }
}

#[derive(Debug, PartialEq)]
pub enum BlockData {
    Air,
    Stone,
    Dirt,
    Water { height: u8 },
    WaterSource,
}
