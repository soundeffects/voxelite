use bevy::{
	utils::HashMap,
	tasks::{AsyncComputeTaskPool, Task},
	prelude::*
};
use futures_lite::future::{block_on, poll_once};
use crate::{
	chunk::{Chunk, CHUNK_DIM},
	player_controller::{PlayerController, PlayerSettings}
};

#[derive(Default)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<World>()
			.add_system(spawn_chunk_tasks)
			.add_system(handle_chunk_tasks)
			.add_system(despawn_chunks);
	}
}

#[derive(Clone)]
enum ChunkSlot {
	Generating,
	Generated(Chunk)
}

#[derive(Default)]
pub struct World {
	chunks: HashMap<IVec3, ChunkSlot>
}

impl World {
	pub fn chunk_pos(world_position: Vec3) -> IVec3 {
		IVec3::new(
			(world_position.x / CHUNK_DIM as f32).floor() as i32,
			(world_position.y / CHUNK_DIM as f32).floor() as i32,
			(world_position.z / CHUNK_DIM as f32).floor() as i32
		)
	}

	pub fn world_position(chunk_pos: IVec3) -> Vec3 {
		Vec3::new(
			(chunk_pos.x * CHUNK_DIM as i32) as f32,
			(chunk_pos.y * CHUNK_DIM as i32) as f32,
			(chunk_pos.z * CHUNK_DIM as i32) as f32
		)
	}
}
struct ChunkResult(IVec3, ChunkSlot);

#[derive(Component)]
struct ChunkResultTask(Task<ChunkResult>);

fn spawn_chunk_tasks(
	mut commands: Commands,
	settings: Res<PlayerSettings>,
	mut world: ResMut<World>,
	query: Query<&Transform, With<PlayerController>>
) {
	let thread_pool = AsyncComputeTaskPool::get();

	for player_transform in &query {
		let player_chunk = World::chunk_pos(player_transform.translation);
		let dist = settings.view_distance as i32;

		for x in -dist..dist {
			for y in -dist..dist {
				for z in -dist..dist {
					let chunk_pos = player_chunk + IVec3::new(x, y, z);
					if !world.chunks.contains_key(&chunk_pos) {
						let task = thread_pool.spawn(async move {
							ChunkResult(
								chunk_pos,
								ChunkSlot::Generated(Chunk::generate())
							)
						});
						commands.spawn().insert(ChunkResultTask(task));
						world.chunks.insert(chunk_pos, ChunkSlot::Generating);
					}
				}
			}
		}
	}
}

fn handle_chunk_tasks(
	mut commands: Commands,
	mut world: ResMut<World>,
	mut tasks: Query<(Entity, &mut ChunkResultTask)>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>
) {
	for (entity, mut task) in &mut tasks {
		if let Some(ChunkResult(chunk_pos, generated_chunk))
			= block_on(poll_once(&mut task.0)) {
				if let ChunkSlot::Generated(chunk) = generated_chunk {
					let mesh = chunk.mesh(&mut meshes);
					world.chunks.insert(chunk_pos, ChunkSlot::Generated(chunk));
					let mut material
						= StandardMaterial::from(Color::rgb(0.0, 0.0, 0.0));
					material.perceptual_roughness = 0.9;
					commands.spawn_bundle(PbrBundle {
						mesh,
						material: materials.add(material),
						transform: Transform::from_translation(World::world_position(chunk_pos)),
						..Default::default()
					});
				}
				commands.entity(entity).remove::<ChunkResultTask>();
			}

	}
}

fn despawn_chunks() {

}
