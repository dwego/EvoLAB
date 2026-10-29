use avian3d::prelude::*;
use bevy::prelude::*;

use crate::creatures::Creature;

const GROUND_WIDTH: f32 = 100.0;
const GROUND_DEPTH: f32 = 60.0;

#[derive(Component)]
struct VisualGround;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_world)
            .add_systems(Update, follow_ground);
    }
}

fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Infinite physical ground
    commands.spawn((
        Name::new("Infinite Ground"),
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
        Friction::new(1.0).with_combine_rule(CoefficientCombine::Multiply),
        Transform::default(),
    ));

    // Visual-only ground, without a collider
    commands.spawn((
        Name::new("Visual Ground"),
        VisualGround,
        Mesh3d(meshes.add(Cuboid::new(GROUND_WIDTH, 0.2, GROUND_DEPTH))),
        MeshMaterial3d(materials.add(Color::srgb(0.15, 0.17, 0.20))),
        Transform::from_xyz(0.0, -0.1, 0.0),
    ));
}

fn follow_ground(
    creatures: Query<&Creature>,
    positions: Query<&Position>,
    mut ground: Query<&mut Transform, With<VisualGround>>,
) {
    let Some(creature) = creatures.iter().next() else {
        return;
    };

    let Ok([left, right]) = positions.get_many(creature.masses) else {
        return;
    };

    let Ok(mut transform) = ground.single_mut() else {
        return;
    };

    let center = (left.0 + right.0) * 0.5;

    // Reposition only the visual mesh.
    // Snap to avoid unnecessary movement.
    transform.translation.x = (center.x / 10.0).round() * 10.0;

    transform.translation.z = (center.z / 10.0).round() * 10.0;
}
