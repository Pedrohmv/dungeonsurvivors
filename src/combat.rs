use bevy::prelude::*;

use crate::{player::Player, sprite_sheets::SpriteSheetsMaps};

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
            .add_system(handle_enemy_damage);
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

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
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
