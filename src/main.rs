mod creatures;
mod graphics;
mod physics;
mod world;

use avian3d::prelude::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default(),
            world::WorldPlugin,
            graphics::GraphicsPlugin,
            creatures::CreaturePlugin,
            physics::SpringPhysicsPlugin,
        ))
        .run();
}
