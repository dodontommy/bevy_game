use bevy::prelude::*;
use crate::components::PlayerShip;
use crate::components::Bullet;
use crate::resources::GameData;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(keyboard_input);
    }
}

fn fire_bullet(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    game_data: &mut ResMut<GameData>,
    ship_transform: &Mut<Transform>,
    rotation: Quat,
) -> bool {
    let mut spawned = false;
    if game_data.bullet_count <= game_data.max_bullets {
        commands.spawn((SpriteBundle {
            texture: asset_server.load("./bullet.png"),
            transform: Transform {
                translation: ship_transform.translation,
                rotation: rotation,
                ..Default::default()
            },
            ..Default::default()
        },
        Bullet::default()));
        spawned = true;
    }
    
    spawned
}

fn keyboard_input(
    keyboard_input: ResMut<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &PlayerShip)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: ResMut<Windows>,
    timer: Res<Time>,
    mut game_data: ResMut<GameData>,
) {
    let (mut ship_transform, player_ship) = query.single_mut();
    let mut spawned = false;

    if keyboard_input.just_pressed(KeyCode::Right) {
        spawned = fire_bullet(
            &mut commands,
            &asset_server,
            &mut game_data,
            &mut ship_transform,
            Quat::from_rotation_z(f32::to_radians(270.0))
        );
    }

    if keyboard_input.just_pressed(KeyCode::Left) {
        spawned = fire_bullet(
            &mut commands,
            &asset_server,
            &mut game_data,
            &mut ship_transform,
            Quat::from_rotation_z(f32::to_radians(90.0))
        );
    }

    if keyboard_input.just_pressed(KeyCode::Up) {
        spawned = fire_bullet(
            &mut commands,
            &asset_server,
            &mut game_data,
            &mut ship_transform,
            Quat::from_rotation_z(f32::to_radians(0.0))
        );
    }

    if keyboard_input.just_pressed(KeyCode::Down) {
        spawned = fire_bullet(
            &mut commands,
            &asset_server,
            &mut game_data,
            &mut ship_transform,
            Quat::from_rotation_z(f32::to_radians(180.0))
        );
    }

    if spawned { 
        game_data.bullet_count += 1;
    }

    if keyboard_input.pressed(KeyCode::Right) { 
        ship_transform.rotation = Quat::from_rotation_z(f32::to_radians(270.0));
    } else if keyboard_input.pressed(KeyCode::Left) {
        ship_transform.rotation = Quat::from_rotation_z(f32::to_radians(90.0));
    } else if keyboard_input.pressed(KeyCode::Up) {
        ship_transform.rotation = Quat::from_rotation_z(f32::to_radians(0.0));
    } else if keyboard_input.pressed(KeyCode::Down) {
        ship_transform.rotation = Quat::from_rotation_z(f32::to_radians(180.0));
    }

    let mut x_direction: f32 = 0.0;
    let mut y_direction: f32 = 0.0;

    if keyboard_input.pressed(KeyCode::A) {
        x_direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        x_direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
        y_direction += 1.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        y_direction -= 1.0;
    }

    let new_x = ship_transform.translation.x + x_direction * player_ship.speed * timer.delta_seconds();
    let new_y = ship_transform.translation.y + y_direction * player_ship.speed * timer.delta_seconds();
    let max_right = windows.get_primary().unwrap().width() / 2.0;
    let max_left = -max_right;

    let max_top = windows.get_primary().unwrap().height() / 2.0;
    let max_bottom = -max_top;

    ship_transform.translation.x = new_x.clamp(max_left, max_right);   
    ship_transform.translation.y = new_y.clamp(max_bottom, max_top);
}