use crate::{
    combat::{Damage, DamageEvent, Health},
    enemy::Enemy,
    player::{Player, SpellEvent},
    sprite_sheets::{Animation, SpriteSheetsMaps},
    Score,
};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Particle {
    damage: u16,
}

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(shoot_particle)
            .add_system(handle_particle_contacts);
    }
}

fn shoot_particle(
    mut commands: Commands,
    sprite_sheet_maps: Res<SpriteSheetsMaps>,
    mut spell_events: EventReader<SpellEvent>,
    mut player_query: Query<&Transform, With<Player>>,
) {
    let transform = player_query.single_mut();
    for spell_event in spell_events.iter() {
        commands.spawn((
            RigidBody::KinematicVelocityBased,
            Velocity {
                linvel: Vec2::new(spell_event.direction.x, spell_event.direction.y) * 300.,
                ..default()
            },
            Collider::compound(vec![(Vec2::new(4.0, 0.0), 0., Collider::ball(10.))]),
            GravityScale(0.),
            SpriteSheetBundle {
                texture_atlas: sprite_sheet_maps.fireball_atlas.clone(),
                sprite: TextureAtlasSprite {
                    index: 0,
                    custom_size: Some(Vec2::splat(32.)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    transform.translation.x,
                    transform.translation.y,
                    0.,
                )
                .with_rotation(Quat::from_rotation_arc(
                    Vec3::X,
                    spell_event.direction.extend(0.),
                )),
                ..default()
            },
            Name::from("Particle"),
            Particle { damage: 8 },
            Animation {
                start: 0,
                end: 2,
                timer: Timer::from_seconds(0.1, TimerMode::Repeating),
            },
        ));
    }
}

fn handle_particle_contacts(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    query: Query<(Entity, &mut Particle)>,
    mut enemy_query: Query<(Entity, &mut Enemy, &mut TextureAtlasSprite)>,
    mut score: ResMut<Score>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    for collision_event in collision_events.iter() {
        for (entity, particle) in query.iter() {
            if let CollisionEvent::Started(e1, e2, _) = collision_event {
                if e1 == &entity || e2 == &entity {
                    commands.entity(entity).despawn();
                    if let Some((entity, mut enemy, mut texture)) = enemy_query
                        .iter_mut()
                        .filter(|(enemy_entity, _, _)| enemy_entity == e1 || enemy_entity == e2)
                        .next()
                    {
                        enemy.health -= particle.damage as i16;
                        damage_events.send(DamageEvent { entity });
                        texture.color = Color::rgba(255., 255., 255., 1.);
                        commands.entity(entity).insert(Damage::default());
                    };
                }
            }
        }
    }

    for (entity, enemy, _) in enemy_query.iter() {
        if enemy.health <= 0 {
            commands.entity(entity).despawn();
            score.value += 1;
        }
    }
}
