use bevy::{
    prelude::*,
};

const SCREEN_W: f32 = 1000.0;
const SCREEN_H: f32 = 800.0;
const BORDER_BUF: f32 = 50.0;
const HEX_RADIUS: f32 = 40.0;
const N_HEX_EDGE: i32 = 5;
const CLICK_BURNOUT: f32 = 1.0;


mod hexmap;
mod hex;
mod click;
mod camera;
mod player;
mod debug;

use hexmap::MapPlugin;
use hex::create_hex;
use click::ClickPlugin;
use camera::CameraPlugin;
use player::PlayerPlugin;
use debug::DebugPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor{
            width: SCREEN_W,
            height: SCREEN_H,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(ClickPlugin)
        .add_plugin(PlayerPlugin)
        .run();
}
