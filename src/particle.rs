use crate::{enemy::Enemy, player::Player, Score};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const PLAYER_SIZE: f32 = 32.;

#[derive(Component)]
pub struct Particle {
    damage: u16,
}

pub struct ParticlePlugin;

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(create_particles)
            .add_system(handle_particle_contacts);
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
        Vec2::new(16., 16.),
        12,
        11,
        Some(Vec2::new(1., 1.)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let player_index = (9 - 1) * 12 + (6 - 1);
    for (transform, mut player) in query.iter_mut() {
        player.particle_timer.tick(delta);
        if player.particle_timer.just_finished() {
            for i in 0..10 {
                let rad = std::f32::consts::TAU * (i as f32) / 10.;
                let player_translation = transform.translation;
                let particle_translation = Vec3::new(
                    player_translation.x + PLAYER_SIZE * rad.cos(),
                    player_translation.y + PLAYER_SIZE * rad.sin(),
                    0.,
                );
                let direction = (particle_translation - player_translation).normalize();
                commands.spawn((
                    RigidBody::KinematicVelocityBased,
                    Velocity {
                        linvel: Vec2::new(direction.x, direction.y) * 100.,
                        ..default()
                    },
                    Collider::ball(PLAYER_SIZE / 2. - 3.),
                    GravityScale(0.),
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
                            0.,
                        ),
                        ..default()
                    },
                    Name::from("Particle"),
                    Particle { damage: 8 },
                ));
            }
        }
    }
}

fn handle_particle_contacts(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut query: Query<(Entity, &mut Particle)>,
    mut enemy_query: Query<(Entity, &mut Enemy)>,
    mut score: ResMut<Score>,
) {
    for collision_event in collision_events.iter() {
        for (entity, particle) in query.iter() {
            if let CollisionEvent::Started(e1, e2, _) = collision_event {
                if e1 == &entity || e2 == &entity {
                    commands.entity(entity).despawn();
                    if let Some((enemy_entity, mut enemy)) = enemy_query
                        .iter_mut()
                        .filter(|(enemy_entity, _)| enemy_entity == e1 || enemy_entity == e2)
                        .next()
                    {
                        if enemy.health <= particle.damage {
                            commands.entity(enemy_entity).despawn();
                            score.value += 1;
                        } else {
                            enemy.health -= 8;
                        }
                    };
                }
                //commands.entity(*x).despawn();
                //          commands.entity(*other_entity).despawn();
            }
        }
    }
}
