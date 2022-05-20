#[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
pub struct Voxel(pub u16);

impl Default for Voxel {
	fn default() -> Self {
		Voxel(0);
	}
}
