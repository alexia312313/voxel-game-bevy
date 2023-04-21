use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_ui_health(
    mut commands: Commands,
    asset_server:Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
){
    build_ui_health(&mut commands,&asset_server);

    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle{
            transform: Transform::from_xyz(window.width()/2.0,window.height()/2.0,0.0),
            ..default()
        },
        UICamera{},
    ));
}

pub fn despawn_ui_health(
    mut commands: Commands,
 ui_health_query: Query<Entity, With<UiHealth>>,
    camera_query: Query<Entity, With<UICamera>>,
    ){
        
    if let Ok(ui_health_entity) = ui_health_query.get_single() {
        commands.entity(ui_health_entity).despawn_recursive();
    }
    if let Ok(ui_camera_entity) = camera_query.get_single() {
        commands.entity(ui_camera_entity).despawn();
    }
}

pub fn build_ui_health(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let ui_health_entity = commands
        .spawn((
            NodeBundle {
                style: main_menu_style,
                ..default()
            },
            UiHealth {},
        ))
        .with_children(|parent| {
            //Title
            parent
                .spawn(NodeBundle {
                    style: title_style,
                    ..default()
                })
                .with_children(|parent| {
                    // Title text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Bevy Voxel Game",
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });                   
        })
        .id();
    ui_health_entity
}
