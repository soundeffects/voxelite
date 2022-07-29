use block_mesh::{
	ndshape::{ConstShape, ConstShape3u32},
	greedy_quads,
	GreedyQuadsBuffer,
	RIGHT_HANDED_Y_UP_CONFIG
};
use bevy::{
	prelude::*,
	render::{
		mesh::{Indices, VertexAttributeValues},
		render_resource::PrimitiveTopology
	}
};
use rand::prelude::*;
use crate::voxel::{Voxel, EMPTY, FULL};

pub struct Chunk {
	voxel_data: Vec<Voxel>
}

pub type ChunkShape = ConstShape3u32<32, 32, 32>;
pub type ChunkShapeWithBorder = ConstShape3u32<34, 34, 34>;

impl Chunk {
	pub fn new() -> Self {
		Self {
			voxel_data: vec![EMPTY; ChunkShape::SIZE as usize]
		}
	}

	pub fn from(voxel_data: Vec<Voxel>) -> Self  {
		Self { voxel_data }
	}

	pub fn generate() -> Self {
		let mut voxel_data = vec![EMPTY; ChunkShape::SIZE as usize];
		for index in 0..voxel_data.len() {
			let coordinates = ChunkShape::delinearize(index as u32);
			let mut rng = thread_rng();
			if coordinates[1] < (6 + rng.gen_range(0..3)) {
				voxel_data[index] = FULL;
			}
		}
		Self { voxel_data }
	}

	pub fn mesh(
		&self,
		meshes: &mut Assets<Mesh>,
	) -> Handle<Mesh> {
		let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

		let mut buffer = GreedyQuadsBuffer::new(self.voxel_data.len());

		greedy_quads(
			&self.voxel_data,
			&ChunkShape {},
			[0; 3],
			[31; 3],
			&faces,
			&mut buffer
		);

		let num_indices = buffer.quads.num_quads() * 6;
		let num_vertices = buffer.quads.num_quads() * 4;

		let mut indices = Vec::with_capacity(num_indices);
		let mut positions = Vec::with_capacity(num_vertices);
		let mut normals = Vec::with_capacity(num_vertices);

		for (group, face) in buffer.quads.groups.into_iter()
			.zip(faces.into_iter()) {
			for quad in group.into_iter() {
				indices.extend_from_slice(
					&face.quad_mesh_indices(positions.len() as u32)
				);
				positions.extend_from_slice(&face.quad_mesh_positions(&quad, 1.0));
				normals.extend_from_slice(&face.quad_mesh_normals());
			}
		}

		let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

		mesh.insert_attribute(
			Mesh::ATTRIBUTE_POSITION,
			VertexAttributeValues::Float32x3(positions)
		);

		mesh.insert_attribute(
			Mesh::ATTRIBUTE_NORMAL,
			VertexAttributeValues::Float32x3(normals)
		);

		mesh.insert_attribute(
			Mesh::ATTRIBUTE_UV_0,
			VertexAttributeValues::Float32x2(vec![[0.0; 2]; num_vertices])
		);

		mesh.set_indices(Some(Indices::U32(indices.clone())));

		meshes.add(mesh)
	}
}
