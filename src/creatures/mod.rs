mod genome;
mod spawn;

pub use genome::Genome;

use bevy::prelude::*;

#[derive(Component)]
pub struct Creature {
    pub masses: [Entity; 2],
    pub genome: Genome,

    pub stiffness: f32,
    pub damping: f32,
    pub elapsed: f32,
}

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn::spawn_first_creature);
    }
}
