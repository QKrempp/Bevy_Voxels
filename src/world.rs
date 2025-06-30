use core::f32;

use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::mesh::MeshVertexAttribute;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, VertexFormat};
use chunk::CubeTypes;
use noisy_bevy::simplex_noise_2d;

mod chunk;

const ATTRIBUTE_VX_TYPE: MeshVertexAttribute =
    MeshVertexAttribute::new("VxType", 10000, VertexFormat::Uint32);
const ATTRIBUTE_VX_AO: MeshVertexAttribute =
    MeshVertexAttribute::new("VxAo", 10001, VertexFormat::Uint32);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ChunkMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl Material for ChunkMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/chunk_fragment.wgsl".into()
    }

    fn vertex_shader() -> ShaderRef {
        "shaders/chunk_fragment.wgsl".into()
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            ATTRIBUTE_VX_TYPE.at_shader_location(1),
            ATTRIBUTE_VX_AO.at_shader_location(2),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

pub const WORLD_W: usize = 5;
pub const WORLD_H: usize = 2;
pub const WORLD_D: usize = 5;
pub const WORLD_AREA: usize = WORLD_W * WORLD_D;
pub const WORLD_VOL: usize = WORLD_AREA * WORLD_H;
pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_AREA: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_AREA;

struct VxWorld {
    chunks: Vec<chunk::VxChunkMesh>,
    _voxels: Vec<chunk::CubeTypes>,
}

impl VxWorld {
    fn new() -> Self {
        let _voxels = map_generation();
        let chunks = (0..WORLD_VOL)
            .into_iter()
            .map(|i| {
                chunk::VxChunkMesh::new(
                    (
                        (i % WORLD_W),
                        (i / WORLD_AREA),
                        ((i % WORLD_AREA) / WORLD_W),
                    ),
                    &_voxels,
                )
            })
            .collect();
        Self { chunks, _voxels }
    }
}

fn coord_to_usize(chunk_coord: (usize, usize, usize), cube_coord: (usize, usize, usize)) -> usize {
    (chunk_coord.0 + chunk_coord.2 * WORLD_W + chunk_coord.1 * WORLD_AREA) * CHUNK_VOLUME
        + (cube_coord.0 + cube_coord.2 * CHUNK_SIZE + cube_coord.1 * CHUNK_AREA)
}

fn map_generation() -> Vec<chunk::CubeTypes> {
    let mut voxels = vec![CubeTypes::Empty; WORLD_VOL * CHUNK_VOLUME];
    for c_x in 0..WORLD_W {
        for c_y in 0..WORLD_H {
            for c_z in 0..WORLD_D {
                for x in 0..CHUNK_SIZE {
                    for z in 0..CHUNK_SIZE {
                        let height = (CHUNK_SIZE as f32)
                            * (simplex_noise_2d(
                                0.01 * Vec2::new(
                                    (x + c_x * CHUNK_SIZE) as f32,
                                    (z + c_z * CHUNK_SIZE) as f32,
                                ),
                            ) + 1.0);
                        for y in 0..CHUNK_SIZE {
                            if ((c_y * CHUNK_SIZE + y) as f32) < height {
                                voxels[coord_to_usize((c_x, c_y, c_z), (x, y, z))] =
                                    CubeTypes::Dirt;
                            }
                        }
                    }
                }
            }
        }
    }
    voxels
}

pub fn spawn_world_model(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let my_world = VxWorld::new();
    // Custom chunk
    for chunk in my_world.chunks {
        commands.spawn((
            Mesh3d(meshes.add(chunk.mesh)),
            MeshMaterial3d(materials.add(ChunkMaterial {
                color: LinearRgba::WHITE,
                color_texture: Some(asset_server.load("textures.png")),
            })),
            Transform::from_translation(
                (CHUNK_SIZE as f32)
                    * Vec3::from((
                        chunk.coord.0 as f32,
                        chunk.coord.1 as f32,
                        chunk.coord.2 as f32,
                    )),
            ),
        ));
    }

    //    commands.spawn((
    //        Mesh3d(meshes.add(Cuboid::default())),
    //        MeshMaterial3d(materials.add(ChunkMaterial {
    //            color: LinearRgba::BLUE,
    //            color_texture: Some(asset_server.load("textures.png")),
    //            alpha_mode: AlphaMode::Blend,
    //        })),
    //    ));

    // light
    //    commands.spawn((
    //        DirectionalLight {
    //            illuminance: 15000.0,
    //            shadows_enabled: true,
    //            ..default()
    //        },
    //        Transform::from_rotation(Quat::from_euler(EulerRot::ZYX, 0.0, PI * -0.15, PI * -0.15)),
    //        CascadeShadowConfigBuilder {
    //            maximum_distance: 3.0,
    //            first_cascade_far_bound: 0.9,
    //            ..default()
    //        }
    //        .build(),
    //    ));
}
