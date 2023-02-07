mod plugins;
mod components;
mod constants;
mod resources;

use bevy::prelude::*;
use rand::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::render::camera::ScalingMode;
use plugins::input::InputPlugin;
use components::Bullet;
use components::PlayerShip;
use components::Enemy;
use resources::GameData;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin { 
            window: WindowDescriptor {
                title: "Space Shooter".to_string(),
                width: 800.0,
                height: 600.0,
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(InputPlugin)
        .add_startup_system(setup)
        .insert_resource(GameData::default())
        .add_system(move_bullets)
        .add_system(spawn_enemies)
        .add_system(check_collisions)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::FixedVertical(600.);
    commands.spawn(camera);
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("./ship-1.png"),
            ..default()
        },
        PlayerShip::default(),
    ));
}

fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    windows: ResMut<Windows>,
    time: Res<Time>,
    mut game_data: ResMut<GameData>,
) {
    if game_data.enemy_count >= 10 {
        return;
    }

    game_data.enemy_timer.tick(time.delta());

    if !game_data.enemy_timer.finished() {
        return;
    }

    let max_right = windows.get_primary().unwrap().width() / 2.0;
    let max_left = -max_right;

    let max_top = windows.get_primary().unwrap().height() / 2.0;
    let max_bottom = -max_top;

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(max_left..max_right);
    let y = rng.gen_range(max_bottom..max_top);

    commands.spawn((SpriteBundle {
        texture: asset_server.load("./enemy-1.png"),
        transform: Transform {
            scale: Vec3::new(0.15, 0.15, 0.15),
            translation: Vec3::new(x, y, 0.0),
            ..Default::default()
        },
        ..Default::default()
    },
    Enemy::default()));
    game_data.enemy_count += 1;
}

fn move_bullets(
    mut query: Query<(Entity, &mut Transform, &Bullet)>,
    timer: Res<Time>,
) {
    for (_entity, mut transform, bullet) in query.iter_mut() {
        let movement_direction = transform.rotation * Vec3::new(0.0, 1.0, 0.0);
        transform.translation += movement_direction * bullet.speed * timer.delta_seconds();
    }
}

fn check_collisions(
    mut commands: Commands,
    mut player_bullet_query: Query<(Entity, &Transform, &Sprite, With<Bullet>)>,
    mut enemy_ship_query: Query<(Entity, &Transform, &Sprite, With<Enemy>)>,
    windows: ResMut<Windows>,
    mut game_data: ResMut<GameData>,
) {
    let max_right = windows.get_primary().unwrap().width() / 2.0;
    let max_left = -max_right;

    let max_top = windows.get_primary().unwrap().height() / 2.0;
    let max_bottom = -max_top;

    for (entity, transform, _, _) in player_bullet_query.iter_mut() {
        if transform.translation.x > max_right {
            commands.entity(entity).despawn();
            game_data.bullet_count -= 1;

            println!("Bullet count: {}", game_data.bullet_count);
        }

        if transform.translation.x < max_left {
            commands.entity(entity).despawn();
            game_data.bullet_count -= 1;

            println!("Bullet count: {}", game_data.bullet_count);
        }

        if transform.translation.y > max_top {
            commands.entity(entity).despawn();
            game_data.bullet_count -= 1;

            println!("Bullet count: {}", game_data.bullet_count);
        }

        if transform.translation.y < max_bottom {
            commands.entity(entity).despawn();
            game_data.bullet_count -= 1;

            println!("Bullet count: {}", game_data.bullet_count);
        }
    }

    for (entity, transform, _, _) in enemy_ship_query.iter_mut() {
        for (bullet_entity, bullet_transform, bullet_sprite, _) in player_bullet_query.iter_mut() {
            if collide(
                transform.translation,
                Vec2::new(32., 32.),
                bullet_transform.translation,
                Vec2::new(12., 12.),
            ).is_some() {
                commands.entity(entity).despawn();
                commands.entity(bullet_entity).despawn();
                game_data.bullet_count -= 1;
                game_data.enemy_count -= 1;
            }
        }
    }
}