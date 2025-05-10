use bevy::prelude::*;

#[derive(Resource, Clone, Debug)]
pub struct SimulationParameters {
    // Grid setup
    pub grid_size: usize,
    
    // Particle properties
    pub particle_radius: f32,
    pub particle_mass: f32,
    pub min_radius: f32,
    pub surface_density: f32,
    pub max_density: f32,
    
    // Material properties
    pub cohesion_coefficient: f32, // γ parameter
    pub friction_coefficient: f32, // μ̂ parameter
    pub attraction_radius_factor: f32, // Rattr/R
    
    // Environment
    pub gravity: Vec3,
    
    // Simulation
    pub time_step: f32,
    
    // Tool interaction
    pub tool_stiffness: f32,
    pub tool_damping: f32,
}

impl Default for SimulationParameters {
    fn default() -> Self {
        Self {
            grid_size: 16, // 16x16 grid = 256 particles
            
            particle_radius: 0.007, // 7mm as mentioned in the paper
            particle_mass: 0.0005,  // Calculated from density
            min_radius: 0.005,      // Minimum radius when compressed
            surface_density: 1270.0, // kg/m³, from the paper
            max_density: 1820.0,     // kg/m³, from the paper
            
            cohesion_coefficient: 4.0e-5, // γ parameter
            friction_coefficient: 0.342,   // μ̂ parameter (medium friction from P3 in paper)
            attraction_radius_factor: 1.1, // Rattr/R as mentioned in the paper
            
            gravity: Vec3::new(0.0, -1.625, 0.0), // Lunar gravity (m/s²)
            
            time_step: 1.0 / 240.0, // Physics timestep
            
            tool_stiffness: 500.0,  // N/m
            tool_damping: 0.7,      // Damping coefficient
        }
    }
}