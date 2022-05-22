use bevy::{
	prelude::*,
	input::mouse::MouseMotion
};
use bevy_voxel_server::{
	VoxelServerPlugin,
	material::{
		VoxelMaterial,
		VoxelMaterialRegistry
	}
};

// This example is organized into plugins
fn main() {
	App::default()
		.add_plugins(DefaultPlugins)
		.add_plugin(VoxelServerPlugin)
		.add_plugin(MaterialPlugin)
		.add_plugin(bevy_flycam::PlayerPlugin)
		.run();
}

// set up a set of materials (block types) for our voxel world
pub struct MaterialPlugin;
impl Plugin for MaterialPlugin {
	fn build(& self, app: & mut App) {
		let mut registry = app.get_resource_mut::<VoxelMaterialRegistry>()
			.unwrap();
		registry.register(VoxelMaterial {
			name: "Dirt",
			color: Color::BEIGE
		});
		registry.register(VoxelMaterial {
			name: "Grass",
			color: Color::LIME_GREEN
		});
		registry.register(VoxelMaterial {
			name: "Stone",
			color: Color::GRAY
		});
	}
}
