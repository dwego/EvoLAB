use avian3d::prelude::*;
use bevy::prelude::*;

use crate::creatures::Creature;

const HALF_X: i32 = 30;
const HALF_Z: i32 = 20;

pub fn draw_grid(creatures: Query<&Creature>, positions: Query<&Position>, mut gizmos: Gizmos) {
    let Some(creature) = creatures.iter().next() else {
        return;
    };

    let Ok([a, b]) = positions.get_many(creature.masses) else {
        return;
    };

    let center = (a.0 + b.0) * 0.5;

    let cx = center.x.round() as i32;
    let cz = center.z.round() as i32;

    let minor = Color::srgb(0.32, 0.36, 0.42);
    let major = Color::srgb(0.52, 0.57, 0.63);

    let y = 0.02;

    for x in (cx - HALF_X)..=(cx + HALF_X) {
        let color = if x == 0 {
            Color::srgb(0.25, 0.5, 1.0)
        } else if x % 5 == 0 {
            major
        } else {
            minor
        };

        gizmos.line(
            Vec3::new(x as f32, y, (cz - HALF_Z) as f32),
            Vec3::new(x as f32, y, (cz + HALF_Z) as f32),
            color,
        );
    }

    for z in (cz - HALF_Z)..=(cz + HALF_Z) {
        let color = if z == 0 {
            Color::srgb(1.0, 0.25, 0.25)
        } else if z % 5 == 0 {
            major
        } else {
            minor
        };

        gizmos.line(
            Vec3::new((cx - HALF_X) as f32, y, z as f32),
            Vec3::new((cx + HALF_X) as f32, y, z as f32),
            color,
        );
    }
}
