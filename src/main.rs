use bevy::{
  pbr::wireframe::{WireframeConfig, WireframePlugin},
  prelude::*
};

mod chunk;
mod voxel;
mod world;
mod player_controller;

fn main() {
  App::new()
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(WireframePlugin)
    .add_plugin(player_controller::PlayerControllerPlugin)
    .add_plugin(world::WorldPlugin)
    .add_startup_system(setup)
    .run();
}

fn setup(
  mut wireframe_config: ResMut<WireframeConfig>,
) {
  wireframe_config.global = true;
}
