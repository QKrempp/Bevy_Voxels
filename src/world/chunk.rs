use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::PrimitiveTopology};

use super::{
    ATTRIBUTE_VX_ID, ATTRIBUTE_VX_TYPE, CHUNK_AREA, CHUNK_SIZE, CHUNK_VOLUME, WORLD_AREA, WORLD_D,
    WORLD_H, WORLD_W,
};

#[derive(Debug, Clone, PartialEq)]
pub enum CubeTypes {
    Empty,
    Dirt,
    // Stone,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FaceType {
    Top,
    Bottom,
    Right,
    Left,
    Back,
    Front,
}

impl Into<(i8, i8, i8)> for FaceType {
    fn into(self) -> (i8, i8, i8) {
        match self {
            FaceType::Top => (0, 1, 0),
            FaceType::Bottom => (0, -1, 0),
            FaceType::Right => (1, 0, 0),
            FaceType::Left => (-1, 0, 0),
            FaceType::Back => (0, 0, 1),
            FaceType::Front => (0, 0, -1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VxChunkMesh {
    pub mesh: Mesh,
    pub coord: (usize, usize, usize),
}

impl VxChunkMesh {
    pub fn new(coord: (usize, usize, usize), voxels: &[CubeTypes]) -> Self {
        let (vertices_coord, uv_coord, vertices_normal, vertices_order, vertices_type, vertices_id) =
            build_mesh(voxels, &coord);
        println!("Created chunk with coord {:?}", coord);
        Self {
            mesh: Mesh::new(
                PrimitiveTopology::TriangleList,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
            )
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices_coord.clone())
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv_coord.clone())
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vertices_normal.clone())
            .with_inserted_attribute(ATTRIBUTE_VX_TYPE, vertices_type.clone())
            .with_inserted_attribute(ATTRIBUTE_VX_ID, vertices_id.clone())
            .with_inserted_indices(bevy::render::mesh::Indices::U32(vertices_order.clone())),
            coord,
        }
    }
}

fn get_cube_type(
    voxels: &[CubeTypes],
    chunk_coord: &(usize, usize, usize),
    cube_coord: &(usize, usize, usize),
) -> CubeTypes {
    // let index = chunk_coord.dot(Vec3::new(1.0, WORLD_AREA as f32, WORLD_W as f32)) as usize
    //     * CHUNK_VOLUME
    //     + cube_coord.dot(Vec3::new(1.0, CHUNK_AREA as f32, CHUNK_SIZE as f32)) as usize;

    match get_id(chunk_coord, cube_coord, &(0, 0, 0)) {
        Some(index) => voxels[index].clone(),
        None => CubeTypes::Empty,
    }
}

fn is_void(
    voxels: &[CubeTypes],
    chunk_coord: &(usize, usize, usize),
    cube_coord: &(usize, usize, usize),
    direction: &(i8, i8, i8),
) -> bool {
    match get_id(chunk_coord, cube_coord, direction) {
        Some(index) => voxels[index] == CubeTypes::Empty,
        None => true,
    }
}

// fn _is_void(voxels: &[CubeTypes], chunk_coord: &Vec3, cube_coord: &Vec3) {}

fn get_id(
    chunk_coord: &(usize, usize, usize),
    cube_coord: &(usize, usize, usize),
    direction: &(i8, i8, i8),
) -> Option<usize> {
    let mut index = 0;

    // Index due to X component
    if direction.0 >= 0 {
        let direction_usize = direction.0 as usize;
        if cube_coord.0 + direction_usize >= CHUNK_SIZE {
            if chunk_coord.0 + 1 >= WORLD_W {
                return None;
            } else {
                index += (chunk_coord.0 + 1) * CHUNK_VOLUME
                    + (cube_coord.0 + direction_usize - CHUNK_SIZE)
            }
        } else {
            index += chunk_coord.0 * CHUNK_VOLUME + cube_coord.0 + direction_usize
        }
    } else {
        let direction_usize = -direction.0 as usize;
        if cube_coord.0 < direction_usize {
            if chunk_coord.0 == 0 {
                return None;
            } else {
                index += (chunk_coord.0 - 1) * CHUNK_VOLUME
                    + (CHUNK_SIZE - (direction_usize - cube_coord.0));
            }
        } else {
            index += chunk_coord.0 * CHUNK_VOLUME + cube_coord.0 - direction_usize;
        }
    }

    // Index due to Y component
    if direction.1 >= 0 {
        let direction_usize = direction.1 as usize;
        if cube_coord.1 + direction_usize >= CHUNK_SIZE {
            if chunk_coord.1 + 1 >= WORLD_H {
                return None;
            } else {
                index += (chunk_coord.1 + 1) * WORLD_AREA * CHUNK_VOLUME
                    + (cube_coord.1 + direction_usize - CHUNK_SIZE) * CHUNK_AREA;
            }
        } else {
            index += chunk_coord.1 * WORLD_AREA * CHUNK_VOLUME
                + (cube_coord.1 + direction_usize) * CHUNK_AREA;
        }
    } else {
        let direction_usize = -direction.1 as usize;
        if cube_coord.1 < direction_usize {
            if chunk_coord.1 == 0 {
                return None;
            } else {
                index += (chunk_coord.1 - 1) * WORLD_AREA * CHUNK_VOLUME
                    + (CHUNK_SIZE - (direction_usize - cube_coord.1)) * CHUNK_AREA;
            }
        } else {
            index += chunk_coord.1 * WORLD_AREA * CHUNK_VOLUME
                + (cube_coord.1 - direction_usize) * CHUNK_AREA;
        }
    }

    // Index due to Z component
    if direction.2 >= 0 {
        let direction_usize = direction.2 as usize;
        if cube_coord.2 + direction_usize >= CHUNK_SIZE {
            if chunk_coord.2 + 1 >= WORLD_D {
                return None;
            } else {
                index += (chunk_coord.2 + 1) * WORLD_W * CHUNK_VOLUME
                    + (cube_coord.2 + direction_usize - CHUNK_SIZE) * CHUNK_SIZE;
            }
        } else {
            index += chunk_coord.2 * WORLD_W * CHUNK_VOLUME
                + (cube_coord.2 + direction_usize) * CHUNK_SIZE;
        }
    } else {
        let direction_usize = -direction.2 as usize;
        if cube_coord.2 < direction_usize {
            if chunk_coord.2 == 0 {
                return None;
            } else {
                index += (chunk_coord.2 - 1) * WORLD_W * CHUNK_VOLUME
                    + (CHUNK_SIZE - (direction_usize - cube_coord.2)) * CHUNK_SIZE;
            }
        } else {
            index += chunk_coord.2 * WORLD_W * CHUNK_VOLUME
                + (cube_coord.2 - direction_usize) * CHUNK_SIZE;
        }
    }
    if index == 1638400 {
        dbg!(chunk_coord, cube_coord, direction);
    }
    Some(index)
}

// fn is_void(
//     voxels: &[CubeTypes],
//     chunk_coord: &Vec3,
//     cube_coord: &Vec3,
//     face_type: &FaceType,
// ) -> bool {
//     match face_type {
//         FaceType::Top => {
//             if cube_coord.y as usize == CHUNK_SIZE - 1 {
//                 if chunk_coord.y as usize == WORLD_H - 1 {
//                     true
//                 } else {
//                     get_cube_type(voxels, &(chunk_coord + Vec3::Y), &(cube_coord.with_y(0.0)))
//                         == CubeTypes::Empty
//                 }
//             } else {
//                 get_cube_type(voxels, chunk_coord, &(cube_coord + Vec3::Y)) == CubeTypes::Empty
//             }
//         }
//         FaceType::Bottom => {
//             if cube_coord.y as usize == 0 {
//                 if chunk_coord.y as usize == 0 {
//                     true
//                 } else {
//                     get_cube_type(
//                         voxels,
//                         &(chunk_coord - Vec3::Y),
//                         &(cube_coord.with_y((CHUNK_SIZE - 1) as f32)),
//                     ) == CubeTypes::Empty
//                 }
//             } else {
//                 get_cube_type(voxels, chunk_coord, &(cube_coord - Vec3::Y)) == CubeTypes::Empty
//             }
//         }
//         FaceType::Right => {
//             if cube_coord.x as usize == CHUNK_SIZE - 1 {
//                 if chunk_coord.x as usize == WORLD_W - 1 {
//                     true
//                 } else {
//                     get_cube_type(voxels, &(chunk_coord + Vec3::X), &(cube_coord.with_x(0.0)))
//                         == CubeTypes::Empty
//                 }
//             } else {
//                 get_cube_type(voxels, chunk_coord, &(cube_coord + Vec3::X)) == CubeTypes::Empty
//             }
//         }
//         FaceType::Left => {
//             if cube_coord.x as usize == 0 {
//                 if chunk_coord.x as usize == 0 {
//                     true
//                 } else {
//                     get_cube_type(
//                         voxels,
//                         &(chunk_coord - Vec3::X),
//                         &(cube_coord.with_x((CHUNK_SIZE - 1) as f32)),
//                     ) == CubeTypes::Empty
//                 }
//             } else {
//                 get_cube_type(voxels, chunk_coord, &(cube_coord - Vec3::X)) == CubeTypes::Empty
//             }
//         }
//         FaceType::Back => {
//             if cube_coord.z as usize == CHUNK_SIZE - 1 {
//                 if chunk_coord.z as usize == WORLD_D - 1 {
//                     true
//                 } else {
//                     get_cube_type(voxels, &(chunk_coord + Vec3::Z), &(cube_coord.with_z(0.0)))
//                         == CubeTypes::Empty
//                 }
//             } else {
//                 get_cube_type(voxels, chunk_coord, &(cube_coord + Vec3::Z)) == CubeTypes::Empty
//             }
//         }
//         FaceType::Front => {
//             if cube_coord.z as usize == 0 {
//                 if chunk_coord.z as usize == 0 {
//                     true
//                 } else {
//                     get_cube_type(
//                         voxels,
//                         &(chunk_coord - Vec3::Z),
//                         &(cube_coord.with_z((CHUNK_SIZE - 1) as f32)),
//                     ) == CubeTypes::Empty
//                 }
//             } else {
//                 get_cube_type(voxels, chunk_coord, &(cube_coord - Vec3::Z)) == CubeTypes::Empty
//             }
//         }
//     }
// }

fn map_texture(uv_coord: &mut Vec<Vec2>, coord_x: u32, coord_y: u32) {
    if coord_x < 32 && coord_y < 32 {
        let (x, y) = (coord_x as f32, coord_y as f32);
        uv_coord.push(Vec2::new(y / 32.0, (x + 1.0) / 32.0));
        uv_coord.push(Vec2::new(y / 32.0, x / 32.0));
        uv_coord.push(Vec2::new((y + 1.0) / 32.0, x / 32.0));
        uv_coord.push(Vec2::new((y + 1.0) / 32.0, (x + 1.0) / 32.0));
    }
}

fn add_face(
    vertices_coord: &mut Vec<Vec3>,
    uv_coord: &mut Vec<Vec2>,
    vertices_normal: &mut Vec<Vec3>,
    vertices_order: &mut Vec<u32>,
    vertices_type: &mut Vec<u32>,
    vertices_id: &mut Vec<f32>,
    face_type: FaceType,
    cube_type: &CubeTypes,
    cube_center: Vec3,
) {
    let v1: Vec3;
    let v2: Vec3;
    let v3: Vec3;
    let v4: Vec3;
    let normal: Vec3;
    let v_type: u32;
    let v_id = cube_center.element_sum();

    match face_type {
        FaceType::Top => {
            v1 = Vec3::new(-0.5, 0.5, -0.5);
            v2 = Vec3::new(0.5, 0.5, -0.5);
            v3 = Vec3::new(0.5, 0.5, 0.5);
            v4 = Vec3::new(-0.5, 0.5, 0.5);
            normal = Vec3::Y;
            v_type = 0;
        }
        FaceType::Bottom => {
            v1 = Vec3::new(-0.5, -0.5, -0.5);
            v2 = Vec3::new(-0.5, -0.5, 0.5);
            v3 = Vec3::new(0.5, -0.5, 0.5);
            v4 = Vec3::new(0.5, -0.5, -0.5);
            normal = Vec3::NEG_Y;
            v_type = 1;
        }
        FaceType::Right => {
            v1 = Vec3::new(0.5, -0.5, -0.5);
            v2 = Vec3::new(0.5, -0.5, 0.5);
            v3 = Vec3::new(0.5, 0.5, 0.5);
            v4 = Vec3::new(0.5, 0.5, -0.5);
            normal = Vec3::X;
            v_type = 2;
        }

        FaceType::Left => {
            v1 = Vec3::new(-0.5, -0.5, -0.5);
            v2 = Vec3::new(-0.5, 0.5, -0.5);
            v3 = Vec3::new(-0.5, 0.5, 0.5);
            v4 = Vec3::new(-0.5, -0.5, 0.5);
            normal = Vec3::NEG_X;
            v_type = 3
        }

        FaceType::Back => {
            v1 = Vec3::new(-0.5, -0.5, 0.5);
            v2 = Vec3::new(-0.5, 0.5, 0.5);
            v3 = Vec3::new(0.5, 0.5, 0.5);
            v4 = Vec3::new(0.5, -0.5, 0.5);
            normal = Vec3::Z;
            v_type = 4;
        }

        FaceType::Front => {
            v1 = Vec3::new(-0.5, -0.5, -0.5);
            v2 = Vec3::new(0.5, -0.5, -0.5);
            v3 = Vec3::new(0.5, 0.5, -0.5);
            v4 = Vec3::new(-0.5, 0.5, -0.5);
            normal = Vec3::NEG_Z;
            v_type = 5;
        }
    };

    let offset = vertices_coord.len() as u32;

    for vx in &[v1, v2, v3, v4] {
        vertices_type.push(v_type);
        vertices_id.push(v_id);

        vertices_coord.push(vx + cube_center);
        vertices_normal.push(normal);
    }

    for offset_increment in &[0, 3, 1, 1, 3, 2] {
        vertices_order.push(offset + offset_increment);
    }

    match cube_type {
        CubeTypes::Dirt => {
            map_texture(uv_coord, 8, 4);
        }
        // CubeTypes::Stone => {
        //     map_texture(uv_coord, 5, 3);
        // }
        CubeTypes::Empty => {
            println!("Representing empty cube ? {:?}", cube_center);
        }
    }
}

fn build_mesh(
    voxels: &[CubeTypes],
    chunk_coord: &(usize, usize, usize),
) -> (
    Vec<Vec3>,
    Vec<Vec2>,
    Vec<Vec3>,
    Vec<u32>,
    Vec<u32>,
    Vec<f32>,
) {
    let (
        mut vertices_coord,
        mut uv_coord,
        mut vertices_normal,
        mut vertices_order,
        mut vertices_type,
        mut vertices_id,
    ) = (
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    for p_x in 0..CHUNK_SIZE {
        for p_y in 0..CHUNK_SIZE {
            for p_z in 0..CHUNK_SIZE {
                // let cube_coord = Vec3::new(p_x as f32, p_y as f32, p_z as f32);
                let cube_coord = (p_x, p_y, p_z);
                if get_cube_type(voxels, chunk_coord, &cube_coord) != CubeTypes::Empty {
                    let cube_type = get_cube_type(voxels, chunk_coord, &cube_coord);
                    let mut face_to_add: Vec<FaceType> = Vec::new();

                    // Top vertices
                    if is_void(voxels, chunk_coord, &cube_coord, &FaceType::Top.into()) {
                        face_to_add.push(FaceType::Top);
                    }
                    // Bottom vertices_coord
                    if is_void(voxels, chunk_coord, &cube_coord, &FaceType::Bottom.into()) {
                        face_to_add.push(FaceType::Bottom)
                    }
                    // Right vertices_coord
                    if is_void(voxels, chunk_coord, &cube_coord, &FaceType::Right.into()) {
                        face_to_add.push(FaceType::Right);
                    }
                    // Left vertices_coord
                    if is_void(voxels, chunk_coord, &cube_coord, &FaceType::Left.into()) {
                        face_to_add.push(FaceType::Left);
                    }
                    // Back vertices_coord
                    if is_void(voxels, chunk_coord, &cube_coord, &FaceType::Back.into()) {
                        face_to_add.push(FaceType::Back);
                    }
                    // Front vertices_coord
                    if is_void(voxels, chunk_coord, &cube_coord, &FaceType::Front.into()) {
                        face_to_add.push(FaceType::Front);
                    }
                    for face in face_to_add {
                        add_face(
                            &mut vertices_coord,
                            &mut uv_coord,
                            &mut vertices_normal,
                            &mut vertices_order,
                            &mut vertices_type,
                            &mut vertices_id,
                            face,
                            &cube_type,
                            Vec3::new(p_x as f32, p_y as f32, p_z as f32),
                        );
                    }
                }
            }
        }
    }
    (
        vertices_coord,
        uv_coord,
        vertices_normal,
        vertices_order,
        vertices_type,
        vertices_id,
    )
}
