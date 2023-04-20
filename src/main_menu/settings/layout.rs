use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::main_menu::components::*;
use crate::main_menu::styles::*;

pub fn spawn_settings_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    build_option_menu(&mut commands, &asset_server);

    let window = window_query.get_single().unwrap();

    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        CameraMenu {},
    ));
}

pub fn despawn_settings_menu(
    mut commands: Commands,
    option_menu_query: Query<Entity, With<OptionMenu>>,
    camera_query: Query<Entity, With<CameraMenu>>,
) {
    if let Ok(option_menu_entity) = option_menu_query.get_single() {
        commands.entity(option_menu_entity).despawn_recursive();
    }
    if let Ok(camera_entity) = camera_query.get_single() {
        commands.entity(camera_entity).despawn();
    }
}


pub fn build_option_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let option_menu_entity = commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            //Title
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Title text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Settings MENU ",
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                });

            //Bottom holder
            parent
                .spawn(NodeBundle {
                    style: BOTTOM_HOLDER,

                    ..Default::default()
                })
                .with_children(|parent| {
                    //Image Holder

                    parent
                        .spawn(NodeBundle {
                            style: IMAGE_HOLDER,
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            //Image Loader
                            parent.spawn(ImageBundle {
                                style: IMAGE_STYLE,
                                image: asset_server.load("campfire.png").into(),
                                ..Default::default()
                            });
                        });

                    // Menu
                    parent
                        .spawn(NodeBundle {
                            style: TEXT_MENU,
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // Play Button
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: BUTTON_STYLE,
                                        background_color: NORMAL_BUTTON_COLOR.into(),
                                        ..default()
                                    },
                                    PlayButton {},
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text {
                                            sections: vec![TextSection::new(
                                                "Play",
                                                get_button_text_style(&asset_server),
                                            )],
                                            alignment: TextAlignment::Center,
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });
                            //options button
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: BUTTON_STYLE,
                                        background_color: NORMAL_BUTTON_COLOR.into(),
                                        ..default()
                                    },
                                    OptionButton {},
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text {
                                            sections: vec![TextSection::new(
                                                "Options",
                                                get_button_text_style(&asset_server),
                                            )],
                                            alignment: TextAlignment::Center,
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });

                            //  Quit Button
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: BUTTON_STYLE,
                                        background_color: NORMAL_BUTTON_COLOR.into(),
                                        ..default()
                                    },
                                    QuitButton {},
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle {
                                        text: Text {
                                            sections: vec![TextSection::new(
                                                "Quit",
                                                get_button_text_style(&asset_server),
                                            )],
                                            alignment: TextAlignment::Center,
                                            ..default()
                                        },
                                        ..default()
                                    });
                                });
                        });
                });
        })
        .id();

    option_menu_entity
}
