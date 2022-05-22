use bevy::prelude::{info, Color};

/*
 * VoxelMaterial
 *
 * Every voxel has a material id, which links to a material. This material
 * determines the voxel's characteristics, including name and color.
 * Characteristics are read only, accessible by getter functions.
 */
pub struct VoxelMaterial {
	name: & 'static str,
	color: Color,
}
impl VoxelMaterial {
	pub fn get_name(& self) -> & 'static str { self.name }
	pub fn get_color(& self) -> Color { self.color }
}

/*
 * VoxelMaterialRegistry
 *
 * The registry links material id's to voxel materials.
 */
pub struct VoxelMaterialRegistry {
	materials: Vec<VoxelMaterial>,
}
impl VoxelMaterialRegistry {
	//#[inline]
	pub fn get(& self, id: u16) -> Option<& VoxelMaterial> {
		self.materials.get(id as usize)
	}
	pub fn register(& mut self, material: VoxelMaterial) {
		self.materials.push(material);
		info!(
			"Registered material {} (ID: {})",
			material.get_name(),
			self.materials.len() - 1
		);
	}
	pub fn iter(& self) -> impl Iterator<Item = & VoxelMaterial> {
		self.materials.iter()
	}
}
impl Default for VoxelMaterialRegistry {
	fn default() -> Self {
		let mut registry = Self {
			materials: Default::default()
		};
		registry.register(VoxelMaterial {
			name: "Void",
			color: Color::BLACK
		});
		registry
	}
}
