use bevy::{
    prelude::*,
    input::mouse::{MouseMotion, MouseButton},
    window::PrimaryWindow,
};
use crate::{
    components::{Particle, Tool},
    resources::SimulationParameters,
};

pub fn handle_tool_interaction(
    mut tool_query: Query<(&mut Tool, &mut Transform)>,
    mut particle_query: Query<&mut Particle>,
    parameters: Res<SimulationParameters>,
    mut mouse_motion: EventReader<MouseMotion>,
    mouse_button: Res<Input<MouseButton>>,
    keyboard: Res<Input<KeyCode>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera: Query<(&Camera, &GlobalTransform)>,
) {
    // Get tool
    let (mut tool, mut tool_transform) = if let Ok(t) = tool_query.get_single_mut() {
        t
    } else {
        return;
    };
    
    // Handle tool movement with keyboard
    let mut movement = Vec3::ZERO;
    if keyboard.pressed(KeyCode::W) {
        movement.z -= 1.0;
    }
    if keyboard.pressed(KeyCode::S) {
        movement.z += 1.0;
    }
    if keyboard.pressed(KeyCode::A) {
        movement.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::D) {
        movement.x += 1.0;
    }
    if keyboard.pressed(KeyCode::Q) {
        movement.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::E) {
        movement.y += 1.0;
    }
    
    let move_speed = 0.03;
    if movement != Vec3::ZERO {
        movement = movement.normalize() * move_speed;
        tool.position += movement;
        tool_transform.translation = tool.position;
    }
    
    // Handle tool rotation with mouse when right mouse button is pressed
    if mouse_button.pressed(MouseButton::Right) {
        let mut rotation_delta = Vec2::ZERO;
        for event in mouse_motion.read() {
            rotation_delta += event.delta;
        }
        
        if rotation_delta != Vec2::ZERO {
            let rotation_speed = 0.01;
            let pitch = Quat::from_rotation_x(-rotation_delta.y * rotation_speed);
            let yaw = Quat::from_rotation_y(-rotation_delta.x * rotation_speed);
            tool.rotation = yaw * pitch * tool.rotation;
            tool_transform.rotation = tool.rotation;
        }
    }
    
    // Calculate tool-particle interactions
    let tool_pos = tool.position;
    let tool_size = Vec3::new(0.05, 0.01, 0.04); // Size of the tool
    
    // Reset forces on tool
    tool.forces = Vec3::ZERO;
    tool.torque = Vec3::ZERO;
    
    // Check interaction with each particle
    for mut particle in particle_query.iter_mut() {
        // Simple box collision check (simplification of the tool interaction)
        let rel_pos = particle.position - tool_pos;
        
        // Transform to tool's local coordinates
        let local_pos = tool.rotation.inverse() * rel_pos;
        
        // Check if particle is inside or close to the tool
        if local_pos.x.abs() < tool_size.x / 2.0 + particle.radius &&
           local_pos.y.abs() < tool_size.y / 2.0 + particle.radius &&
           local_pos.z.abs() < tool_size.z / 2.0 + particle.radius {
            
            // Calculate penetration
            let penetration_x = (tool_size.x / 2.0 + particle.radius) - local_pos.x.abs();
            let penetration_y = (tool_size.y / 2.0 + particle.radius) - local_pos.y.abs();
            let penetration_z = (tool_size.z / 2.0 + particle.radius) - local_pos.z.abs();
            
            // Find minimum penetration axis
            let (axis, penetration) = if penetration_x < penetration_y && penetration_x < penetration_z {
                (Vec3::new(if local_pos.x > 0.0 { 1.0 } else { -1.0 }, 0.0, 0.0), penetration_x)
            } else if penetration_y < penetration_z {
                (Vec3::new(0.0, if local_pos.y > 0.0 { 1.0 } else { -1.0 }, 0.0), penetration_y)
            } else {
                (Vec3::new(0.0, 0.0, if local_pos.z > 0.0 { 1.0 } else { -1.0 }), penetration_z)
            };
            
            // Convert axis back to world space
            let world_axis = tool.rotation * axis;
            
            // Calculate relative velocity
            let rel_vel = particle.velocity - tool.velocity;
            
            // Calculate force based on viscoelastic model (eq. 21 in paper)
            let stiffness = parameters.tool_stiffness;
            let damping = parameters.tool_damping;
            
            let normal_vel = rel_vel.dot(world_axis);
            let force_magnitude = stiffness * penetration + damping * normal_vel;
            let mass = particle.mass; // Store the mass first

            if force_magnitude > 0.0 {
                let force = world_axis * force_magnitude;
                
                // Apply force to particle
                particle.acceleration -= force / mass;
                
                // Apply force and torque to tool
                tool.forces += force;
                let torque = rel_pos.cross(force);
                tool.torque += torque;
            }
            
            // Add cohesion when particle is close to the tool surface
            if penetration < 0.001 && penetration > -0.002 * particle.radius {
                let cohesion_force = world_axis * parameters.cohesion_coefficient * 0.5;
                particle.acceleration -= cohesion_force / mass; // Then use the stored value
                tool.forces += cohesion_force;
            }
        }
    }
}