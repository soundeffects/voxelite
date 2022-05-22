/*
 * Voxel
 *
 * The world is subdivided into a cubic grid, where each cell is a voxel.
 * This struct stores the characteristics that every cell holds at a
 * minimum. This includes the material that fills the cell.
 */
struct Voxel {
	material_id: u16
}
impl Default for Voxel {
	fn default() -> Self {
		Voxel { material_id: 0 }
	}
}

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
	rows: Vec<Voxel>
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
