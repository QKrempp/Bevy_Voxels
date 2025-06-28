use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use world::ChunkMaterial;

mod player;
mod world;

pub struct BevyVoxelPlugin;

impl Plugin for BevyVoxelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<ChunkMaterial>::default());
        app.insert_resource(ClearColor(Color::srgb(0.5, 0.5, 0.9)));
        app.add_systems(
            Startup,
            (
                // cursor_grab,
                player::spawn_view_model,
                world::spawn_world_model,
            ),
        );
        app.add_systems(Update, (player::rotate_player, player::move_player));
    }
}

// fn cursor_grab(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
//     let mut primary_window = q_windows.single_mut();

//     primary_window.cursor_options.grab_mode = CursorGrabMode::Locked;
//     primary_window.cursor_options.visible = false;
// }
