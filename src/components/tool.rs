use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Tool {
    pub position: Vec3,
    pub rotation: Quat,
    pub velocity: Vec3,
    pub forces: Vec3,
    pub torque: Vec3,
}