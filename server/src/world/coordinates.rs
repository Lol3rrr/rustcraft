/// A Coordinate in World Space
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct WorldCoordinate(pub i64);

/// A Coordinate in World Chunk Space
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct WorldChunkCoordinate(pub i64);

/// An in Chunk Coordinate
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ChunkCoordinate(pub u8);

impl From<WorldChunkCoordinate> for WorldCoordinate {
    fn from(value: WorldChunkCoordinate) -> Self {
        Self(value.0 * 16)
    }
}

impl From<WorldCoordinate> for WorldChunkCoordinate {
    fn from(value: WorldCoordinate) -> Self {
        Self(value.0 / 16)
    }
}

impl From<WorldCoordinate> for (WorldChunkCoordinate, ChunkCoordinate) {
    fn from(value: WorldCoordinate) -> Self {
        (
            WorldChunkCoordinate(value.0 / 16),
            ChunkCoordinate((value.0 % 16) as u8),
        )
    }
}

impl core::ops::Add<ChunkCoordinate> for WorldCoordinate {
    type Output = WorldCoordinate;

    fn add(self, rhs: ChunkCoordinate) -> Self::Output {
        WorldCoordinate(self.0 + rhs.0 as i64)
    }
}
impl core::ops::Add<ChunkCoordinate> for WorldChunkCoordinate {
    type Output = WorldCoordinate;

    fn add(self, rhs: ChunkCoordinate) -> Self::Output {
        WorldCoordinate(self.0 * 16 + rhs.0 as i64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_to_chunk() {
        assert_eq!(WorldChunkCoordinate(11), WorldCoordinate(176).into());
        assert_eq!(WorldChunkCoordinate(11), WorldCoordinate(191).into());
        assert_eq!(WorldChunkCoordinate(12), WorldCoordinate(192).into());
    }

    #[test]
    fn chunk_to_world() {
        assert_eq!(WorldCoordinate(160), WorldChunkCoordinate(10).into());
        assert_eq!(WorldCoordinate(176), WorldChunkCoordinate(11).into());
    }

    #[test]
    fn world_plus_in_chunk() {
        assert_eq!(
            WorldCoordinate(166),
            WorldCoordinate(160) + ChunkCoordinate(6)
        );
    }

    #[test]
    fn chunk_plus_in_chunk() {
        assert_eq!(
            WorldCoordinate(166),
            WorldChunkCoordinate(10) + ChunkCoordinate(6)
        );
    }
}
