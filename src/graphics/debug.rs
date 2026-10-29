use avian3d::prelude::*;
use bevy::prelude::*;

use crate::creatures::Creature;

pub fn draw_springs(creatures: Query<&Creature>, positions: Query<&Position>, mut gizmos: Gizmos) {
    for creature in &creatures {
        let Ok([a, b]) = positions.get_many(creature.masses) else {
            continue;
        };

        gizmos.line(a.0, b.0, Color::srgb(1.0, 0.75, 0.2));
        let phase = std::f32::consts::TAU * creature.genome.frequency * creature.elapsed
            + creature.genome.phase_offset;

        let left_anchored = phase.cos() >= 0.0;

        let green = Color::srgb(0.1, 1.0, 0.3);
        let red = Color::srgb(1.0, 0.2, 0.2);

        let left_color = if left_anchored { green } else { red };
        let right_color = if left_anchored { red } else { green };

        gizmos.line(a.0 + Vec3::Y * 0.3, a.0 + Vec3::Y * 0.65, left_color);

        gizmos.line(b.0 + Vec3::Y * 0.3, b.0 + Vec3::Y * 0.65, right_color);
    }
}

pub fn trace_motion(
    time: Res<Time<Fixed>>,
    mut counter: Local<f32>,
    creatures: Query<&Creature>,
    positions: Query<&Position>,
) {
    *counter += time.delta_secs();

    if *counter < 1.0 {
        return;
    }

    *counter = 0.0;

    for creature in &creatures {
        let Ok([a, b]) = positions.get_many(creature.masses) else {
            continue;
        };

        let center = (a.0 + b.0) * 0.5;
        let distance = a.0.distance(b.0);

        println!(
            "t={:.1}s | x={:.4} | length={:.4} | y=({:.3}, {:.3})",
            creature.elapsed, center.x, distance, a.0.y, b.0.y,
        );
    }
}
