use std::f32::consts::FRAC_PI_2;

use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use super::PLAYER_POS;

/// A struct to identify the Player component through queries
#[derive(Debug, Component)]
pub struct Player;

/// A struct to identify the Camera sensitivity parameter component through queries
#[derive(Debug, Component, Deref, DerefMut)]
pub struct CameraSensitivity(Vec2);

impl Default for CameraSensitivity {
    fn default() -> Self {
        Self(Vec2::new(0.006, 0.004))
    }
}

#[derive(Debug, Component)]
struct WorldModelCamera;

/// Spawning the camera into the scene. Skipping the arm part of the [Bevy first person view
/// model example](https://bevyengine.org/examples/camera/first-person-view-model/)
pub fn spawn_view_model(mut commands: Commands) {
    commands
        .spawn((
            Player,
            CameraSensitivity::default(),
            Transform::from_translation(PLAYER_POS),
            Visibility::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                WorldModelCamera,
                Camera3d::default(),
                Projection::from(PerspectiveProjection {
                    fov: 90.0_f32.to_radians(),
                    ..default()
                }),
            ));
        });
}

pub fn rotate_player(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    mut player: Query<(&mut Transform, &CameraSensitivity), With<Player>>,
) {
    let Ok((mut transform, camera_sensitivity)) = player.single_mut() else {
        return;
    };
    let delta = accumulated_mouse_motion.delta;
    if delta != Vec2::ZERO {
        let delta_yaw = -delta.x * camera_sensitivity.x;
        let delta_pitch = -delta.y * camera_sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);

        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}

pub fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    // Here we handle the key pressed by the player, by modifying his position according ro the
    // direction he's aimed at. To do so, we rotate some vector (found by trial and error) by the
    // camera rotation and increase or decrease the position accordingly.
    // NOTE: The *xxx_directions* vectors definitions are scoped to avoid the computation if no key is pressed. It might be
    // useless.

    let Ok(mut transform) = player.single_mut() else {
        return;
    };

    let mut velocity: Vec3 = Vec3::ZERO;
    let speed = 0.2;

    if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyS) {
        let face_direction = transform.rotation.mul_vec3(Vec3::Z);
        velocity += speed * face_direction;
    }
    if input.pressed(KeyCode::ArrowUp) || input.pressed(KeyCode::KeyW) {
        let face_direction = transform.rotation.mul_vec3(Vec3::Z);
        velocity -= speed * face_direction;
    }
    if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
        let straff_direction = transform.rotation.mul_vec3(Vec3::X);
        velocity += speed * straff_direction;
    }
    if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
        let straff_direction = transform.rotation.mul_vec3(Vec3::X);
        velocity -= speed * straff_direction;
    }
    if input.pressed(KeyCode::Space) {
        velocity += speed * Vec3::Y
    }
    if input.pressed(KeyCode::ShiftLeft) {
        velocity -= speed * Vec3::Y
    }
    transform.translation += velocity;
}
