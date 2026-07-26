use bevy::prelude::*;

#[derive(Component)]
pub struct Transform{
    pub position: Vec3,
    pub rotation: Vec3 //at this time, we *only* rotate the Y axis in the player transform
}

#[derive(Component)]
pub struct Velocity{
    pub current: f32,
    pub maximum: f32
}
