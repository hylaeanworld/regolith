use bevy::prelude::*;
use crate::components::{Particle, Tool};

pub fn update_visualization(
    mut particles: Query<(&Particle, &mut Transform)>,
    mut tool: Query<(&Tool, &mut Transform), Without<Particle>>,
) {
    // Update particle visualizations
    for (particle, mut transform) in particles.iter_mut() {
        transform.translation = particle.position;
        
        // Optional: Change particle appearance based on density
        // This could be done by scaling or changing color, etc.
    }
    
    // Update tool visualization
    if let Ok((tool, mut transform)) = tool.get_single_mut() {
        transform.translation = tool.position;
        transform.rotation = tool.rotation;
    }
}