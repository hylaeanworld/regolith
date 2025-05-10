use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Particle {
    pub position: Vec3,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub mass: f32,
    pub radius: f32,
    pub min_radius: f32,   // Minimum radius for compressed particles
    pub density: f32,      // Current density of the particle
}