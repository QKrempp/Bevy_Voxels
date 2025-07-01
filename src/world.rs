use core::f32;

use bevy::image::{ImageLoaderSettings, ImageSampler};
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy::render::mesh::MeshVertexAttribute;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, VertexFormat};
use chunk::{CubeTypes, VxWorldCoord};
use noisy_bevy::simplex_noise_2d;

mod chunk;

use super::{
    CHUNK_AREA, CHUNK_SIZE, CHUNK_VOLUME, WORLD_AREA, WORLD_D, WORLD_H, WORLD_VOL, WORLD_W,
};

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
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
            ATTRIBUTE_VX_TYPE.at_shader_location(2),
            ATTRIBUTE_VX_AO.at_shader_location(3),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

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
                                voxels[VxWorldCoord::new((c_x, c_y, c_z), (x, y, z)).get_id()] =
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
                color_texture: Some(asset_server.load_with_settings(
                    "textures.png",
                    |settings: &mut ImageLoaderSettings| {
                        settings.sampler = ImageSampler::nearest();
                    },
                )),
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
}
