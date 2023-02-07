use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerShip {
    pub rotation_speed: f32,
    pub speed: f32,
}

impl Default for PlayerShip {
    fn default() -> Self {
        Self {
            rotation_speed: f32::to_radians(360.0) / 2.0,
            speed: 200.,
        }
    }
}

#[derive(Component)]
pub struct Bullet {
    pub speed: f32
}

impl Default for Bullet {
    fn default() -> Self {
        Self {
            speed: 600.0
        }
    }
}

#[derive(Component)]
pub struct Enemy {
    pub speed: f32
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            speed: 100.0
        }
    }
}