mod enemy;
mod particle;
mod player;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use enemy::EnemyPlugin;
use particle::ParticlePlugin;
use player::{Player, PlayerPlugin};

const PLAYER_SIZE: f32 = 32.;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Bevy game".to_string(), // ToDo
            resolution: (1280., 720.).into(),
            canvas: Some("#bevy".to_owned()),
            ..default()
        }),
        ..default()
    }))
    .add_plugin(PlayerPlugin)
    .add_plugin(EnemyPlugin)
    .add_plugin(ParticlePlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));

    if cfg!(feature = "debug") {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(RapierDebugRenderPlugin::default());
    }
    app.add_startup_system(setup_camera)
        .add_system(confine_player_movement)
        .run();
}

fn setup_camera(mut commands: Commands, query: Query<&Window, With<PrimaryWindow>>) {
    let window = query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..default()
    });
}

fn confine_player_movement(
    mut query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut transform) = query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let x_min = PLAYER_SIZE / 2.;
        let x_max = window.width() - PLAYER_SIZE / 2.;
        let y_min = PLAYER_SIZE / 2.;
        let y_max = window.height() - PLAYER_SIZE / 2.;

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
