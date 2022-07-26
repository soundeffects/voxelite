use block_mesh::{
  ilattice::glam::Vec3A,
  ndshape::{ConstShape, ConstShape3u32},
  greedy_quads,
  GreedyQuadsBuffer,
  MergeVoxel,
  Voxel,
  VoxelVisibility,
  RIGHT_HANDED_Y_UP_CONFIG
};

use bevy::{
  pbr::wireframe::{WireframeConfig, WireframePlugin},
  prelude::*,
  render::{
    mesh::{Indices, VertexAttributeValues},
    render_resource::PrimitiveTopology
  }
};

fn main() {
  App::new()
    .insert_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(WireframePlugin)
    .add_plugin(bevy_flycam::PlayerPlugin)
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

  commands.spawn_bundle(PointLightBundle {
    transform: Transform::from_translation(Vec3::new(25.0, 25.0, 25.0)),
    point_light: PointLight {
      range: 200.0,
      intensity: 8000.0,
      ..Default::default()
    },
    ..Default::default()
  });

  let mesh = generate_greedy_mesh(&mut meshes, |p| sphere(0.9, p));

  let mut material = StandardMaterial::from(Color::rgb(0.0, 0.0, 0.0));
  material.perceptual_roughness = 0.9;

  commands.spawn_bundle(PbrBundle {
    mesh,
    material: materials.add(material),
    transform: Transform::from_translation(Vec3::new(8.0, -16.0, -16.0)),
    ..Default::default()
  });
}

fn generate_greedy_mesh(
  meshes: &mut Assets<Mesh>,
  sdf: impl Fn(Vec3A) -> BoolVoxel
) -> Handle<Mesh> {
  type SampleShape = ConstShape3u32<34, 34, 34>;

  let mut samples = [EMPTY; SampleShape::SIZE as usize];
  
  for i in 0u32..(SampleShape::SIZE) {
    let p = into_domain(32, SampleShape::delinearize(i));
    samples[i as usize] = sdf(p);
  }

  let faces = RIGHT_HANDED_Y_UP_CONFIG.faces;

  let mut buffer = GreedyQuadsBuffer::new(samples.len());

  greedy_quads(
    &samples,
    &SampleShape {},
    [0; 3],
    [33; 3],
    &faces,
    &mut buffer
  );

  let num_indices = buffer.quads.num_quads() * 6;
  let num_vertices = buffer.quads.num_quads() * 4;

  let mut indices = Vec::with_capacity(num_indices);
  let mut positions = Vec::with_capacity(num_vertices);
  let mut normals = Vec::with_capacity(num_vertices);

  for (group, face) in buffer.quads.groups.into_iter()
    .zip(faces.into_iter()) {
    for quad in group.into_iter() {
      indices.extend_from_slice(
        &face.quad_mesh_indices(positions.len() as u32)
      );
      positions.extend_from_slice(&face.quad_mesh_positions(&quad, 1.0));
      normals.extend_from_slice(&face.quad_mesh_normals());
    }
  }

  let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

  mesh.insert_attribute(
    Mesh::ATTRIBUTE_POSITION,
    VertexAttributeValues::Float32x3(positions)
  );

  mesh.insert_attribute(
    Mesh::ATTRIBUTE_NORMAL,
    VertexAttributeValues::Float32x3(normals)
  );

  mesh.insert_attribute(
    Mesh::ATTRIBUTE_UV_0,
    VertexAttributeValues::Float32x2(vec![[0.0; 2]; num_vertices])
  );

  mesh.set_indices(Some(Indices::U32(indices.clone())));

  meshes.add(mesh)
}

fn into_domain(array_dim: u32, [x, y, z]: [u32; 3]) -> Vec3A {
  (2.0 / array_dim as f32) * Vec3A::new(x as f32, y as f32, z as f32)
}

fn sphere(radius: f32, p: Vec3A) -> BoolVoxel {
  BoolVoxel(p.length() < radius)
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct BoolVoxel(bool);

const EMPTY: BoolVoxel = BoolVoxel(false);
//const FULL: BoolVoxel = BoolVoxel(true);

impl Voxel for BoolVoxel {
  fn get_visibility(&self) -> VoxelVisibility {
    if *self == EMPTY {
      VoxelVisibility::Empty
    } else {
      VoxelVisibility::Opaque
    }
  }
}

impl MergeVoxel for BoolVoxel {
  type MergeValue = Self;

  fn merge_value(&self) -> Self::MergeValue {
    *self
  }
}

//type ChunkShape = ConstShape3u32<18, 18, 18>;

/*
use material::VoxelMaterialRegistry;
pub struct VoxelServerPlugin;
impl Plugin for VoxelServerPlugin {
	fn build(& self, app: & mut App) {
		app.init_resource::<VoxelMaterialRegistry>();
		/*
		app.insert_resource(VoxelMap::<Voxel, ChunkShape>::new(ChunkShape {}))
			.add_plugin(chunks::VoxelChunkingPlugin)
			.add_plugin(meshing::VoxelMeshingPlugin)
			.add_plugin(terrain::VoxelTerrainPlugin)
			.add_plugin(super::render::VoxelRenderPipelinePlugin)
			.add_plugin(super::material::VoxelMaterialPlugin)
			.add_plugin(materials::VoxelBaseMaterialsPlugin)
			.add_plugin(player::VoxelWorldPlayerControllerPlugin);
		*/
	}
}

use ndshape::{ConstPow2Shape2u8, ConstPow2Shape3u16};

pub struct Chunk([Voxel; ConstPow2Shape3u16::SIZE as usize]);
pub struct Chunk32([Voxel; ConstPow2Shape3u32::SIZE as usize]);

const EMPTY: Chunk = Chunk([AIR; ConstPow2Shape3u16::SIZE as usize]);

struct Map {
  chunk_shape: ConstPow2Shape3u16,
  chunk_map: [Chunk; ConstPow2Shape2u8::SIZE as usize]
}
*/
