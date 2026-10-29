mod debug;
mod scene;

use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, scene::setup_scene);

        app.add_systems(Update, debug::draw_springs);
    }
}
