use bevy::{
  prelude::*,
  core::CorePlugin,
  app::ScheduleRunnerPlugin,
  log::LogPlugin
};

mod commands;
use crate::commands::CommandPlugin;

/*
use ndshape::{ConstPow2Shape2u8, ConstPow2Shape3u16};
struct Voxel(bool);

const AIR: Voxel = Voxel(false);
const GROUND: Voxel = Voxel(true);

pub struct Chunk([Voxel; ConstPow2Shape3u16::SIZE as usize]);
pub struct Chunk32([Voxel; ConstPow2Shape3u32::SIZE as usize]);

const EMPTY: Chunk = Chunk([AIR; ConstPow2Shape3u16::SIZE as usize]);

struct Map {
  chunk_shape: ConstPow2Shape3u16,
  chunk_map: [Chunk; ConstPow2Shape2u8::SIZE as usize]
}
*/

fn main() {
  App::new()
    .add_plugin(CorePlugin::default())
    .add_plugin(ScheduleRunnerPlugin::default())
    .add_plugin(LogPlugin::default())
    .add_plugin(CommandPlugin)
    .run();
}
