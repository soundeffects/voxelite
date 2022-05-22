use bevy::utils::HashMap;

use crate::chunk::{Chunk, ChunkPosition};

/*
 * VoxelMap
 *
 * Stores Chunks in a HashMap, indexed to by ChunkPosition.
 */
pub struct VoxelMap {
	chunks: HashMap<ChunkPosition, Chunk>,
	settings: VoxelMapSettings
}
impl VoxelMap {
	pub fn new(settings: VoxelMapSettings) -> Self {
		VoxelMap {
			chunks: HashMap::default(),
			settings
		}
	}
	pub fn get_chunk(& self, position: ChunkPosition) -> Option<& Chunk> {
		if position.exceeds_bounds(self.settings.bounds) {
			return None
		}
		if !self.chunks.contains_key(& position) {
			self.chunks.insert(position, Chunk::new());
		}
		self.chunks.get(& position)
	}
}
impl Default for VoxelMap {
	fn default() -> Self {
		VoxelMap {
			..Default::default()
		}
	}
}

/*
 * VoxelMapSettings
 *
 * A struct passed when constructing a new VoxelMap. Determines a few
 * global values for the VoxelMap.
 */
pub struct VoxelMapSettings {
	bounds: ChunkPosition
}
impl Default for VoxelMapSettings {
	fn default() -> Self {
		// sensible defaults for a Minecraft-like world
		VoxelMapSettings {
			bounds: ChunkPosition {
				x: 500,
				y: 20,
				z: 500
			}
		}
	}
}
