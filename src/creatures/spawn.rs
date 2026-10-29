use crate::brain::NeuralController;
use avian3d::prelude::*;
use bevy::prelude::*;

use super::{Creature, Genome};

pub fn spawn_first_creature(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Sphere::new(0.25));

    let left = commands
        .spawn((
            Name::new("Left Mass"),
            Mesh3d(mesh.clone()),
            MeshMaterial3d(materials.add(Color::srgb(0.2, 0.5, 1.0))),
            Transform::from_xyz(-0.6, 0.27, 0.0),
            RigidBody::Dynamic,
            Collider::sphere(0.25),
            Mass(1.0),
            LockedAxes::ROTATION_LOCKED,
            Friction::new(0.0).with_combine_rule(CoefficientCombine::Multiply),
        ))
        .id();

    let right = commands
        .spawn((
            Name::new("Right Mass"),
            Mesh3d(mesh),
            MeshMaterial3d(materials.add(Color::srgb(0.7, 0.3, 0.9))),
            Transform::from_xyz(0.6, 0.27, 0.0),
            RigidBody::Dynamic,
            Collider::sphere(0.25),
            Mass(1.0),
            LockedAxes::ROTATION_LOCKED,
            Friction::new(0.0).with_combine_rule(CoefficientCombine::Multiply),
        ))
        .id();

    commands.spawn((
        Name::new("Creature 001"),
        Creature {
            masses: [left, right],
            genome: Genome::default(),
            stiffness: 70.0,
            damping: 5.0,
            elapsed: 0.0,
        },
        NeuralController::default(),
    ));
}
