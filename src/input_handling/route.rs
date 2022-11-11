use bevy::prelude::*;

use crate::{
    player::Player,
    tools::compute_distance,
};


#[derive(Component)]
pub struct Route;

#[derive(Component)]
pub struct RouteVisual;


pub trait RouteVisualization {
    fn spawn_route(
        &self,
        commands: &mut Commands, 
        player_transform: &Transform, 
        player: &Player);
}

impl RouteVisualization for RouteVisual {
    
    fn spawn_route(
        &self,
        commands: &mut Commands,
        player_transform: &Transform, 
        player: &Player,
    ) { 
        if player.on_move {
            let route_parent = commands.spawn_bundle(SpatialBundle::default()).insert(Self).id();
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
                        translation: Vec3::new(pos[0], pos[1], pos[2]),
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

}

pub fn check_route_visual(
    mut commands: Commands,
    player_query: Query<&Player>,
    route_query: Query<Entity, (With<Route>, Without<Player>)>
){
    let player = player_query.get_single().unwrap();
    for route in route_query.iter(){
        if !player.on_move {
            commands.entity(route).despawn_recursive();
        }
    }
}


/* 

pub fn spawn_route(
    commands: &mut Commands,
    player_transform: &Transform, 
    player: &Player,
) { 
    if player.on_move {
        let route_parent = commands.spawn_bundle(SpatialBundle::default()).insert(Route).id();
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
                    translation: Vec3::new(pos[0], pos[1], 1.0),
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

*/
