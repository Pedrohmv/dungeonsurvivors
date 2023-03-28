use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use bevy_rapier2d::prelude::*;

use crate::sprite_sheets::SpriteSheetsMaps;

const PLAYER_SIZE: f32 = 32.;

#[derive(Component)]
pub struct Cursor {
    pub translation: Vec3,
}

#[derive(Component)]
pub struct Player {
    pub destination: Vec3,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_startup_system(spawn_cursor)
            .add_startup_system(cursor_grab_system)
            .add_event::<SpellEvent>()
            .add_system(setup_player_movement)
            .add_system(handle_player_movement)
            .add_system(setup_player_spells)
            .add_system(mouse_motion);
    }
}

fn spawn_cursor(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let cursor_image = asset_server.load("sprites/cursor.png");
    let window = query.get_single().unwrap();
    let translation =
        Transform::from_xyz(window.width() / 2., window.height() / 2., 0.).translation;
    commands.spawn((
        SpriteBundle {
            texture: cursor_image,
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        },
        Cursor { translation },
    ));
}

fn spawn_player(
    mut commands: Commands,
    query: Query<&Window, With<PrimaryWindow>>,
    sprite_sheets_maps: Res<SpriteSheetsMaps>,
) {
    let window = query.get_single().unwrap();
    let player_index = (8 - 1) * 12 + (1 - 1);
    commands.spawn((
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(PLAYER_SIZE / 2., PLAYER_SIZE / 2.),
        Velocity { ..default() },
        SpriteSheetBundle {
            texture_atlas: sprite_sheets_maps.characters_atlas.clone(),
            sprite: TextureAtlasSprite {
                index: player_index,
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(window.width() / 2., window.height() / 2., 0.),
            ..default()
        },
        Player {
            destination: Vec3::ZERO,
        },
    ));
}

fn setup_player_movement(
    mut controllers: Query<(&mut Velocity, &Transform, &mut Player)>,
    mouse_input: Res<Input<MouseButton>>,
    query: Query<&Cursor>,
) {
    if let Ok((mut velocity, transform, mut player)) = controllers.get_single_mut() {
        if mouse_input.just_pressed(MouseButton::Right) {
            let cursor_translation = query.single().translation;
            player.destination = cursor_translation;
            let direction = Vec2::new(cursor_translation.x, cursor_translation.y)
                - Vec2::new(transform.translation.x, transform.translation.y);
            velocity.linvel = direction.normalize() * 120.;
        }
    }
}

fn handle_player_movement(mut query: Query<(&mut Velocity, &Transform, &Player)>) {
    if let Ok((mut velocity, transform, player)) = query.get_single_mut() {
        let distance = transform.translation.distance(player.destination);
        if transform.translation.distance(player.destination) < 1. {
            velocity.linvel = Vec2::ZERO;
        }
    }
}

pub struct SpellEvent {
    pub direction: Vec2,
}

fn setup_player_spells(
    keyboard_input: Res<Input<KeyCode>>,
    mut spell_event: EventWriter<SpellEvent>,
    controllers: Query<&Transform, With<Player>>,
    query: Query<&Transform, With<Cursor>>,
) {
    if keyboard_input.just_pressed(KeyCode::Q) {
        let player_translation = controllers.single().translation;
        let cursor_translation = query.single().translation;
        let direction = Vec2::new(cursor_translation.x - 8., cursor_translation.y + 8.)
            - Vec2::new(player_translation.x, player_translation.y);
        spell_event.send(SpellEvent {
            direction: direction.normalize(),
        });
    }
}

fn mouse_motion(
    mut query: Query<(&mut Cursor, &mut Transform)>,
    mut motion_evr: EventReader<MouseMotion>,
) {
    if let Ok((mut cursor, mut transform)) = query.get_single_mut() {
        for ev in motion_evr.iter() {
            let translation = Vec3::new(ev.delta.x, -ev.delta.y, 0.);
            cursor.translation += translation;
            transform.translation += translation;
        }
    }
}

fn cursor_grab_system(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();

    // if you want to use the cursor, but not let it leave the window,
    // use `Confined` mode:
    window.cursor.grab_mode = CursorGrabMode::Confined;

    // for a game that doesn't use the cursor (like a shooter):
    // use `Locked` mode to keep the cursor in one place
    //window.cursor.grab_mode = CursorGrabMode::Confined;
    // also hide the cursor
    window.cursor.visible = false;
}
