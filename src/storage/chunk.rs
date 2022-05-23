use super::row::{CHUNK_SIZE, Row};

/*
 * Chunk
 *
 * Chunks are groupings of voxels, themselves being defined as cubic
 * structures in 3D space. Each chunk holds 32^3 voxels. Chunks store
 * sparse voxel data by linearized rows: if a row holds only one type of
 * voxel, then only one slot of the row is allocated. This achieves
 * efficient memory usage and voxel read times, at the cost of voxel write
 * times.
 */
pub struct Chunk {
	rows: [Box<dyn Row>; CHUNK_SIZE * CHUNK_SIZE]
}
impl Chunk {
	pub fn new() -> Self {
		Chunk {
			rows: Vec::new()
		}
	}
}

/*
 * ChunkPosition
 *
 * Used as a key for VoxelMap, and determines the relative location of
 * chunks to each other.
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPosition {
	x: i32,
	y: i32,
	z: i32
}
impl ChunkPosition {
	pub fn exceeds_bounds(& self, bounds: ChunkPosition) -> bool {
		self.x.abs() > bounds.x ||
			self.y.abs() > bounds.y ||
			self.z.abs() > bounds.z
	}
}
