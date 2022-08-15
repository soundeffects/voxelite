use ndshape::{ConstShape, ConstPow2Shape3u32};
use block_mesh::*;
use bevy::prelude::*;
use rand::prelude::*;
use crate::voxel::{Voxel, EMPTY, FULL};

#[derive(Clone)]
pub struct Chunk {
	pub voxel_data: [Voxel; ChunkShape::USIZE]
}

pub const CHUNK_DIM: u32 = 32;
pub const CHUNK_EXP: u32 = 5;
pub type ChunkShape = ConstPow2Shape3u32<CHUNK_EXP, CHUNK_EXP, CHUNK_EXP>;

impl Chunk {
	pub fn empty() -> Self {
		Self {
			voxel_data: [EMPTY; ChunkShape::USIZE]
		}
	}

	pub fn generate(
		_chunk_pos: UVec3,
		_generator: impl Fn(UVec3) -> Voxel
	) -> Self {
		let mut voxel_data = [EMPTY; ChunkShape::USIZE];
		for index in 0..ChunkShape::SIZE {
			let coordinates
				= ChunkShape::delinearize(index);
			let mut rng = thread_rng();
			if coordinates[1] < (6 + rng.gen_range(0..3)) {
				voxel_data[index as usize] = FULL;
			}
		}
		Self { voxel_data }
	}
}
