use avian3d::prelude::*;
use bevy::prelude::*;

use crate::creatures::Creature;

#[derive(Component)]
pub struct FollowCamera;

pub fn follow_creature(
    time: Res<Time>,
    creatures: Query<(&Creature)>,
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

    // Center of mass (equal masses)
    let center = (left.0 + right.0) * 0.5;

    // Point the camera toward the organism
    let target = Vec3::new(center.x, 0.6, center.z);

    // Camera offset relative to the organism
    let offset = Vec3::new(5.5, 4.5, 9.0);

    let desired_position = target + offset;

    // Frame-rate independent smoothing
    let alpha = 1.0 - (-4.0 * time.delta_secs()).exp();

    camera.translation = camera.translation.lerp(desired_position, alpha);

    camera.look_at(target, Vec3::Y);
}
