mod creatures;
mod graphics;
mod physics;

mod brain;
mod world;

use avian3d::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            world::WorldPlugin,
            brain::BrainPlugin,
            graphics::GraphicsPlugin,
            creatures::CreaturePlugin,
            physics::SpringPhysicsPlugin,
        ))
        .run();
}
