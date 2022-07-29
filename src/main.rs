use bevy::{
  pbr::wireframe::{WireframeConfig, WireframePlugin},
  prelude::*
};

mod chunk;
mod voxel;
//mod world;
mod player_controller;

fn main() {
  App::new()
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(WireframePlugin)
    .add_plugin(player_controller::PlayerControllerPlugin)
    .add_startup_system(setup)
    .run();
}

fn setup(
  mut commands: Commands,
  mut wireframe_config: ResMut<WireframeConfig>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut meshes: ResMut<Assets<Mesh>>
) {
  wireframe_config.global = true;

  let chunk = chunk::Chunk::generate();

  let mesh = chunk.mesh(&mut meshes);

  let mut material = StandardMaterial::from(Color::rgb(0.0, 0.0, 0.0));
  material.perceptual_roughness = 0.9;

  commands.spawn_bundle(PbrBundle {
    mesh,
    material: materials.add(material),
    transform: Transform::from_translation(Vec3::new(8.0, -16.0, -16.0)),
    ..Default::default()
  });
}
