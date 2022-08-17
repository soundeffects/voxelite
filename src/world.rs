use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::utils::HashSet;
use bevy::prelude::*;
use futures_lite::future::{block_on, poll_once};
use ndshape::{Shape, RuntimeShape};
use crate::chunk::{Chunk, CHUNK_DIM};
use crate::meshing_chunk::MeshingChunk;
use crate::player_controller::{PlayerController, PlayerSettings};
use crate::voxel::EMPTY;
use crate::directions::Directions;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
	fn build(&self, app: &mut App) {
		app.insert_resource::<World>(World::generate([10, 4, 10]))
			.add_system(spawn_mesh_tasks)
			.add_system(handle_mesh_tasks);
	}
}

pub struct World {
	shape: RuntimeShape<u32, 3>,
	chunks: Vec<Chunk>,
	visible: HashSet<UVec3>
}

impl World {
	fn chunk_pos(world_position: Vec3) -> UVec3 {
		(world_position / CHUNK_DIM as f32).as_uvec3()
	}

	fn world_position(chunk_pos: UVec3) -> Vec3 {
		chunk_pos.as_vec3() * CHUNK_DIM as f32
	}

	fn bounded_add(&self, pos: UVec3, add: IVec3) -> Option<UVec3> {
		let unbounded_pos = pos.as_ivec3() + add;
		let bounds = self.shape.as_array();
		let in_bounds = |coord: i32, bound_index: usize| {
			coord >= 0 && coord < bounds[bound_index] as i32
		};
		if in_bounds(unbounded_pos.x, 0) && in_bounds(unbounded_pos.y, 1)
			&& in_bounds(unbounded_pos.z, 2) {
			Some(unbounded_pos.as_uvec3())
		} else {
			None
		}
	}

	fn generate(dimensions: [u32; 3]) -> Self {
		let shape = RuntimeShape::<u32, 3>::new(dimensions);
		let mut chunks = Vec::<Chunk>::new();
		for index in 0..shape.size() {
			info!("generated {} of {} chunks", index, shape.size());
			chunks.push(
				Chunk::generate(
					UVec3::from_array(shape.delinearize(index)),
					|_| { EMPTY }
				)
			);
		}
		World { shape, chunks, visible: HashSet::new() }
	}

	fn get_meshing_chunk(&self, chunk_pos: UVec3) -> MeshingChunk {
		let mut chunks: [Option<&Chunk>; 7] = [None; 7];
		let directions = Directions::all();
		for index in 0..chunks.len() {
			if let Some(pos)
				= self.bounded_add(chunk_pos, directions[index].to_vector()) {
				let world_index = self.shape.linearize(pos.to_array()) as usize;
				chunks[index] = Some(&self.chunks[world_index]);
			}
		}
		MeshingChunk::new(chunks)
	}
}


#[derive(Component)]
struct MeshResultTask(Task<MeshResult>);
struct MeshResult(UVec3, Mesh);

fn spawn_mesh_tasks(
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
					if let Some(pos)
						= world.bounded_add(player_chunk, IVec3::new(x, y, z)) {
						if !world.visible.contains(&pos) {
							let meshing_chunk = world.get_meshing_chunk(pos);
							let task = thread_pool.spawn(async move {
								MeshResult(pos, meshing_chunk.mesh())
							});
							commands.spawn().insert(MeshResultTask(task));
							world.visible.insert(pos);
						}
					}
				}
			}
		}
	}
}

fn handle_mesh_tasks(
	mut commands: Commands,
	mut tasks: Query<(Entity, &mut MeshResultTask)>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>
) {
	for (entity, mut task) in &mut tasks {
		if let Some(MeshResult(chunk_pos, mesh))
			= block_on(poll_once(&mut task.0)) {
			let mut material = StandardMaterial::from(Color::rgb(0., 0., 0.));
			material.perceptual_roughness = 0.9;
			commands.spawn_bundle(PbrBundle {
				mesh: meshes.add(mesh),
				material: materials.add(material),
				transform: Transform::from_translation(
					World::world_position(chunk_pos)
				).with_scale(Vec3::from_array([1.;3])),
					// we scale the mesh so that it is approximately one meter per
					// voxel.
				..Default::default()
			});
			commands.entity(entity).remove::<MeshResultTask>();
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::world::World;

	#[test]
	fn world_generation_succeeds() {
		World::generate([4, 4, 4]);
	}
}
