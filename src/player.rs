use bevy::{
    prelude::*, 
};

use crate::HEX_RADIUS;

const START_POS: (f32, f32) = (0.0, 0.0);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App ) {
        app
            .add_startup_system(setup_player)
            .add_system(animate_sprite);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("gabe-idle-run.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player = commands.spawn_bundle( SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform {
            translation: Vec3::new(START_POS.0 * HEX_RADIUS,START_POS.1*HEX_RADIUS,20.0),
            scale: Vec3::splat(2.5),
            ..default()
        },
        ..default() 
    }).id();
    commands.entity(player).insert(Player).insert(AnimationTimer(Timer::from_seconds(0.1, true)));
}