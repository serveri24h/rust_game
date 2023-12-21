use bevy::prelude::*;

use crate::{
    player::Player,
    tools::compute_distance,
    input_handling::{ClickTracker, UpdateStyle }
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
        if !player.on_move {
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



fn spawn_route(
    commands: &mut Commands,
    player_transform: &Transform, 
    player: &Player,
) { 
    if !player.on_move {
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


pub fn check_route_visual(
    mut commands: Commands,
    player_query: Query<(&Transform, &Player), With<Player>>,
    route_query: Query<(Entity, &RouteVisual), (With<RouteVisual>, Without<Player>)>,
    mut click_tracker: ResMut<ClickTracker>,
){
    let (player_transform, player) = player_query.single();
    if click_tracker.requires_update {
        click_tracker.requires_update = false;
        for (route, route_visual) in route_query.iter(){
            commands.entity(route).despawn_recursive();
        }
        match click_tracker.change {
            UpdateStyle::Create => spawn_route(&mut commands, player_transform, player),
            UpdateStyle::Delete => println!("lol"),
            UpdateStyle::Idle => println!("lol"),
        }
    }
}
