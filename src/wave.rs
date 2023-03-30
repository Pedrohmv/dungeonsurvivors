use std::cmp::max;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::random;

use crate::{combat::Health, enemy::Enemy};

const ENEMY_SIZE: f32 = 32.;

#[derive(Component)]
pub struct Wave {
    index: u32,
    timer: Timer,
}

pub struct WavePlugin;

impl Plugin for WavePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_wave)
            .add_system(spawn_enemy_wave);
    }
}

fn spawn_wave(mut commands: Commands) {
    commands.spawn(Wave {
        index: 0,
        timer: Timer::from_seconds(10., TimerMode::Repeating),
    });
}

fn spawn_enemy_wave(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    time: Res<Time>,
    mut query: Query<&mut Wave>,
) {
    let mut wave = query.get_single_mut().unwrap();
    wave.timer.tick(time.delta());
    if wave.timer.just_finished() {
        wave.index += 1;
        wave.timer.reset();

        let window = window_query.get_single().unwrap();
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

        let enemy_count = ((wave.index as f32).log(1.1) + 10.) as usize;
        println!("{}", enemy_count);
        for _ in 0..enemy_count {
            let radious = max(window.height() as usize, window.width() as usize) as f32 / 2.;
            let angle = (random::<f32>() * 360.0).to_radians();
            let x = window.width() / 2. + radious * angle.cos();
            let y = window.height() / 2. + radious * angle.sin();
            commands.spawn((
                RigidBody::KinematicVelocityBased,
                Collider::cuboid(ENEMY_SIZE / 2., ENEMY_SIZE / 2.),
                LockedAxes::ROTATION_LOCKED,
                Velocity {
                    ..Default::default()
                },
                GravityScale(0.),
                ActiveEvents::COLLISION_EVENTS,
                CollisionGroups::new(Group::GROUP_1, Group::GROUP_2),
                (ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC),
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
                Enemy,
                Health {
                    total: 16,
                    current: 16,
                },
            ));
        }
    }
}
