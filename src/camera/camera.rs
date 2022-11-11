use bevy::{
    prelude::*, 
};

use crate::{
    SCREEN_H,
    SCREEN_W,
    HEX_RADIUS,
    N_HEX_EDGE,
    BORDER_BUF
};

#[derive(Component)]
pub struct CameraData {
    pub offset_w: f32,
    pub offset_h: f32,
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(camera_setup);
    }
}

fn camera_setup(
    mut commands: Commands,
){
    let cam_data = CameraData {
        offset_w: SCREEN_W/2.0-BORDER_BUF-N_HEX_EDGE as f32 * HEX_RADIUS,
        offset_h: SCREEN_H/2.0-BORDER_BUF-N_HEX_EDGE as f32 * HEX_RADIUS,
    };

    commands.spawn_bundle(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(cam_data.offset_w, cam_data.offset_h, 100.0),
            ..default()
        },
        ..default()
    });

    commands.insert_resource(cam_data);
}