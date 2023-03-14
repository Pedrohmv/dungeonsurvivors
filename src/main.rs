use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::random;

const PLAYER_SIZE: f32 = 32.0;
const ENEMY_SIZE: f32 = PLAYER_SIZE;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy game".to_string(), // ToDo
                resolution: (800., 600.).into(),
                canvas: Some("#bevy".to_owned()),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_enemies)
        .add_system(setup_player_movement)
        .add_system(confine_player_movement)
        .add_system(confine_enemy_movement)
        .add_system(setup_enemy_movement)
        .add_system(create_particles)
        .add_system(setup_particle_movement)
        .run();
}

fn setup_camera(mut commands: Commands, query: Query<&Window, With<PrimaryWindow>>) {
    let window = query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

#[derive(Component)]
struct Player {
    particle_timer: Timer,
}

#[derive(Component)]
struct Enemy {
    direction: Vec2,
}

#[derive(Component)]
struct Particle {
    direction: Vec3,
}

fn spawn_player(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = query.get_single().unwrap();
    let texture_handle = asset_server.load("sprites/tilemap.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        12,
        11,
        Some(Vec2::new(1.0, 1.0)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player_index = (8 - 1) * 12 + (1 - 1);
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite {
                index: player_index,
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        Player {
            particle_timer: Timer::from_seconds(2.5, TimerMode::Repeating),
        },
    ));
}

fn spawn_enemies(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = query.get_single().unwrap();
    let texture_handle = asset_server.load("sprites/tilemap.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        12,
        11,
        Some(Vec2::new(1.0, 1.0)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let enemy_index = (11 - 1) * 12 + (2 - 1);

    for _ in 0..5 {
        let x = rand::random::<f32>() * (window.width() - ENEMY_SIZE) + ENEMY_SIZE / 2.0;
        let y = rand::random::<f32>() * (window.height() - ENEMY_SIZE) + ENEMY_SIZE / 2.0;
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: enemy_index,
                    custom_size: Some(Vec2::splat(ENEMY_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.0),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

fn setup_player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        direction = direction.normalize_or_zero();
        transform.translation += direction * 500.0 * time.delta_seconds();
    }
}

fn confine_player_movement(
    mut query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let x_min = PLAYER_SIZE / 2.0;
        let x_max = window.width() - PLAYER_SIZE / 2.0;
        let y_min = PLAYER_SIZE / 2.0;
        let y_max = window.height() - PLAYER_SIZE / 2.0;

        let mut translation = transform.translation;

        if translation.x < x_min {
            translation.x = x_min
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        if translation.y < y_min {
            translation.y = y_min
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        transform.translation = translation;
    }
}

fn confine_enemy_movement(
    mut query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let x_min = ENEMY_SIZE / 2.0;
    let x_max = window.width() - ENEMY_SIZE / 2.0;
    let y_min = ENEMY_SIZE / 2.0;
    let y_max = window.height() - ENEMY_SIZE / 2.0;

    for (transform, mut enemy) in query.iter_mut() {
        let translation = transform.translation;
        if translation.x < x_min {
            enemy.direction *= Vec2::new(-1.0, 1.0);
        } else if translation.x > x_max {
            enemy.direction *= Vec2::new(-1.0, 1.0);
        }

        if translation.y < y_min {
            enemy.direction *= Vec2::NEG_Y + Vec2::X;
        } else if translation.y > y_max {
            enemy.direction *= Vec2::NEG_Y + Vec2::X;
        }
    }
}

fn setup_enemy_movement(mut query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in query.iter_mut() {
        transform.translation +=
            Vec3::new(enemy.direction.x, enemy.direction.y, 0.0) * 200.0 * time.delta_seconds();
    }
}
fn create_particles(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut query: Query<(&Transform, &mut Player)>,
) {
    let delta = time.delta();
    let texture_handle = asset_server.load("sprites/tilemap.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16.0, 16.0),
        12,
        11,
        Some(Vec2::new(1.0, 1.0)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player_index = (9 - 1) * 12 + (6 - 1);
    for (transform, mut player) in query.iter_mut() {
        player.particle_timer.tick(delta);
        if player.particle_timer.just_finished() {
            for i in 0..10 {
                let rad = (360.0 * (i as f32 + 1.0) / 10.0) * std::f32::consts::PI / 180.;
                let player_translation = transform.translation;
                let particle_translation = Vec3::new(
                    player_translation.x + PLAYER_SIZE * rad.cos(),
                    player_translation.y + PLAYER_SIZE * rad.sin(),
                    0.0,
                );
                commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        sprite: TextureAtlasSprite {
                            index: player_index,
                            custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            transform.translation.x + PLAYER_SIZE * rad.cos(),
                            transform.translation.y + PLAYER_SIZE * rad.sin(),
                            0.0,
                        ),
                        ..default()
                    },
                    Particle {
                        direction: (particle_translation - player_translation).normalize(),
                    },
                ));
            }
        }
    }
}
fn setup_particle_movement(time: Res<Time>, mut query: Query<(&mut Transform, &Particle)>) {
    for (mut transform, particle) in query.iter_mut() {
        transform.translation += Vec3::new(particle.direction.x, particle.direction.y, 0.0)
            * 200.0
            * time.delta_seconds();
    }
}
