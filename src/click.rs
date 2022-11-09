
use bevy::{
    prelude::*,
};
use bevy_inspector_egui::Inspectable;

use crate::{
    SCREEN_W, SCREEN_H, HEX_RADIUS, CLICK_BURNOUT,
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
            .add_system(check_click_visual);
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

fn clicker(
    mut commands: Commands, 
    mouse_input: Res<Input<MouseButton>>,
    mut mouse_event: EventReader<CursorMoved>,
    cam_data: Res<CameraData>,
    mut click_tracker: ResMut<ClickTracker>,
    mut hex_query: Query<(&Transform, &mut Visibility), With<HexTile>>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<HexTile>)>,
    time: Res<Time>,
) {
    if !click_tracker.on_burnout {
        if mouse_input.just_pressed(MouseButton::Left) {
            let mut player_transport = player_query.get_single_mut().unwrap();
            click_tracker.on_burnout = true;
            let x = mouse_event.iter().next().unwrap();
            let mouse_pos = Vec3::new(x.position[0]+cam_data.offset_w - SCREEN_W/2.0, x.position[1]+cam_data.offset_h - SCREEN_H/2.0, 100.0);
            spawn_click_visual(&mut commands, mouse_pos.clone());
            for (hex_transform, mut hex_visibility) in hex_query.iter_mut() {
                if hex_click_collision_check(mouse_pos, hex_transform.translation) {
                    hex_visibility.is_visible = !hex_visibility.is_visible;
                    player_transport.translation = hex_transform.translation;
                }
            }
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
    direction: (i8, i8),
}

fn spawn_click_visual(
    commands: &mut Commands,
    pos: Vec3,
) {
    let parent = commands.spawn_bundle(SpatialBundle::default()).id();

    // do the same for the child
    let child = commands.spawn_bundle(SpriteBundle{
        transform: Transform {
            translation: pos,
            scale: Vec3::splat(10.0),
            ..default()
        },
        ..default()
    }).insert(ClickVisual {
        timer: Timer::from_seconds(CLICK_BURNOUT, false),
        direction: (1,1)
    }).id();

    // add the child to the parent
    commands.entity(parent).push_children(&[child]);
}

fn check_click_visual(
    mut commands: Commands,
    time: Res<Time>,
    mut click_query: Query<(Entity, &mut Transform, &mut ClickVisual), With<ClickVisual>>
){
    for (click_sprite, mut click_transform, mut click) in click_query.iter_mut(){
        click.timer.tick(time.delta());
        click_transform.translation += Vec3::new(1.0,1.0,0.0);
        if click.timer.finished(){
            commands.entity(click_sprite).despawn_recursive();
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
