use bevy::prelude::*;
use bevy_rapier2d::prelude::CollisionEvent;

use crate::{enemy::Enemy, player::Player, utils::lerp};

#[derive(Component)]
struct HealthGlobe;

#[derive(Component)]
pub struct Health {
    pub total: usize,
    pub current: usize,
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
            .add_system(health_globe_update)
            .add_system(handle_collisions)
            .add_system(handle_damage);
    }
}

fn spawn_health_globe(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            transform: Transform::from_xyz(32.0, 34.0, 0.0).with_scale(Vec3::splat(4.0)),
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
    mut query: Query<(&mut Sprite, &mut Transform), With<HealthGlobe>>,
    player_query: Query<&Health, (With<Player>, Changed<Health>)>,
) {
    if let Ok(player_health) = player_query.get_single() {
        let health_percentage = player_health.current as f32 * 100.0 / player_health.total as f32;
        let (mut health_globe_sprite, mut transform) = query.single_mut();
        let missing_percentage = (100.0 - health_percentage) / 100.;
        let rect_height = lerp(1.0, 15.0, missing_percentage);
        health_globe_sprite.rect = Some(Rect::new(0.0, rect_height, 16.0, 15.0));
        transform.translation.y = 33.0 - (rect_height + missing_percentage * 16.0);
    }
}

fn handle_damage(
    mut commands: Commands,
    mut query: Query<(Entity, &mut TextureAtlasSprite, &mut Damage)>,
    time: Res<Time>,
) {
    let delta = time.delta();
    for (entity, mut texture, mut damage) in query.iter_mut() {
        damage.timer.tick(delta);
        if damage.timer.just_finished() {
            texture.color = Color::WHITE;
            commands.entity(entity).remove::<Damage>();
        }
    }
}

fn handle_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<(Entity, &mut Health, &mut TextureAtlasSprite), With<Player>>,
    mut query: Query<(Entity, &mut Enemy)>,
) {
    let (player_entity, mut health, mut texture) = player_query.single_mut();
    for collision_event in collision_events.iter() {
        if let CollisionEvent::Started(e1, e2, _) = collision_event {
            for (entity, _) in query.iter_mut() {
                let is_player_entity = e1 == &player_entity || e2 == &player_entity;
                let is_enemy_entity = e1 == &entity || e2 == &entity;
                if is_player_entity && is_enemy_entity {
                    if health.current < 1 {
                        health.current = 0;
                    } else {
                        health.current -= 1;
                    }
                    texture.color = Color::rgba(255., 255., 255., 1.);
                    commands.entity(player_entity).insert(Damage::default());
                };
            }
        }
    }
}
