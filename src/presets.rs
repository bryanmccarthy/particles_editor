use macroquad::prelude::*;
use macroquad_particles::{self as particles};

pub fn default() -> particles::EmitterConfig {
    particles::EmitterConfig {
        ..Default::default()
    }
}

pub fn smoke() -> particles::EmitterConfig {
    particles::EmitterConfig {
        lifetime: 0.8,
        amount: 20,
        initial_direction_spread: 0.2,
        atlas: Some(particles::AtlasConfig::new(4, 4, 0..8)),
        ..Default::default()
    }
}

pub fn fire() -> particles::EmitterConfig {
    particles::EmitterConfig {
        lifetime: 0.4,
        lifetime_randomness: 0.1,
        amount: 10,
        initial_direction_spread: 0.5,
        initial_velocity: 300.0,
        atlas: Some(particles::AtlasConfig::new(4, 4, 8..)),
        size: 20.0,
        blend_mode: particles::BlendMode::Additive,
        ..Default::default()
    }
}

pub fn explosion() -> particles::EmitterConfig {
    particles::EmitterConfig {
        lifetime: 0.3,
        lifetime_randomness: 0.7,
        explosiveness: 0.95,
        amount: 30,
        initial_direction_spread: 2.0 * std::f32::consts::PI,
        initial_velocity: 200.0,
        size: 30.0,
        gravity: vec2(0.0, -1000.0),
        atlas: Some(particles::AtlasConfig::new(4, 4, 8..)),
        blend_mode: particles::BlendMode::Additive,
        ..Default::default()
    }
}
