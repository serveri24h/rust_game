
use bevy::{
    prelude::*, 
    sprite::ColorMaterial,
};

use super::{ check_route_visual, RouteVisual, route::RouteVisualization };

use crate::{
    SCREEN_W, SCREEN_H, CLICK_BURNOUT, CLICK_VISUAL_BURNOUT,
    tools::Collider,
    camera::CameraData, 
    hexmap::{HexTile},
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

pub enum UpdateStyle {
    Delete,
    Create,
    Idle,
}

#[derive(Component)]
pub struct ClickTracker {
    pub on_burnout: bool,
    pub timer: Timer,
    pub selected_hex: Option<u16>,
    pub requires_update: bool,
    pub change: UpdateStyle,
} 

impl ClickTracker {
    fn check_burnout(&mut self,time: Res<Time>){
        self.timer.tick(time.delta());
        if self.timer.just_finished() {
            self.on_burnout = false;
            self.timer.reset();
        }
    }
}

fn setup_clicker(mut commands: Commands){
    commands.insert_resource( ClickTracker {
        on_burnout: false,
        timer: Timer::from_seconds(CLICK_BURNOUT, false),
        selected_hex: None,
        requires_update: false,
        change: UpdateStyle::Create,
    });
    commands.insert_resource(RouteVisual);
}

fn compute_direction(
    click_pos: Vec3,
    player_pos: Vec3,
) -> (f32, f32) {
    let x_dif = click_pos[0]-player_pos[0];
    let y_dif = click_pos[1]-player_pos[1];
    let len = (x_dif*x_dif+y_dif*y_dif).sqrt();
    (x_dif/len,y_dif/len)
}

fn clicker(
    mut commands: Commands, 
    mut mouse_event: EventReader<CursorMoved>,
    mouse_input: Res<Input<MouseButton>>,
    cam_data: Res<CameraData>,
    time: Res<Time>,
    mut click_tracker: ResMut<ClickTracker>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut hex_query: Query<(
        &Transform, 
        &mut Handle<ColorMaterial>, 
        &HexTile
    ), With<HexTile>>,
    mut player_query: Query<(
        &mut Transform, 
        &mut Player
    ), (With<Player>, Without<HexTile>)>,
) {
    if !click_tracker.on_burnout {
        if mouse_input.just_pressed(MouseButton::Left) {

            // Get the player
            let (player_transport, mut player) = player_query.get_single_mut().unwrap();

            // Set click-burnout to true to avoid multiple inputs
            click_tracker.on_burnout = true;

            // Mouse Position
            let x = mouse_event.iter().next().unwrap();
            let mouse_pos = Vec3::new(x.position[0]+cam_data.offset_w - SCREEN_W/2.0, x.position[1]+cam_data.offset_h - SCREEN_H/2.0, 100.0);

            // Default click color is red
            let mut click_color = Color::rgb(0.6, 0.0, 0.0);

            // Stationary Player Logic
            if !player.on_move {
                let old_click = click_tracker.selected_hex.clone();
                for (hex_transform, hex_material, hex_tile) in hex_query.iter_mut() {  
                    
                    if hex_tile.collision_check(mouse_pos) {
                        click_color = Color::rgb(0.0, 1.0, 0.0);
                        let mut a =  materials.get_mut(&hex_material).unwrap();
                        match click_tracker.selected_hex {
                            Some(id) => {
                                if id == hex_tile.id {
                                    player.on_move = true;
                                    click_tracker.requires_update = true;
                                    click_tracker.selected_hex = None;
                                    click_tracker.change = UpdateStyle::Delete;
                                } else {
                                    player.target = Some(hex_transform.translation);
                                    player.direction = compute_direction(hex_tile.get_translation(), player_transport.translation);
                                    a.color = Color::rgb(0.0, 1.0, 0.0);
                                    click_tracker.requires_update = true;
                                    click_tracker.selected_hex = Some(hex_tile.id);
                                    click_tracker.change = UpdateStyle::Create;
                                }
                            }
                            None => { 
                                player.target = Some(hex_transform.translation);
                                player.direction = compute_direction(hex_tile.get_translation(), player_transport.translation);
                                a.color = Color::rgb(0.0, 1.0, 0.0);
                                click_tracker.requires_update = true;
                                click_tracker.selected_hex = Some(hex_tile.id);
                                click_tracker.change = UpdateStyle::Create;
                            }
                        }
                    }

                    else if old_click == Some(hex_tile.id) {
                        let mut a =  materials.get_mut(&hex_material).unwrap();
                        a.color = Color::rgba(1.0,1.0,0.0, 0.1);

                    }

                }
            }
            spawn_click_visual(&mut commands, mouse_pos.clone(), click_color);
        }
    } else {
        click_tracker.check_burnout(time);
    }
}

#[derive(Component)]
pub struct ClickVisual {
    timer: Timer,
    direction: (f32, f32),
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


