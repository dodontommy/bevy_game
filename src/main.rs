use bevy::{prelude::*, input::keyboard};

const TIME_STEP: f32 = 1.0 / 60.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(sprite_movement)
        .add_system(sprite_rotation)
        .add_system(bullet_firing)
        .add_system(move_bullets)
        .add_system(check_collisions)
        .run();
}
#[derive(Component)]
struct PlayerShip {
    rotation_speed: f32,
}

#[derive(Component)]
struct Bullet {
    speed: f32
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("./ship-1.png"),
            ..default()
        },
        PlayerShip {
            rotation_speed: f32::to_radians(360.0) / 2.0,
        }
    ));
}

fn bullet_firing(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&Transform, With<PlayerShip>)>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        let (ship_transform, _) = query.single_mut();
        commands.spawn((SpriteBundle {
            texture: asset_server.load("./bullet.png"),
            transform: Transform {
                translation: ship_transform.translation,
                rotation: ship_transform.rotation,
                ..Default::default()
            },
            ..Default::default()
        },
        Bullet {
            speed: 500.0
        }));
    }
}

fn move_bullets(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Bullet)>,
    windows: ResMut<Windows>
) {
    for (entity, mut transform, bullet) in query.iter_mut() {
        let movement_direction = transform.rotation * Vec3::new(0.0, 1.0, 0.0);
        transform.translation += movement_direction * bullet.speed * TIME_STEP;
    }
}

fn check_collisions(
    mut commands: Commands,
    mut query: Query<(Entity, &Transform, &Sprite, With<Bullet>)>,
    windows: ResMut<Windows>,
) {
    let max_right = windows.get_primary().unwrap().width() / 2.0;
    let max_left = -max_right;

    let max_top = windows.get_primary().unwrap().height() / 2.0;
    let max_bottom = -max_top;

    for (entity, transform, _, _) in query.iter_mut() {
        if transform.translation.x > max_right {
            commands.entity(entity).despawn();
        }

        if transform.translation.x < max_left {
            commands.entity(entity).despawn();
        }

        if transform.translation.y > max_top {
            commands.entity(entity).despawn();
        }

        if transform.translation.y < max_bottom {
            commands.entity(entity).despawn();
        }
    }
}

fn sprite_rotation(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&PlayerShip, &mut Transform)>,
) {
    let (ship, mut ship_transform) = query.single_mut();
    let mut rotation_factor = 0.0;

    if keyboard_input.pressed(KeyCode::Left) {
        rotation_factor += 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        rotation_factor -= 1.0;
    }

    ship_transform.rotate(Quat::from_rotation_z(
        rotation_factor * ship.rotation_speed * TIME_STEP,
    ));
}

fn sprite_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Sprite, With<PlayerShip>)>,
    windows: ResMut<Windows>,
) {
    let mut ship_transform = query.single_mut();
    let mut x_direction: f32 = 0.0;
    let mut y_direction: f32 = 0.0;

    if keyboard_input.pressed(KeyCode::A) {
        x_direction -= 2.0;
    }

    if keyboard_input.pressed(KeyCode::D) {
        x_direction += 2.0;
    }

    if keyboard_input.pressed(KeyCode::W) {
        y_direction += 2.0;
    }

    if keyboard_input.pressed(KeyCode::S) {
        y_direction -= 2.0;
    }

    let new_x = ship_transform.0.translation.x + x_direction;
    let new_y = ship_transform.0.translation.y + y_direction;
    let max_right = windows.get_primary().unwrap().width() / 2.0;
    let max_left = -max_right;

    let max_top = windows.get_primary().unwrap().height() / 2.0;
    let max_bottom = -max_top;

    ship_transform.0.translation.x = new_x.clamp(max_left, max_right);
    ship_transform.0.translation.y = new_y.clamp(max_bottom, max_top);
}