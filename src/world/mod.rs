use avian3d::prelude::*;
use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world);
    }
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Name::new("Ground"),
        Mesh3d(meshes.add(Cuboid::new(200.0, 0.2, 40.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.25, 0.28, 0.32))),
        Transform::from_xyz(0.0, -0.1, 0.0),
        RigidBody::Static,
        Collider::cuboid(200.0, 0.2, 40.0),
        Friction::new(1.0).with_combine_rule(CoefficientCombine::Multiply),
    ));
}
