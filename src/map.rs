use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn spawn_map(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(WorldAssetRoot(
        asset_server.load("levels/DebugGym.glb#Scene0")
    ));
}

pub fn spawn_test_floor(
    mut commands: Commands,
) {
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(10.0, 0.1, 10.0),
                    Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(10.0, 0.1, 10.0),
                       Transform::from_xyz(0.0, 0.25, -10.0),
    ));
}

struct Plane { //defined by normal and distance from world origin
    normal: Vec3,
    distance: f32,
}
