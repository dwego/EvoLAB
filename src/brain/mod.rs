mod controller;
mod network;

pub use controller::{MotorCommands, NeuralController};

use bevy::prelude::*;

pub struct BrainPlugin;

impl Plugin for BrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, controller::update_brains);
    }
}
