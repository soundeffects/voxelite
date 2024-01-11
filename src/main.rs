use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy_infinite_grid::{InfiniteGridPlugin, InfiniteGridBundle};

mod chunk;
mod meshing_chunk; 
mod voxel;
mod world;
mod player_controller;
mod directions;

fn main() {
    println!("{}", 1 + 1);
  App::new()
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(WireframePlugin)
    .add_plugin(InfiniteGridPlugin)
    .add_plugin(player_controller::PlayerControllerPlugin)
    .add_plugin(world::WorldPlugin)
    .add_startup_system(setup)
    .run();
}

fn setup(
  mut commands: Commands,
  mut wireframe_config: ResMut<WireframeConfig>
) {
  wireframe_config.global = true;
  commands.spawn_bundle(InfiniteGridBundle::default());
}
