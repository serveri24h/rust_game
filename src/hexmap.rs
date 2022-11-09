use bevy::{
    prelude::*, 
    sprite::MaterialMesh2dBundle,
};
//use rand::prelude::*;

use crate::{
    create_hex,
    HEX_RADIUS,
    N_HEX_EDGE,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup);
    }
}


#[derive(Component)]
pub struct HexTile;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
) {
    let hex = create_hex();
    let s: f32 = HEX_RADIUS*(3.0 as f32).sqrt()/2.0; 
    let mut col: Color;
    //let mut rng = rand::thread_rng();
    let mut j;

    for k in 0..(N_HEX_EDGE*2-1){
        if k < N_HEX_EDGE {
            j = k;
        } else {
            j = N_HEX_EDGE*2-2-k;
        }
        for i in 0..(N_HEX_EDGE+j) {
            //col = Color::rgb( rng.gen(), rng.gen(), rng.gen() );
            col = Color::rgb(1.0,1.0,0.0);
            let tile = commands.spawn_bundle(MaterialMesh2dBundle {
                mesh: meshes.add(hex.clone()).into(),
                transform: Transform {
                    translation: Vec3::new(1.5 * HEX_RADIUS * k as f32 ,i as f32 * 2.0*s -s * j as f32 ,0.0),
                    ..default()
                },
                material: materials.add(ColorMaterial {
                    color: col,
                    texture: Some(asset_server.load("texture.png")),
                }),
                ..default()
            }).id();
            commands.entity(tile).insert(HexTile);
        }
    }
}