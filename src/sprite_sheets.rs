use bevy::prelude::*;

pub struct SpriteSheetPlugin;
impl Plugin for SpriteSheetPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_sprite_sheets.in_base_set(StartupSet::PreStartup))
            .add_system(animate_sprites);
    }
}

#[derive(Resource)]
pub struct SpriteSheetsMaps {
    pub characters_atlas: Handle<TextureAtlas>,
    pub fireball_atlas: Handle<TextureAtlas>,
}

#[derive(Component)]
pub struct Animation {
    pub start: usize,
    pub end: usize,
    pub timer: Timer,
}

fn animate_sprites(mut query: Query<(&mut TextureAtlasSprite, &mut Animation)>, time: Res<Time>) {
    let delta = time.delta();
    for (mut sprite, mut animation) in query.iter_mut() {
        animation.timer.tick(delta);
        if animation.timer.just_finished() {
            sprite.index = if sprite.index == animation.end {
                animation.start
            } else {
                sprite.index + 1
            }
        }
    }
}

fn setup_sprite_sheets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/tilemap.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(16., 16.),
        12,
        11,
        Some(Vec2::new(1., 1.)),
        None,
    );
    let characters_atlas = texture_atlases.add(texture_atlas);
    let player_index = (8 - 1) * 12 + (1 - 1);

    let texture_handle = asset_server.load("sprites/fireball.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(16., 16.), 3, 1, None, None);
    let fireball_atlas = texture_atlases.add(texture_atlas);
    commands.insert_resource(SpriteSheetsMaps {
        characters_atlas,
        fireball_atlas,
    });
}
