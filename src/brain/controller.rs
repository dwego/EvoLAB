use avian3d::prelude::*;
use bevy::prelude::*;

use crate::creatures::Creature;

use super::network::NeuralNetwork;

#[derive(Debug, Clone, Copy, Default)]
pub struct MotorCommands {
    pub muscle: f32,
    pub left_grip: f32,
    pub right_grip: f32,
}

#[derive(Component)]
pub struct NeuralController {
    pub network: NeuralNetwork,
    pub commands: MotorCommands,
}

impl Default for NeuralController {
    fn default() -> Self {
        Self {
            network: NeuralNetwork::baseline(),
            commands: MotorCommands::default(),
        }
    }
}

pub fn update_brains(
    mut creatures: Query<(&Creature, &mut NeuralController)>,
    bodies: Query<(&Position, &LinearVelocity)>,
) {
    for (creature, mut controller) in &mut creatures {
        let Ok([(pa, va), (pb, vb)]) = bodies.get_many(creature.masses) else {
            continue;
        };

        let delta = pb.0 - pa.0;
        let distance = delta.length();
        let direction = delta.normalize_or_zero();

        let relative_speed = (vb.0 - va.0).dot(direction);

        let phase = std::f32::consts::TAU * creature.genome.frequency * creature.elapsed
            + creature.genome.phase_offset;

        let inputs = [
            ((distance - creature.genome.rest_length) / 0.5).clamp(-1.0, 1.0),
            (relative_speed / 2.0).clamp(-1.0, 1.0),
            phase.sin(),
            phase.cos(),
        ];

        let outputs = controller.network.forward(inputs);

        controller.commands = MotorCommands {
            muscle: outputs[0],
            left_grip: ((outputs[1] + 1.0) * 0.5).clamp(0.0, 1.0),
            right_grip: ((outputs[2] + 1.0) * 0.5).clamp(0.0, 1.0),
        };
    }
}
