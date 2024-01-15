use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::render::{
    render_resource::WgpuFeatures,
    settings::{RenderCreation, WgpuSettings},
    RenderPlugin,
};

mod chunk;
mod directions;
mod meshing_chunk;
mod player_controller;
mod voxel;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
            }),
            WireframePlugin,
            player_controller::PlayerControllerPlugin,
            world::WorldPlugin,
        ))
        .insert_resource(WireframeConfig {
            global: true,
            default_color: Color::WHITE,
        })
        .run();
}
