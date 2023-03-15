use bevy::{prelude::*, window::PrimaryWindow};

const PLAYER_SIZE: f32 = 32.;

#[derive(Component)]
pub struct Player {
    pub particle_timer: Timer,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(setup_player_movement);
    }
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
        Vec2::new(16., 16.),
        12,
        11,
        Some(Vec2::new(1., 1.)),
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
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        },
        Player {
            particle_timer: Timer::from_seconds(1.5, TimerMode::Repeating),
        },
    ));
}

fn setup_player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            direction -= Vec3::new(1., 0., 0.);
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1., 0., 0.);
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0., 1., 0.);
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction -= Vec3::new(0., 1., 0.);
        }

        direction = direction.normalize_or_zero();
        transform.translation += direction * 500. * time.delta_seconds();
    }
}
