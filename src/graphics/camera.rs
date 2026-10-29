use avian3d::prelude::*;
use bevy::prelude::*;

use crate::creatures::Creature;

#[derive(Component)]
pub struct FollowCamera;

#[derive(Resource, Default, Clone, Copy, PartialEq, Eq)]
pub enum CameraMode {
    #[default]
    Follow,
    Fixed,
    Top,
    Orbit,
}

pub fn switch_camera_mode(keys: Res<ButtonInput<KeyCode>>, mut mode: ResMut<CameraMode>) {
    if keys.just_pressed(KeyCode::Digit1) {
        *mode = CameraMode::Follow;
    } else if keys.just_pressed(KeyCode::Digit2) {
        *mode = CameraMode::Fixed;
    } else if keys.just_pressed(KeyCode::Digit3) {
        *mode = CameraMode::Top;
    } else if keys.just_pressed(KeyCode::Digit4) {
        *mode = CameraMode::Orbit;
    }
}

pub fn follow_creature(
    time: Res<Time>,
    mode: Res<CameraMode>,
    creatures: Query<&Creature>,
    positions: Query<&Position>,
    mut cameras: Query<&mut Transform, With<FollowCamera>>,
) {
    let Some(creature) = creatures.iter().next() else {
        return;
    };

    let Ok([left, right]) = positions.get_many(creature.masses) else {
        return;
    };

    let Ok(mut camera) = cameras.single_mut() else {
        return;
    };

    let center = (left.0 + right.0) * 0.5;
    let target = Vec3::new(center.x, 0.7, center.z);

    let (desired, look_target, up) = match *mode {
        CameraMode::Follow => (target + Vec3::new(5.5, 4.5, 9.0), target, Vec3::Y),

        CameraMode::Fixed => (Vec3::new(6.0, 5.0, 9.0), Vec3::new(0.0, 1.0, 0.0), Vec3::Y),

        CameraMode::Top => (target + Vec3::new(0.0, 18.0, 0.001), target, Vec3::Z),

        CameraMode::Orbit => {
            let angle = time.elapsed_secs() * 0.3;

            (
                target + Vec3::new(angle.cos() * 10.0, 6.0, angle.sin() * 10.0),
                target,
                Vec3::Y,
            )
        }
    };

    let alpha = 1.0 - (-5.0 * time.delta_secs()).exp();

    camera.translation = camera.translation.lerp(desired, alpha);

    camera.look_at(look_target, up);
}
