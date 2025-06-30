use bevy::{asset::RenderAssetUsages, prelude::*, render::mesh::PrimitiveTopology};

use super::{
    ATTRIBUTE_VX_AO, ATTRIBUTE_VX_TYPE, CHUNK_AREA, CHUNK_SIZE, CHUNK_VOLUME, WORLD_AREA, WORLD_D,
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
        let (vertices_coord, uv_coord, vertices_normal, vertices_order, vertices_type, vertices_ao) =
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
            .with_inserted_attribute(ATTRIBUTE_VX_AO, vertices_ao.clone())
            .with_inserted_indices(bevy::render::mesh::Indices::U32(vertices_order.clone())),
            coord,
        }
    }
}

pub struct VxWorldCoord {
    chunk_coord: (usize, usize, usize),
    cube_coord: (usize, usize, usize),
}

impl VxWorldCoord {
    pub fn new(
        chunk_coord: (usize, usize, usize),
        cube_coord: (usize, usize, usize),
    ) -> VxWorldCoord {
        VxWorldCoord {
            chunk_coord,
            cube_coord,
        }
    }

    fn move_direction(&self, direction: &(i8, i8, i8)) -> Option<VxWorldCoord> {
        let mut new_world_coord = Self { ..*self };
        // Index due to X component
        if direction.0 >= 0 {
            let direction_usize = direction.0 as usize;
            if self.cube_coord.0 + direction_usize >= CHUNK_SIZE {
                if self.chunk_coord.0 + 1 >= WORLD_W {
                    return None;
                } else {
                    new_world_coord.chunk_coord.0 += 1;
                    new_world_coord.cube_coord.0 = self.cube_coord.0 + direction_usize - CHUNK_SIZE;
                }
            } else {
                new_world_coord.cube_coord.0 += direction_usize;
            }
        } else {
            let direction_usize = -direction.0 as usize;
            if self.cube_coord.0 < direction_usize {
                if self.chunk_coord.0 == 0 {
                    return None;
                } else {
                    new_world_coord.chunk_coord.0 -= 1;
                    new_world_coord.cube_coord.0 =
                        CHUNK_SIZE - (direction_usize - self.cube_coord.0);
                }
            } else {
                new_world_coord.cube_coord.0 -= direction_usize;
            }
        }

        // Index due to Y component
        if direction.1 >= 0 {
            let direction_usize = direction.1 as usize;
            if self.cube_coord.1 + direction_usize >= CHUNK_SIZE {
                if self.chunk_coord.1 + 1 >= WORLD_H {
                    return None;
                } else {
                    new_world_coord.chunk_coord.1 += 1;
                    new_world_coord.cube_coord.1 = self.cube_coord.1 + direction_usize - CHUNK_SIZE;
                }
            } else {
                new_world_coord.cube_coord.1 += direction_usize;
            }
        } else {
            let direction_usize = -direction.1 as usize;
            if self.cube_coord.1 < direction_usize {
                if self.chunk_coord.1 == 0 {
                    return None;
                } else {
                    new_world_coord.chunk_coord.1 -= 1;
                    new_world_coord.cube_coord.1 =
                        CHUNK_SIZE - (direction_usize - self.cube_coord.1);
                }
            } else {
                new_world_coord.cube_coord.1 -= direction_usize;
            }
        }

        // Index due to Z component
        if direction.2 >= 0 {
            let direction_usize = direction.2 as usize;
            if self.cube_coord.2 + direction_usize >= CHUNK_SIZE {
                if self.chunk_coord.2 + 1 >= WORLD_D {
                    return None;
                } else {
                    new_world_coord.chunk_coord.2 += 1;
                    new_world_coord.cube_coord.2 = self.cube_coord.2 + direction_usize - CHUNK_SIZE;
                }
            } else {
                new_world_coord.cube_coord.2 += direction_usize;
            }
        } else {
            let direction_usize = -direction.2 as usize;
            if self.cube_coord.2 < direction_usize {
                if self.chunk_coord.2 == 0 {
                    return None;
                } else {
                    new_world_coord.chunk_coord.2 -= 1;
                    new_world_coord.cube_coord.2 =
                        CHUNK_SIZE - (direction_usize - self.cube_coord.2);
                }
            } else {
                new_world_coord.cube_coord.2 -= direction_usize;
            }
        }
        Some(new_world_coord)
    }

