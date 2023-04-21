use bevy::prelude::*;
use bevy::window::PrimaryWindow;


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

    pub fn despawn_ui_health(
    mut commands: Commands,
    settings_menu_query: Query<Entity, With<OptionMenu>>,
    camera_query: Query<Entity, With<CameraMenu>>,
    ){
        
    }


}