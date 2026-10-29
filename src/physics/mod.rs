mod springs;

use avian3d::prelude::*;
use bevy::prelude::*;

pub struct SpringPhysicsPlugin;

impl Plugin for SpringPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedPostUpdate,
            springs::apply_spring_forces
                .after(PhysicsSystems::Prepare)
                .before(PhysicsSystems::StepSimulation),
        );
    }
}
