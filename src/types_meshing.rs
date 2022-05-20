use block_mesh::{MergeVoxel, Voxel as MeshVoxel}

// use types.rs

impl MeshVoxel for Voxel {
	//[#inline]
	fn get_visibility(& self) -> block_mesh::VoxelVisibility {
		match self.0 {
			0 => block_mesh::VoxelVisibility::Empty,
			_ => block_mesh::VoxelVisibility::Opaque
		}
	}
}

impl MergeVoxel for Voxel {
	type MergeValue = u8;

	// #[inline]
	fn merge_value(& self) -> Self::MergeValue {
		self.0
	}
}

pub trait MaterialVoxel: MergeVoxel + MeshVoxel {
	fn material_id(& self) -> u16;
}

impl MaterialVoxel for Voxel {
	fn material_id(& self) -> u16 {
		self.0
	}
}
