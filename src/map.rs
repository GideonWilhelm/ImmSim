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

struct Plane { //defined by normal and distance from world origin
    normal: Vec3,
    distance: f32,
}

struct Brush {
    planes: Vec<Plane>,
    faces: Vec<Face>,
    collision: bool, //just true for starters, will update later
}

struct Face {
    plane_index: usize, //which plane the face is on
    vertices: Vec<Vec3>,
    material: usize,
}
