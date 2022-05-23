use bevy::prelude::{Plugin, App};

mod storage;
pub mod material;
mod messages;
mod world;

use material::VoxelMaterialRegistry;

pub struct VoxelServerPlugin;
impl Plugin for VoxelServerPlugin {
	fn build(& self, app: & mut App) {
		app.init_resource::<VoxelMaterialRegistry>();
		/*
		app.insert_resource(VoxelMap::<Voxel, ChunkShape>::new(ChunkShape {}))
			.add_plugin(chunks::VoxelChunkingPlugin)
			.add_plugin(meshing::VoxelMeshingPlugin)
			.add_plugin(terrain::VoxelTerrainPlugin)
			.add_plugin(super::render::VoxelRenderPipelinePlugin)
			.add_plugin(super::material::VoxelMaterialPlugin)
			.add_plugin(materials::VoxelBaseMaterialsPlugin)
			.add_plugin(player::VoxelWorldPlayerControllerPlugin);
		*/
	}
}

/*
use ndshape::{ConstPow2Shape2u8, ConstPow2Shape3u16};

pub struct Chunk([Voxel; ConstPow2Shape3u16::SIZE as usize]);
pub struct Chunk32([Voxel; ConstPow2Shape3u32::SIZE as usize]);

const EMPTY: Chunk = Chunk([AIR; ConstPow2Shape3u16::SIZE as usize]);

struct Map {
  chunk_shape: ConstPow2Shape3u16,
  chunk_map: [Chunk; ConstPow2Shape2u8::SIZE as usize]
}
*/
