use bevy::{
    prelude::*, 
};

use crate::{
    HEX_RADIUS,
    FPS,
    PLAYER_SPEED,
};

const START_POS: (f32, f32) = (0.0, 0.0);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App ) {
        app
            .add_startup_system(setup_player)
            .add_system(animate_sprite)
            .add_system(movement);
    }
}

#[derive(Component)]
pub struct Player {
    pub on_move: bool,
    pub direction: (f32, f32),
    pub target: Option<Vec3>,
    pub frame_timer: Timer,
}

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
            translation: Vec3::new(START_POS.0 * HEX_RADIUS,START_POS.1*HEX_RADIUS,100.0),
            scale: Vec3::splat(2.5),
            ..default()
        },
        ..default() 
    }).id();
    commands
        .entity(player)
        .insert(Player {
                on_move: false, 
                direction: (0.0,0.0), 
                target: None, 
                frame_timer: Timer::from_seconds(1.0/FPS, true)
            })
        .insert(AnimationTimer( 
            Timer::from_seconds(0.1, true) )
        );
}

pub fn compute_distance(    
    pos_a: Vec3,
    pos_b: Vec3,
) -> f32 {
        let x_dif = pos_a[0]-pos_b[0];
        let y_dif = pos_a[1]-pos_b[1];
        (x_dif*x_dif+y_dif*y_dif).sqrt()
}

fn movement(
    mut player_query: Query<(&mut Player, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let (mut player, mut player_transform) = player_query.single_mut();
    player.frame_timer.tick(time.delta());
    if player.on_move && player.frame_timer.just_finished(){
        let target_pos = player.target.unwrap();
        if compute_distance(player_transform.translation, target_pos) > PLAYER_SPEED {
            player_transform.translation += Vec3::new(player.direction.0*PLAYER_SPEED, player.direction.1*PLAYER_SPEED, 0.0);
        } else {
            player.on_move = false;
            player_transform.translation = Vec3::new(target_pos[0],target_pos[1],20.0);
        }
    }
}