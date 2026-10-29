use super::camera::FollowCamera;
use bevy::prelude::*;

pub fn setup_scene(mut commands: Commands) {
    commands.spawn((
        Name::new("Follow Camera"),
        Camera3d::default(),
        Transform::from_xyz(6.0, 5.0, 9.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
        FollowCamera,
    ));

    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
