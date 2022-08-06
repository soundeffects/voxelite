use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::utils::HashSet;
use bevy::prelude::*;
use futures_lite::future::{block_on, poll_once};
use ndshape::{Shape, RuntimeShape};
use crate::chunk::{Chunk, CHUNK_DIM};
use crate::player_controller::{PlayerController, PlayerSettings};
use crate::voxel::EMPTY;

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
	pub fn chunk_pos(world_position: Vec3) -> UVec3 {
		(world_position / CHUNK_DIM as f32).as_uvec3()
	}

	pub fn world_position(chunk_pos: UVec3) -> Vec3 {
		(chunk_pos * CHUNK_DIM).as_vec3()
	}

	pub fn out_of_bounds(&self, chunk_pos: IVec3) -> bool {
		let bounds = self.shape.as_array();
		let out = |coord: i32, bound_index: usize| {
			coord < 0 || coord >= bounds[bound_index] as i32
		};
		out(chunk_pos.x, 0) || out(chunk_pos.y, 1) || out(chunk_pos.z, 2)
	}

	pub fn generate(dimensions: [u32; 3]) -> Self {
		let shape = RuntimeShape::<u32, 3>::new(dimensions);
		let mut chunks = Vec::<Chunk>::new();

		for index in 0..shape.size() {
			info!("{} out of {} generated", index, shape.size());
			chunks.push(
				Chunk::generate(
					UVec3::from_array(shape.delinearize(index)),
					|_| { EMPTY }
				)
			);
		}

		World {
			shape,
			chunks,
			visible: HashSet::new()
		}
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
					let unbound_pos = player_chunk.as_ivec3() + IVec3::new(x, y, z);
					if !world.out_of_bounds(unbound_pos) {
						let chunk_pos = unbound_pos.as_uvec3();
						if !world.visible.contains(&chunk_pos) {
							let index = world.shape.linearize(chunk_pos.to_array());
							let chunk = world.chunks[index as usize].clone();
							let task = thread_pool.spawn(async move {
								MeshResult(chunk_pos, chunk.mesh())
							});
							commands.spawn().insert(MeshResultTask(task));
							world.visible.insert(chunk_pos);
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
				).with_scale(Vec3::new(1.063, 1.063, 1.063)),
				..Default::default()
			});
			commands.entity(entity).remove::<MeshResultTask>();
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::world::World;
	use bevy::math::IVec3;

	#[test]
	fn world_generation_succeeds() {
		World::generate([4, 4, 4]);
	}

	#[test]
	fn world_bounding_confirmation() {
		let world = World::generate([4, 4, 4]);
		assert!(world.out_of_bounds(IVec3::new(0, -1, 0)));
		assert!(world.out_of_bounds(IVec3::new(5, 4, 4)));
	}
}
