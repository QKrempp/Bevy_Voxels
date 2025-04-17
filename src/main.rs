use bevy::prelude::*;
use bevy_voxel::BevyVoxelPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BevyVoxelPlugin)
        .run();
}
