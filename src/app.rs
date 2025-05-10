use bevy::{
    prelude::*,
    window::{Window, WindowPlugin},
    DefaultPlugins,
};

use crate::{
    components::*,
    systems::*,
    resources::*,
};

pub struct RegolithApp {
    app: App,
}

impl RegolithApp {
    pub fn new() -> Self {
        let mut app = App::new();
        
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Regolith Simulator".to_string(),
                    ..default()
                }),
                ..default()
            }))
            .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_plugins(bevy::diagnostic::LogDiagnosticsPlugin::default())
            .insert_resource(parameters::SimulationParameters::default())
            .add_systems(Startup, setup)
            .add_systems(Update, (
                physics::update_particles,
                physics::handle_particle_interactions,
                interaction::handle_tool_interaction,
                rendering::update_visualization,
            ));
            
        Self { app }
    }
    
    pub fn run(&mut self) {
        self.app.run();
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    parameters: Res<parameters::SimulationParameters>,
) {
    // Add camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1.0, 2.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });
    
    // Add light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    
    // Initialize particles based on parameters
    spawn_particles(&mut commands, &mut meshes, &mut materials, &parameters);
    
    // Spawn interaction tool
    spawn_tool(&mut commands, &mut meshes, &mut materials);
}

fn spawn_particles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    parameters: &parameters::SimulationParameters,
) {
    // Replace Icosphere with UVSphere
    let particle_mesh = meshes.add(Mesh::from(shape::UVSphere {
        radius: parameters.particle_radius, 
        sectors: 16,
        stacks: 8
    }));
    
    let particle_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.8, 0.7, 0.6),
        ..default()
    });
    
    // Create a grid of particles
    let grid_size = parameters.grid_size;
    let spacing = parameters.particle_radius * 2.1; // Slight overlap
    
    for i in 0..grid_size {
        for j in 0..grid_size {
            for k in 0..3 { // 3 layers as mentioned in the paper
                let offset_x = if j % 2 == 1 { parameters.particle_radius } else { 0.0 };
                let offset_z = if k % 2 == 1 { parameters.particle_radius } else { 0.0 };
                
                let position = Vec3::new(
                    i as f32 * spacing * 0.866 + offset_x, // Hexagonal close packing
                    k as f32 * spacing * 0.816, // Slightly compressed in y-direction
                    j as f32 * spacing + offset_z,
                );
                
                commands.spawn((
                    PbrBundle {
                        mesh: particle_mesh.clone(),
                        material: particle_material.clone(),
                        transform: Transform::from_translation(position),
                        ..default()
                    },
                    particle::Particle {
                        position,
                        velocity: Vec3::ZERO,
                        acceleration: Vec3::ZERO,
                        mass: parameters.particle_mass,
                        radius: parameters.particle_radius,
                        min_radius: parameters.min_radius,
                        density: parameters.surface_density,
                    },
                ));
            }
        }
    }
}

fn spawn_tool(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    let tool_mesh = meshes.add(Mesh::from(shape::Box::new(0.05, 0.01, 0.04)));
    let tool_material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.3, 0.3, 0.8),
        ..default()
    });
    
    commands.spawn((
        PbrBundle {
            mesh: tool_mesh,
            material: tool_material,
            transform: Transform::from_translation(Vec3::new(0.0, 0.3, 0.0)),
            ..default()
        },
        tool::Tool {
            position: Vec3::new(0.0, 0.3, 0.0),
            rotation: Quat::IDENTITY,
            velocity: Vec3::ZERO,
            forces: Vec3::ZERO,
            torque: Vec3::ZERO,
        },
    ));
}