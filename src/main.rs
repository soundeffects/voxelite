use bevy::prelude::{Plugin, App};

/*use ndshape::{ConstPow2Shape2u8, ConstPow2Shape3u16};
struct Voxel(bool);

const AIR: Voxel = Voxel(false);
const GROUND: Voxel = Voxel(true);

pub struct Chunk([Voxel; ConstPow2Shape3u16::SIZE as usize]);
//pub struct Chunk32([Voxel; ConstPow2Shape3u32::SIZE as usize]);

const EMPTY: Chunk = Chunk([AIR; ConstPow2Shape3u16::SIZE as usize]);

struct Map {
  chunk_shape: ConstPow2Shape3u16,
  chunk_map: [Chunk; ConstPow2Shape2u8::SIZE as usize]
}*/

fn greet() {
  println!("Hello world!");
}

pub struct GreetPlugin;

impl Plugin for GreetPlugin {
  fn build(&self, app: &mut App) {
    app.add_startup_system(greet);
  }
}

fn main() {
  App::new()
    .add_plugin(GreetPlugin)
    .run();
}

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
  }
}
