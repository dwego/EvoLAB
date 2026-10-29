mod debug;
mod scene;

use avian3d::prelude::PhysicsSystems;
use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, scene::setup_scene);

        app.add_systems(Update, debug::draw_springs);
        app.add_systems(
            FixedPostUpdate,
            debug::trace_motion.after(PhysicsSystems::Writeback),
        );
    }
}
