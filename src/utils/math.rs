use bevy::prelude::*;

// Distance between particle centers
pub fn calculate_distance(pos1: Vec3, pos2: Vec3) -> f32 {
    (pos2 - pos1).length()
}

// Check if two particles are colliding
pub fn are_particles_colliding(pos1: Vec3, radius1: f32, pos2: Vec3, radius2: f32) -> bool {
    calculate_distance(pos1, pos2) < (radius1 + radius2)
}

// Calculate contact point between two spheres
pub fn calculate_contact_point(pos1: Vec3, radius1: f32, pos2: Vec3, radius2: f32) -> Vec3 {
    let direction = (pos2 - pos1).normalize();
    pos1 + direction * radius1
}