    pub fn get_id(&self) -> usize {
        let index_x = self.chunk_coord.0 * CHUNK_VOLUME + self.cube_coord.0;
        let index_y =
            self.chunk_coord.1 * WORLD_AREA * CHUNK_VOLUME + self.cube_coord.1 * CHUNK_AREA;
        let index_z = self.chunk_coord.2 * WORLD_W * CHUNK_VOLUME + self.cube_coord.2 * CHUNK_SIZE;
        index_x + index_y + index_z
    }
}

fn get_cube_type(voxels: &[CubeTypes], world_coord: &VxWorldCoord) -> CubeTypes {
    // let index = chunk_coord.dot(Vec3::new(1.0, WORLD_AREA as f32, WORLD_W as f32)) as usize
    //     * CHUNK_VOLUME
    //     + cube_coord.dot(Vec3::new(1.0, CHUNK_AREA as f32, CHUNK_SIZE as f32)) as usize;

    match world_coord.move_direction(&(0, 0, 0)) {
        Some(new_world_coord) => voxels[new_world_coord.get_id()].clone(),
        None => CubeTypes::Empty,
    }
}

fn is_void(voxels: &[CubeTypes], world_coord: &VxWorldCoord, direction: &(i8, i8, i8)) -> bool {
    match world_coord.move_direction(direction) {
        Some(new_world_coord) => voxels[new_world_coord.get_id()] == CubeTypes::Empty,
        None => true,
    }
}

