
use bevy::{
    prelude::*,
};

use crate::{
    SCREEN_W, SCREEN_H, HEX_RADIUS, CLICK_BURNOUT, CLICK_VISUAL_BURNOUT,
    tools::compute_distance,
    camera::CameraData, 
    hexmap::HexTile,
    player::Player,
};

pub struct ClickPlugin;

impl Plugin for ClickPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(setup_clicker)
            .add_system(clicker)
            .add_system(check_click_visual)
            .add_system(check_route_visual);
    }
}

#[derive(Component)]
pub struct ClickTracker {
    pub on_burnout: bool,
    pub timer: Timer,
} 

fn setup_clicker(mut commands: Commands){
    commands.insert_resource( ClickTracker {
        on_burnout: false,
        timer: Timer::from_seconds(CLICK_BURNOUT, false),
    });
}

fn compute_direction(
    click_pos: Vec3,
    player_pos: Vec3,
) -> (f32, f32){
    let x_dif = click_pos[0]-player_pos[0];
    let y_dif = click_pos[1]-player_pos[1];
    let len = (x_dif*x_dif+y_dif*y_dif).sqrt();
    (x_dif/len,y_dif/len)
}

fn clicker(
    mut commands: Commands, 
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_event: EventReader<CursorMoved>,
    cam_data: Res<CameraData>,
    mut click_tracker: ResMut<ClickTracker>,
    mut hex_query: Query<&Transform, With<HexTile>>,
    mut player_query: Query<(&mut Transform, &mut Player), (With<Player>, Without<HexTile>)>,
    time: Res<Time>,
) {
    if !click_tracker.on_burnout {
        if mouse_input.just_pressed(MouseButton::Left) {
            let (player_transport, mut player) = player_query.get_single_mut().unwrap();
            click_tracker.on_burnout = true;
            let x = mouse_event.iter().next().unwrap();
            let mouse_pos = Vec3::new(x.position[0]+cam_data.offset_w - SCREEN_W/2.0, x.position[1]+cam_data.offset_h - SCREEN_H/2.0, 100.0);
            let mut click_color = Color::rgb(1.0, 0.0, 0.0);
            if !player.on_move {
                for hex_transform in hex_query.iter_mut() {
                    if hex_click_collision_check(mouse_pos, hex_transform.translation) {
                        click_color = Color::rgb(0.0, 1.0, 0.0);
                        player.on_move = true;
                        player.direction = compute_direction(hex_transform.translation, player_transport.translation);
                        player.target = Some(hex_transform.translation);
                        spawn_route(&mut commands, &player_transport, &player);
                    }
                }
            }
            spawn_click_visual(&mut commands, mouse_pos.clone(), click_color);
        }
    } else {
        click_tracker.timer.tick(time.delta());
        if click_tracker.timer.just_finished() {
            click_tracker.on_burnout = false;
            click_tracker.timer.reset();
        }

    }
}

#[derive(Component)]
pub struct ClickVisual {
    timer: Timer,
    direction: (f32, f32),
}

#[derive(Component)]
pub struct RouteVisual;

fn spawn_route(
    commands: &mut Commands,
    player_transform: &Transform, 
    player: &Player,
) { 
    if player.on_move {
        let route_parent = commands.spawn_bundle(SpatialBundle::default()).insert(RouteVisual).id();
        let mut children = Vec::new();

        let mut pos = player_transform.translation.clone();
        let end_pos = player.target.unwrap();
        let mov = Vec3::new( player.direction.0*30.0, player.direction.1*30.0, 0.0 );

        
        while compute_distance(pos, end_pos) > 20.0  {
            children.push(commands.spawn_bundle(SpriteBundle{
                sprite: Sprite { 
                    color: Color::rgb(1.0,1.0,1.0), 
                    ..default() 
                },
                transform: Transform {
                    translation: Vec3::new(pos[0], pos[1], 10.0),
                    scale: Vec3::splat(10.0),
                    ..default()
                },
                ..default()
            }).id());
            pos += mov;
        } 
        children.push(commands.spawn_bundle( SpriteBundle{
            sprite: Sprite { 
                color: Color::rgb(1.0,1.0,1.0), 
                ..default() 
            },
            transform: Transform {
                translation: end_pos,
                scale: Vec3::splat(30.0),
                ..default()
            },
            ..default()
        }).id());
        commands.entity(route_parent).push_children(&children);
    }
}

fn spawn_click_visual(
    commands: &mut Commands,
    pos: Vec3,
    color: Color,
) {
    let parent_visual = commands.spawn_bundle(SpatialBundle::default()).id();

    let mut children = Vec::new();
    
    for dir in vec![(1.0,1.0), (1.0,-1.0), (-1.0,-1.0), (-1.0,1.0)] {
        
        children.push(commands.spawn_bundle(SpriteBundle{
            sprite: Sprite { color: color, ..default() },
            transform: Transform {
                translation: pos,
                scale: Vec3::splat(10.0),
                ..default()
            },
            ..default()
        }).insert(ClickVisual {
            timer: Timer::from_seconds(CLICK_VISUAL_BURNOUT, false),
            direction: dir
        }).id());
    }

    // add the child to the parent
    commands.entity(parent_visual).push_children(&children);
}

fn check_click_visual(
    mut commands: Commands,
    time: Res<Time>,
    mut click_query: Query<(Entity, &mut Transform, &mut ClickVisual), With<ClickVisual>>
){
    for (click_sprite, mut click_transform, mut click) in click_query.iter_mut(){
        click.timer.tick(time.delta());
        click_transform.translation += Vec3::new(click.direction.0,click.direction.1,0.0);
        if click.timer.finished(){
            commands.entity(click_sprite).despawn_recursive();
        }
    } 
}

fn check_route_visual(
    mut commands: Commands,
    player_query: Query<&Player>,
    route_query: Query<Entity, (With<RouteVisual>, Without<Player>)>
){
    let player = player_query.get_single().unwrap();
    for route in route_query.iter(){
        if !player.on_move {
            commands.entity(route).despawn_recursive();
        }
    }
}

fn hex_click_collision_check(
    click_pos: Vec3,
    hex_translation: Vec3,
) -> bool {
    let x_dif = hex_translation[0]-click_pos[0];
    let y_dif = hex_translation[1]-click_pos[1];
    let squared_dif = x_dif*x_dif+y_dif*y_dif;

    if squared_dif.sqrt() < HEX_RADIUS {
        return true;
    }; 
    return  false;
}
