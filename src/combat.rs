use bevy::prelude::*;

use crate::{player::Player, sprite_sheets::SpriteSheetsMaps};

#[derive(Component)]
struct HealthGlobe;

#[derive(Component)]
pub struct Health {
    total: usize,
    current: usize,
}

pub struct DamageEvent {
    pub entity: Entity,
}

#[derive(Component)]
pub struct Damage {
    timer: Timer,
}

impl Default for Damage {
    fn default() -> Self {
        Damage {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
        }
    }
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DamageEvent>()
            .add_startup_system(spawn_health_globe)
            .add_system(handle_enemy_damage);
    }
}

fn spawn_health_globe(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sprite_sheets_maps: Res<SpriteSheetsMaps>,
) {
    let handle = asset_server.load("sprites/health_globe_background.png");
    commands.spawn(SpriteBundle {
        texture: handle,
        sprite: Sprite {
            custom_size: Some(Vec2::splat(64.)),
            ..default()
        },
        transform: Transform::from_xyz(32.0, 32.0, 0.0),
        ..default()
    });
    let handle = asset_server.load("sprites/health_globe_health.png");
    commands
        .spawn(SpriteBundle {
            texture: handle,
            sprite: Sprite {
                //rect: Some(Rect::new(0.0, 0.0, 16.0, 8.0)),
                ..default()
            },
            transform: Transform::from_xyz(32.0, 32.0, 0.0)
                .with_rotation(Quat::from_rotation_z(180.0_f32.to_radians()))
                .with_scale(Vec3::splat(4.0)),
            ..default()
        })
        .insert(HealthGlobe);
    let handle = asset_server.load("sprites/health_globe_overlay.png");
    commands.spawn(SpriteBundle {
        texture: handle,
        sprite: Sprite {
            custom_size: Some(Vec2::splat(64.)),
            color: Color::WHITE.with_a(0.3),
            ..default()
        },
        transform: Transform::from_xyz(32.0, 32.0, 0.0),
        ..default()
    });
}

fn health_globe_update(
    mut query: Query<(&mut Sprite), With<HealthGlobe>>,
    player_query: Query<&Health, With<Player>>,
) {
}

fn handle_enemy_damage(
    mut damage_events: EventReader<DamageEvent>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut TextureAtlasSprite, &mut Damage)>,
    time: Res<Time>,
) {
    let delta = time.delta();
    //for damage_event in damage_events.iter() {
    for (entity, mut texture, mut damage) in query.iter_mut() {
        damage.timer.tick(delta);
        if damage.timer.just_finished() {
            texture.color = Color::WHITE;
            commands.entity(entity).remove::<Damage>();
        }
    }
    //}
}
