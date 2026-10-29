use avian3d::prelude::*;
use bevy::prelude::*;

use crate::brain::NeuralController;
use crate::creatures::Creature;

const GRIP_DAMPING: f32 = 45.0;
const MAX_GRIP_FORCE: f32 = 35.0;
const FOOT_RADIUS: f32 = 0.25;

pub fn apply_grip_forces(
    creatures: Query<(&Creature, &NeuralController)>,
    mut bodies: Query<Forces>,
) {
    for (creature, controller) in &creatures {
        let Ok([mut left, mut right]) = bodies.get_many_mut(creature.masses) else {
            continue;
        };

        let left_grip = controller.commands.left_grip;
        let right_grip = controller.commands.right_grip;

        if left.position().y <= FOOT_RADIUS + 0.04 {
            let v = left.linear_velocity();
            let horizontal = Vec3::new(v.x, 0.0, v.z);

            let force = (-horizontal * GRIP_DAMPING * left_grip)
                .clamp_length_max(MAX_GRIP_FORCE * left_grip);

            left.apply_force(force);
        }

        if right.position().y <= FOOT_RADIUS + 0.04 {
            let v = right.linear_velocity();
            let horizontal = Vec3::new(v.x, 0.0, v.z);

            let force = (-horizontal * GRIP_DAMPING * right_grip)
                .clamp_length_max(MAX_GRIP_FORCE * right_grip);

            right.apply_force(force);
        }
    }
}
