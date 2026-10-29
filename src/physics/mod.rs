mod grip;
mod springs;

use avian3d::prelude::*;
use bevy::prelude::*;

pub struct SpringPhysicsPlugin;

impl Plugin for SpringPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedPostUpdate,
            (springs::apply_spring_forces, grip::apply_grip_forces)
                .chain()
                .after(PhysicsSystems::Prepare)
                .before(PhysicsSystems::StepSimulation),
        );
    }
}
