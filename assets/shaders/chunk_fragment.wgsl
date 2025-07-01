// #import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
@group(2) @binding(1) var material_color_texture: texture_2d<f32>;
@group(2) @binding(2) var material_color_sampler: sampler;

// Shading map
const face_shading: array<f32, 6> = array(
    1.0, 0.5,   // top, bottom
    0.5, 0.8,   // right, left
    0.5, 0.8    // front, back
);

const ao_values: array<f32, 4> = array(
    0.1,
    0.25,
    0.5,
    1.0,
);

// Vertex shader input data mapping
struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv_coord: vec2<f32>,
    @location(2) vx_type: u32,
    @location(3) vx_ao: u32,
};

// Vertex shader output data mapping for passing to fragment shader
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv_coord: vec2<f32>,
    @location(1) hash_color: f32,
};

// The vertex shader itself
@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(vertex.position, 1.0),
    );
    out.uv_coord = vertex.uv_coord;
    out.hash_color = face_shading[vertex.vx_type] * ao_values[vertex.vx_ao];
    return out;
}

// The fragment shader itself
@fragment
fn fragment(
       input: VertexOutput,
) -> @location(0) vec4<f32> {
    return material_color * input.hash_color * textureSample(material_color_texture, material_color_sampler, input.uv_coord);
}
