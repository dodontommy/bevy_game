use std::time::Duration;

use bevy::prelude::*;

#[derive(Resource)]
pub struct GameData {
    pub score: i32,
    pub lives: i32,
    pub enemy_count: i32,
    pub enemy_timer: Timer,
    pub bullet_count: i32,
    pub max_bullets: i32,
}

impl Default for GameData { 
    fn default() -> Self {
        Self {
            score: 0,
            lives: 3,
            enemy_count: 0,
            enemy_timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
            bullet_count: 0,
            max_bullets: 5,
        }
    }
}