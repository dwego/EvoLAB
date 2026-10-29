use avian3d::prelude::*;
use bevy::prelude::*;

use crate::creatures::Creature;

pub fn draw_springs(creatures: Query<&Creature>, positions: Query<&Position>, mut gizmos: Gizmos) {
    for creature in &creatures {
        let Ok([a, b]) = positions.get_many(creature.masses) else {
            continue;
        };

        gizmos.line(a.0, b.0, Color::srgb(1.0, 0.75, 0.2));
    }
}
