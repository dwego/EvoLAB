use crate::brain::NeuralController;
use avian3d::prelude::*;
use bevy::prelude::*;

use crate::creatures::Creature;

pub fn apply_spring_forces(
    time: Res<Time<Fixed>>,
    mut creatures: Query<(&mut Creature, &mut NeuralController)>,
    mut bodies: Query<Forces>,
) {
    let dt = time.delta_secs();

    for (mut creature, controller) in &mut creatures {
        let Ok([mut a, mut b]) = bodies.get_many_mut(creature.masses) else {
            continue;
        };

        let delta = b.position() - a.position();
        let distance = delta.length();

        if distance < 1e-4 {
            creature.elapsed += dt;
            continue;
        }

        let direction = delta / distance;

        // Muscle oscillation
        let phase = std::f32::consts::TAU * creature.genome.frequency * creature.elapsed
            + creature.genome.phase_offset;

        let target_length = (creature.genome.rest_length
            + creature.genome.amplitude * controller.commands.muscle)
            .max(0.3);

        // Hooke's law
        let extension = distance - target_length;

        let elastic_force = creature.stiffness * extension;

        // Damping
        let relative_velocity = b.linear_velocity() - a.linear_velocity();

        let damping_force = creature.damping * relative_velocity.dot(direction);

        // Resulting forces
        let force = direction * (elastic_force + damping_force);

        a.apply_force(force);
        b.apply_force(-force);

        creature.elapsed += dt;
    }
}
