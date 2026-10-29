mod camera;
mod debug;
mod grid;
mod scene;

use bevy::prelude::*;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<camera::CameraMode>()
            .add_systems(Startup, scene::setup_scene)
            .add_systems(
                Update,
                (
                    camera::switch_camera_mode,
                    grid::draw_grid,
                    debug::draw_springs,
                ),
            )
            .add_systems(
                PostUpdate,
                camera::follow_creature.before(bevy::transform::TransformSystems::Propagate),
            );
    }
}
