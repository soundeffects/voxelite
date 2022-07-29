use bevy::{
	utils::HashMap,
	prelude::*
};

use crate::chunk::Chunk;

#[derive(Default)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<World>()
			//.add_system(spawn_chunk_gen_tasks)
			//.add_system(handle_chunk_gen_tasks)
			.add_system(mesh_generation)
			.add_system(despawn_chunks);
	}
}

/*enum ChunkSlot {
	Generating,
	Generated(Chunk),
	Visible(Chunk)
}*/

#[derive(Default)]
pub struct World {
	chunks: HashMap<IVec3, Chunk>
}

impl World {
	pub fn chunk_pos(p: Vec3) -> IVec3 {
		((p - (Chunk::SIZE as f32 / 2.0)) / Chunk::SIZE as f32).round()
			.as_ivec3()
	}

	pub fn world_pos(p: IVec3) -> Vec3 {
		p.as_vec3() * Chunk::SIZE as f32
	}
}

#[derive(Debug, Default, Clone, Copy, Component, PartialEq, Eq)]
struct GeneratedChunk(IVec3);

#[derive(Debug, Default, Clone, Copy, Component, PartialEq, Eq)]
struct VisibleChunk(IVec3);

struct ChunkResult(IVec3, Chunk);

/*fn voxel_generation(
	mut commands: Commands,
	mut world: ResMut<World>,
	settings: Res<PlayerSettings>,
	noise: Res<Noise>,
	thread_pool: Res<AsyncComputeTaskPool>,
	query: Query<&Transform, With<PlayerController>>
) {
	for transform in query.iter() {
		let center = World::chunk_pos(transform.translation);
		let generation_distance = settings.view_distance as i32 + 1;

	}
}*/

fn mesh_generation(
	mut commands: Commands,
	mut world: ResMut<World>,
	mut meshes: ResMut<Assets<Mesh>>,
	settings: Res<PlayerSettings>,
	player_query: Query<&Transform, With<PlayerController>>,
	chunk_query: Query<(Entity, &GeneratedChunk)>
) {

}
