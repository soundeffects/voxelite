use block_mesh::{Voxel as MeshableVoxel, VoxelVisibility, MergeVoxel};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Voxel {
	material: bool
}

pub const EMPTY: Voxel = Voxel { material: false };
pub const FULL: Voxel = Voxel { material: true };

impl MeshableVoxel for Voxel {
	fn get_visibility(&self) -> VoxelVisibility {
		if *self == EMPTY {
			VoxelVisibility::Empty
		} else {
			VoxelVisibility::Opaque
		}
	}
}

impl MergeVoxel for Voxel {
	type MergeValue = Self;

	fn merge_value(&self) -> Self::MergeValue {
		*self
	}
}
