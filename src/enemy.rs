use bevy::{prelude::*, window::PrimaryWindow};
use rand::random;

const ENEMY_SIZE: f32 = 32.;

#[derive(Component)]
pub struct Enemy {
    direction: Vec2,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemies)
            .add_system(confine_enemy_movement)
            .add_system(setup_enemy_movement);
    }
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
        Vec2::new(16., 16.),
        12,
        11,
        Some(Vec2::new(1., 1.)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let enemy_index = (11 - 1) * 12 + (2 - 1);

    for _ in 0..5 {
        let x = rand::random::<f32>() * (window.width() - ENEMY_SIZE) + ENEMY_SIZE / 2.;
        let y = rand::random::<f32>() * (window.height() - ENEMY_SIZE) + ENEMY_SIZE / 2.;
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite {
                    index: enemy_index,
                    custom_size: Some(Vec2::splat(ENEMY_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(x, y, 0.),
                ..default()
            },
            Enemy {
                direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            },
        ));
    }
}

fn confine_enemy_movement(
    mut query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let x_min = ENEMY_SIZE / 2.;
    let x_max = window.width() - ENEMY_SIZE / 2.;
    let y_min = ENEMY_SIZE / 2.;
    let y_max = window.height() - ENEMY_SIZE / 2.;

    for (transform, mut enemy) in query.iter_mut() {
        let translation = transform.translation;
        if translation.x < x_min {
            enemy.direction *= Vec2::new(-1., 1.);
        } else if translation.x > x_max {
            enemy.direction *= Vec2::new(-1., 1.);
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
            Vec3::new(enemy.direction.x, enemy.direction.y, 0.) * 200. * time.delta_seconds();
    }
}
