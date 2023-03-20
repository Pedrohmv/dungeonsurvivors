use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::Velocity;
use rand::random;

use crate::player::Player;

const ENEMY_SIZE: f32 = 32.;

#[derive(Component)]
pub struct Enemy {
    pub health: i16,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup_enemy_movement);
    }
}

fn setup_enemy_movement(
    player_query: Query<&Transform, With<Player>>,
    mut query: Query<(&mut Velocity, &Transform), With<Enemy>>,
    time: Res<Time>,
) {
    let player_transform = player_query.get_single().unwrap();
    for (mut velocity, transform) in query.iter_mut() {
        let direction = player_transform.translation - transform.translation;
        velocity.linvel = Vec2::new(direction.x, direction.y).normalize() * 100.;
    }
}
