// #import bevy_pbr::forward_io::VertexOutput
#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}

@group(2) @binding(0) var<uniform> material_color: vec4<f32>;
// @group(2) @binding(1) var material_color_texture: texture_2d<f32>;
// @group(2) @binding(2) var material_color_sampler: sampler;

// Shading map
const face_shading: array<f32, 6> = array(
    1.0, 0.5,   // top, bottom
    0.5, 0.8,   // right, left
    0.5, 0.8    // front, back
);

// Vertex shader input data mapping
struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) vx_type: u32,
    @location(2) vx_id: f32,
};

// Vertex shader output data mapping
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) hash_color: f32,
};

// The vertex shader itself
@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(vertex.instance_index),
        vec4<f32>(vertex.position, 1.0),
    );
    out.hash_color = face_shading[vertex.vx_type];
    return out;
}

// Fragment shader input data mapping
struct FragmentInput {
    @location(0) blend_color: f32,
};

// The fragment shader itself
@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    return material_color * input.blend_color;
}

// // Simulates nearest-neighbor interpolation on a linearly-interpolated texture
// fn texture_nearest(tex: texture_2d<f32>, samp: sampler, uv: vec2<f32>) -> vec4<f32> {
//     let tex_res = vec2<f32>(textureDimensions(material_color_texture));
//     return textureSample(tex, samp, (floor(uv * tex_res) + 0.5) / tex_res);
// }

// @fragment
// fn fragment(
//        mesh: VertexOutput,
// ) -> @location(0) vec4<f32> {
//     return material_color;
//     // return material_color * texture_nearest(material_color_texture, material_color_sampler, mesh.uv);
// }
