use std::iter::zip;
use bevy::prelude::*;
use ndshape::{ConstShape, ConstShape3u32};
use ndcopy::copy3;
use block_mesh::*;
use bevy::render::mesh::{Indices, VertexAttributeValues};
use bevy::render::render_resource::PrimitiveTopology;
use crate::voxel::{Voxel, EMPTY};
use crate::chunk::{Chunk, ChunkShape, CHUNK_DIM};

const MESH_CHUNK_DIM: u32 = CHUNK_DIM + 2;
type MeshChunkShape
	= ConstShape3u32<MESH_CHUNK_DIM, MESH_CHUNK_DIM, MESH_CHUNK_DIM>;

pub struct MeshingChunk {
	samples: [Voxel; MeshChunkShape::USIZE]
}

impl MeshingChunk {
	// these correspond to each direction in directions.rs
	pub const COPY_SHAPES: [([u32; 3], [u32; 3], [u32; 3]); 7] = [
		(ChunkShape::ARRAY, [0, 0, 0], [1, 1, 1]),
		([1, 32, 32], [31, 0, 0], [33, 1, 1]),
		([1, 32, 32], [0, 0, 0], [0, 1, 1]),
		([32, 1, 32], [0, 31, 0], [1, 33, 1]),
		([32, 1, 32], [0, 0, 0], [1, 0, 1]),
		([32, 32, 1], [0, 0, 31], [1, 1, 33]),
		([32, 32, 1], [0, 0, 0], [1, 1, 0])
	];

	pub fn new(chunks: [Option<&Chunk>; 7]) -> Self {
		let mut mesh_chunk = Self { samples: [EMPTY; MeshChunkShape::USIZE] };
		let iter = zip(chunks, Self::COPY_SHAPES);
		for (chunk, (copy_shape, chunk_offset, meshing_offset)) in iter {
			if let Some(valid_chunk) = chunk {
				copy3(
					copy_shape,
					&valid_chunk.voxel_data,
					&ChunkShape {},
					chunk_offset,
					&mut mesh_chunk.samples,
					&MeshChunkShape {},
					meshing_offset
				);
			}
		}
		mesh_chunk
	}

	pub fn mesh(&self) -> Mesh {
		let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

		let mut buffer = GreedyQuadsBuffer::new(self.samples.len());

		greedy_quads(
			&self.samples,
			&MeshChunkShape {},
			[0; 3],
			[33; 3],
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

		mesh
	}
}
