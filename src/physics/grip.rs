use avian3d::prelude::*;
use bevy::prelude::*;

use crate::creatures::Creature;

const GRIP_DAMPING: f32 = 45.0;
const MAX_GRIP_FORCE: f32 = 35.0;
const FOOT_RADIUS: f32 = 0.25;

pub fn apply_grip_forces(creatures: Query<&Creature>, mut bodies: Query<Forces>) {
    for creature in &creatures {
        let phase = std::f32::consts::TAU * creature.genome.frequency * creature.elapsed
            + creature.genome.phase_offset;

        // Expansion: left foot anchored.
        // Contraction: right foot anchored.
        let anchored_entity = if phase.cos() >= 0.0 {
            creature.masses[0]
        } else {
            creature.masses[1]
        };

        let Ok(mut foot) = bodies.get_mut(anchored_entity) else {
            continue;
        };

        // Only apply grip near the ground.
        if foot.position().y > FOOT_RADIUS + 0.04 {
            continue;
        }

        let velocity = foot.linear_velocity();

        let horizontal_velocity = Vec3::new(velocity.x, 0.0, velocity.z);

        // Viscous resistive force with a maximum magnitude.
        let grip_force = (-horizontal_velocity * GRIP_DAMPING).clamp_length_max(MAX_GRIP_FORCE);

        foot.apply_force(grip_force);
    }
}
