use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::game::ui::components::*;
use crate::game::ui::styles::*;

pub fn spawn_ui_health(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    build_ui_health(&mut commands, &asset_server);

    let window = window_query.get_single().unwrap();

    commands
        .spawn((
            Camera2dBundle {
                transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
                camera: Camera {
                    order: (0),
                    ..default()
                },
                ..default()
            },
            UICamera {},
        ))
        .insert(Name::new("UI camera 2d"));
}

pub fn despawn_ui_health(
    mut commands: Commands,
    ui_health_query: Query<Entity, With<UiHealth>>,
    camera_query: Query<Entity, With<UICamera>>,
) {
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
                style: UI,
                ..default()
            },
            UiHealth {},
        ))
        .insert(Name::new("UIhealth setup"))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: HEALTH_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Health
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Health: ",
                                get_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            
                            ..default()
                        },
                        ..default()
                    });
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "3",
                                    get_text_style(&asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        HealthText {},
                    ));
                });

            parent
                .spawn(NodeBundle {
                    style: SCORE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Health
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Score: ",
                                get_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    parent.spawn((
                        TextBundle {
                            text: Text {
                                sections: vec![TextSection::new(
                                    "0",
                                    get_text_style(&asset_server),
                                )],
                                alignment: TextAlignment::Center,
                                ..default()
                            },
                            ..default()
                        },
                        ScoreText {},
                    ));
                });
        })
        .id();
    ui_health_entity
}