fn get_ao(
    voxels: &[CubeTypes],
    world_coord: &VxWorldCoord,
    face_type: FaceType,
) -> (u32, u32, u32, u32) {
    if let Some(new_world_coord) = world_coord.move_direction(&face_type.clone().into()) {
        let mut ao = (0, 0, 0, 0);
        // Let the surrounding of our face look like this:
        //  b | a | h
        // ---0---1---
        //  c |   | g
        // ---3---2---
        //  d | e | f
        let direction_a: (i8, i8, i8);
        let direction_b: (i8, i8, i8);
        let direction_c: (i8, i8, i8);
        let direction_d: (i8, i8, i8);
        let direction_e: (i8, i8, i8);
        let direction_f: (i8, i8, i8);
        let direction_g: (i8, i8, i8);
        let direction_h: (i8, i8, i8);
        if face_type == FaceType::Right || face_type == FaceType::Left {
            direction_a = (0, 0, -1);
            direction_b = (0, -1, -1);
            direction_c = (0, -1, 0);
            direction_d = (0, -1, 1);
            direction_e = (0, 0, 1);
            direction_f = (0, 1, 1);
            direction_g = (0, 1, 0);
            direction_h = (0, 1, -1);
        } else if face_type == FaceType::Top || face_type == FaceType::Bottom {
            direction_a = (0, 0, -1);
            direction_b = (-1, 0, -1);
            direction_c = (-1, 0, 0);
            direction_d = (-1, 0, 1);
            direction_e = (0, 0, 1);
            direction_f = (1, 0, 1);
            direction_g = (1, 0, 0);
            direction_h = (1, 0, -1);
        } else {
            direction_a = (0, -1, 0);
            direction_b = (-1, -1, 0);
            direction_c = (-1, 0, 0);
            direction_d = (-1, 1, 0);
            direction_e = (0, 1, 0);
            direction_f = (1, 1, 0);
            direction_g = (1, 0, 0);
            direction_h = (1, -1, 0);
        }
        // a
        if is_void(voxels, &new_world_coord, &direction_a) {
            ao.0 += 1;
            ao.1 += 1;
        }
        // b
        if is_void(voxels, &new_world_coord, &direction_b) {
            ao.0 += 1;
        }
        // c
        if is_void(voxels, &new_world_coord, &direction_c) {
            ao.0 += 1;
            ao.3 += 1;
        }
        // d
        if is_void(voxels, &new_world_coord, &direction_d) {
            ao.3 += 1;
        }
        // e
        if is_void(voxels, &new_world_coord, &direction_e) {
            ao.2 += 1;
            ao.3 += 1;
        }
        // f
        if is_void(voxels, &new_world_coord, &direction_f) {
            ao.2 += 1;
        }
        // g
        if is_void(voxels, &new_world_coord, &direction_g) {
            ao.1 += 1;
            ao.2 += 1;
        }
        // h
        if is_void(voxels, &new_world_coord, &direction_h) {
            ao.1 += 1;
        }
        ao
    } else {
        (3, 3, 3, 3)
    }
}

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
    vertices_ao: &mut Vec<u32>,
    face_type: FaceType,
    face_ao: (u32, u32, u32, u32),
    cube_type: &CubeTypes,
    cube_center: Vec3,
) {
    let v0: Vec3;
    let v1: Vec3;
    let v2: Vec3;
    let v3: Vec3;
    let normal: Vec3;
    let v_type: u32;

    // Axis are laid down this way:
    //  Z:1 Y:1
    //     \ |
    // -1 ---*--- X:1
    //       | \
    //      -1  -1

    match face_type {
        FaceType::Top => {
            v0 = Vec3::new(-0.5, 0.5, -0.5);
            v1 = Vec3::new(0.5, 0.5, -0.5);
            v2 = Vec3::new(0.5, 0.5, 0.5);
            v3 = Vec3::new(-0.5, 0.5, 0.5);
            normal = Vec3::Y;
            v_type = 0;
        }
        FaceType::Bottom => {
            v0 = Vec3::new(-0.5, -0.5, -0.5);
            v1 = Vec3::new(0.5, -0.5, -0.5);
            v2 = Vec3::new(0.5, -0.5, 0.5);
            v3 = Vec3::new(-0.5, -0.5, 0.5);
            normal = Vec3::NEG_Y;
            v_type = 1;
        }
        FaceType::Right => {
            v0 = Vec3::new(0.5, -0.5, -0.5);
            v1 = Vec3::new(0.5, 0.5, -0.5);
            v2 = Vec3::new(0.5, 0.5, 0.5);
            v3 = Vec3::new(0.5, -0.5, 0.5);
            normal = Vec3::X;
            v_type = 2;
        }
        FaceType::Left => {
            v0 = Vec3::new(-0.5, -0.5, -0.5);
            v1 = Vec3::new(-0.5, 0.5, -0.5);
            v2 = Vec3::new(-0.5, 0.5, 0.5);
            v3 = Vec3::new(-0.5, -0.5, 0.5);
            normal = Vec3::NEG_X;
            v_type = 3
        }
        FaceType::Back => {
            v0 = Vec3::new(-0.5, -0.5, 0.5);
            v1 = Vec3::new(0.5, -0.5, 0.5);
            v2 = Vec3::new(0.5, 0.5, 0.5);
            v3 = Vec3::new(-0.5, 0.5, 0.5);
            normal = Vec3::Z;
            v_type = 4;
        }
        FaceType::Front => {
            v0 = Vec3::new(-0.5, -0.5, -0.5);
            v1 = Vec3::new(0.5, -0.5, -0.5);
            v2 = Vec3::new(0.5, 0.5, -0.5);
            v3 = Vec3::new(-0.5, 0.5, -0.5);
            normal = Vec3::NEG_Z;
            v_type = 5;
        }
    };

    let offset = vertices_coord.len() as u32;

    for vx in &[v0, v1, v2, v3] {
        vertices_type.push(v_type);

        vertices_coord.push(vx + cube_center);
        vertices_normal.push(normal);
    }

    // To deal with anisotropy in the ambient occlusion, we flip the triangles if needed:
    //
    // 0     1     0     1
    //  +---+       +---+
    //  | / |  =>   | \ |
    //  +---+       +---+
    // 3     2     3     2

    let vertex_order: Vec<u32>;
    if face_type == FaceType::Right || face_type == FaceType::Bottom || face_type == FaceType::Back
    {
        if face_ao.1 + face_ao.3 < face_ao.0 + face_ao.2 {
            vertex_order = vec![0, 1, 2, 2, 3, 0];
        } else {
            vertex_order = vec![0, 1, 3, 2, 3, 1];
        }
    } else {
        if face_ao.1 + face_ao.3 < face_ao.0 + face_ao.2 {
            vertex_order = vec![0, 3, 2, 2, 1, 0];
        } else {
            vertex_order = vec![0, 3, 1, 2, 1, 3];
        }
    }

    vertices_ao.push(face_ao.0);
    vertices_ao.push(face_ao.1);
    vertices_ao.push(face_ao.2);
    vertices_ao.push(face_ao.3);

    for offset_increment in vertex_order {
        vertices_order.push(offset + offset_increment);
    }

    match cube_type {
        CubeTypes::Dirt => {
            map_texture(uv_coord, 5, 3);
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
    Vec<u32>,
) {
    let (
        mut vertices_coord,
        mut uv_coord,
        mut vertices_normal,
        mut vertices_order,
        mut vertices_type,
        mut vertices_ao,
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
                let world_coord = VxWorldCoord::new(chunk_coord.clone(), (p_x, p_y, p_z));
                if get_cube_type(voxels, &world_coord) != CubeTypes::Empty {
                    let cube_type = get_cube_type(voxels, &world_coord);
                    let mut face_to_add: Vec<(FaceType, (u32, u32, u32, u32))> = Vec::new();

                    // Top vertices
                    if is_void(voxels, &world_coord, &FaceType::Top.into()) {
                        let face_type = FaceType::Top;
                        face_to_add
                            .push((face_type.clone(), get_ao(voxels, &world_coord, face_type)));
                    }
                    // Bottom vertices_coord
                    if is_void(voxels, &world_coord, &FaceType::Bottom.into()) {
                        let face_type = FaceType::Bottom;
                        face_to_add
                            .push((face_type.clone(), get_ao(voxels, &world_coord, face_type)));
                    }
                    // Right vertices_coord
                    if is_void(voxels, &world_coord, &FaceType::Right.into()) {
                        let face_type = FaceType::Right;
                        face_to_add
                            .push((face_type.clone(), get_ao(voxels, &world_coord, face_type)));
                    }
                    // Left vertices_coord
                    if is_void(voxels, &world_coord, &FaceType::Left.into()) {
                        let face_type = FaceType::Left;
                        face_to_add
                            .push((face_type.clone(), get_ao(voxels, &world_coord, face_type)));
                    }
                    // Back vertices_coord
                    if is_void(voxels, &world_coord, &FaceType::Back.into()) {
                        let face_type = FaceType::Back;
                        face_to_add
                            .push((face_type.clone(), get_ao(voxels, &world_coord, face_type)));
                    }
                    // Front vertices_coord
                    if is_void(voxels, &world_coord, &FaceType::Front.into()) {
                        let face_type = FaceType::Front;
                        face_to_add
                            .push((face_type.clone(), get_ao(voxels, &world_coord, face_type)));
                    }
                    for (face_type, face_ao) in face_to_add {
                        add_face(
                            &mut vertices_coord,
                            &mut uv_coord,
                            &mut vertices_normal,
                            &mut vertices_order,
                            &mut vertices_type,
                            &mut vertices_ao,
                            face_type,
                            face_ao,
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
        vertices_ao,
    )
}
