use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use world::ChunkMaterial;

mod player;
mod world;

pub struct BevyVoxelPlugin;

pub const WORLD_W: usize = 30;
pub const WORLD_H: usize = 2;
pub const WORLD_D: usize = 30;
pub const WORLD_AREA: usize = WORLD_W * WORLD_D;
pub const WORLD_VOL: usize = WORLD_AREA * WORLD_H;
pub const CHUNK_SIZE: usize = 32;
pub const CHUNK_AREA: usize = CHUNK_SIZE * CHUNK_SIZE;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_AREA;

pub const PLAYER_POS: Vec3 = Vec3::new(
    (WORLD_W * CHUNK_SIZE) as f32 / 2_f32,
    48.0,
    (WORLD_D * CHUNK_SIZE) as f32 / 2_f32,
);

impl Plugin for BevyVoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<ChunkMaterial>::default());
        app.insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)));
        app.add_systems(
            Startup,
            (
                cursor_grab,
                player::spawn_view_model,
                world::spawn_world_model,
            ),
        );
        app.add_systems(Update, (player::rotate_player, player::move_player));
    }
}

fn cursor_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut().unwrap();

    primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
    primary_window.cursor_options.visible = false;
}
