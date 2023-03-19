mod enemy;
mod particle;
mod player;
mod wave;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use enemy::EnemyPlugin;
use particle::ParticlePlugin;
use player::{Player, PlayerPlugin};
use wave::WavePlugin;

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
    .add_plugin(FrameTimeDiagnosticsPlugin::default())
    .add_plugin(PlayerPlugin)
    .add_plugin(EnemyPlugin)
    .add_plugin(WavePlugin)
    .add_plugin(ParticlePlugin)
    .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));

    if cfg!(feature = "debug") {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(RapierDebugRenderPlugin::default());
    }
    app.add_startup_system(setup_camera)
        .add_system(camera_follow_player)
        .add_system(display_events)
        .run();
}

fn setup_camera(mut commands: Commands, query: Query<&Window, With<PrimaryWindow>>) {
    let window = query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
        ..default()
    });
}

fn camera_follow_player(
    mut query: Query<&mut Transform, (With<Camera2d>, Without<KinematicCharacterController>)>,
    player_query: Query<&Transform, With<KinematicCharacterController>>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_translation = player_transform.translation;
        if let Ok(mut camera_transform) = query.get_single_mut() {
            camera_transform.translation.x = player_translation.x;
            camera_transform.translation.y = player_translation.y;
        }
    }
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
