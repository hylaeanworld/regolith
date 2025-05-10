use bevy::prelude::*;
use crate::{
    components::Particle,  // Remove Tool since it's unused
    resources::SimulationParameters,
};

pub fn update_particles(
    mut particles: Query<(&mut Particle, &mut Transform)>,
    parameters: Res<SimulationParameters>,
    _time: Res<Time>,  // Add underscore to unused variable
) {
    let dt = parameters.time_step;
    let gravity = parameters.gravity;
    
    for (mut particle, mut transform) in particles.iter_mut() {
        // Apply gravity
        particle.acceleration = gravity;
        
        // Store values locally to avoid borrow conflicts
        let acceleration = particle.acceleration;
        particle.velocity += acceleration * dt;
        
        let velocity = particle.velocity;
        particle.position += velocity * dt;
        
        // Apply damping (based on equation 20 in the paper)
        let damping_factor = 0.98; // Simplified damping
        particle.velocity *= damping_factor;
        
        // Floor constraint (simplified)
        if particle.position.y < particle.radius {
            particle.position.y = particle.radius;
            particle.velocity.y = particle.velocity.y.abs() * -0.5; // Bounce with energy loss
            
            // Apply friction when in contact with floor
            let friction = parameters.friction_coefficient;
            let horizontal_velocity = Vec3::new(particle.velocity.x, 0.0, particle.velocity.z);
            if horizontal_velocity.length() > 0.001 {
                let friction_force = -horizontal_velocity.normalize() * friction * particle.mass * gravity.length();
                let mass = particle.mass; // Store mass locally
                particle.velocity += friction_force * dt / mass; // Use local variable
            }
        }
        
        // Update transform
        transform.translation = particle.position;
    }
}

pub fn handle_particle_interactions(
    mut particles: Query<(Entity, &mut Particle)>,
    parameters: Res<SimulationParameters>,
) {
    let mut forces = Vec::new();
    let particle_radius = parameters.particle_radius;
    let min_radius = parameters.min_radius;
    let cohesion_coefficient = parameters.cohesion_coefficient;
    let friction_coefficient = parameters.friction_coefficient;
    let attraction_radius = particle_radius * parameters.attraction_radius_factor;
    
    let entities: Vec<_> = particles.iter().map(|(e, _)| e).collect();
    
    // First, calculate all interaction forces
    for i in 0..entities.len() {
        for j in (i+1)..entities.len() {
            let (entity_i, entity_j) = (entities[i], entities[j]);
            
            if let (Ok((_, particle_i)), Ok((_, particle_j))) = (
                particles.get(entity_i),
                particles.get(entity_j),
            ) {
                let displacement = particle_j.position - particle_i.position;
                let distance = displacement.length();
                
                if distance < attraction_radius * 2.0 {
                    let direction = displacement.normalize();
                    let force; // Remove 'mut' since the value is properly assigned
                    
                    // Calculate force based on equation 19 in the paper
                    if distance < particle_radius * 2.0 {
                        // Repulsive force when particles overlap
                        let a = distance;
                        let amin = min_radius * 2.0;
                        
                        if a < amin + 0.0001 {
                            // Very close, use linear force to prevent extreme forces
                            let k = 100.0;
                            force = direction * k * (amin + 0.0001 - a);
                        } else {
                            // Inverse proportional force from paper
                            let repulsive_force = cohesion_coefficient * (
                                1.0 / (a - amin) - 1.0 / (2.0 * particle_radius - amin)
                            );
                            force = direction * repulsive_force;
                        }
                    } else {
                        // Attractive force when particles are close but not overlapping
                        let attractive_force = cohesion_coefficient * 0.1 * (
                            1.0 / (distance - min_radius * 2.0) - 
                            1.0 / (attraction_radius * 2.0 - min_radius * 2.0)
                        );
                        force = direction * -attractive_force;
                    }
                    
                    // Apply friction (simplified from the paper)
                    let relative_velocity = particle_j.velocity - particle_i.velocity;
                    let normal_velocity = relative_velocity.dot(direction) * direction;
                    let tangential_velocity = relative_velocity - normal_velocity;
                    
                    if tangential_velocity.length() > 0.001 {
                        let friction_force = -tangential_velocity.normalize() * 
                            friction_coefficient * force.length() * 0.8;
                        forces.push((entity_i, (force + friction_force) * -1.0));
                        forces.push((entity_j, force + friction_force));
                    } else {
                        forces.push((entity_i, force * -1.0));
                        forces.push((entity_j, force));
                    }
                }
            }
        }
    }

    // Now apply all forces
    for (entity, force) in forces {
        if let Ok((_, mut particle)) = particles.get_mut(entity) {
            let mass = particle.mass; // Store mass locally to avoid borrow conflict
            particle.acceleration += force / mass;
        }
    }
}
