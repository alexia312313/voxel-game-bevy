use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Player {
    speed: f32,
}

#[derive(Component)]
pub struct PlayerModel;

#[derive(Resource)]
pub struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Default, Component)]
pub struct PlayerController {
    yaw: Quat,
    pitch: Quat,
    cursor_locked: bool,
}

pub fn setup(mut commands: Commands, ass: Res<AssetServer>) {
    commands.insert_resource(Animations(vec![
        ass.load("mereo.gltf#Animation0"),
        ass.load("mereo.gltf#Animation1"),
    ]));

    commands.spawn(PlayerController::default());
    commands
        .spawn(SceneBundle { ..default() })
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(1.0))
        .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                projection: bevy::render::camera::Projection::Perspective(PerspectiveProjection {
                    fov: PI / 2.,
                    far: 2048.0,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 2.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            });
            parent.spawn((
                SceneBundle {
                    scene: ass.load("mereo.gltf#Scene0"),
                    ..default()
                },
                PlayerModel,
            ));
        })
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(0.2, 0.5, 0.2))
                .insert(TransformBundle {
                    local: Transform::from_xyz(0.0, 0.6, 0.0),
                    global: Default::default(),
                });
        })
        .insert(Player { speed: 3.0 });
}

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    player_query: Query<Entity, With<Player>>,
    children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
    model: Query<&PlayerModel>,
    animations: Res<Animations>,
    mut animation_player: Query<&mut AnimationPlayer>,
    time: Res<Time>,
    mut done: Local<bool>,
) {
    if let Ok(mut player_animation) = animation_player.get_single_mut() {
        for player in player_query.iter() {
            if let Ok(mut transform) = transforms.get_mut(player) {
                let mut jump = Vec3::ZERO;
                let mut direction = Vec3::ZERO;

                if keyboard_input.pressed(KeyCode::A) {
                    direction -= Vec3::new(0.1, 0.0, 0.0);
                }
                if keyboard_input.pressed(KeyCode::D) {
                    direction += Vec3::new(0.1, 0.0, 0.0);
                }
                if keyboard_input.pressed(KeyCode::W) {
                    direction -= Vec3::new(0.0, 0.0, 0.1);
                }
                if keyboard_input.pressed(KeyCode::S) {
                    direction += Vec3::new(0.0, 0.0, 0.1);
                }
                if mouse_input.pressed(MouseButton::Left) {
                    player_animation.play(animations.0[0].clone_weak());
                }
                if keyboard_input.pressed(KeyCode::Space) {
                    jump += Vec3::new(0.0, 2.0, 0.0);
                }

                if direction.length() > 0.0 {
                    direction = direction.normalize();
                    if !*done {
                        player_animation.play(animations.0[1].clone_weak()).repeat();
                        *done = true;
                    }
                } else {
                    player_animation.stop_repeating();
                    *done = false;
                }

                transform.translation += direction * 3.0 * time.delta_seconds();
                transform.translation += jump * 3.0 * time.delta_seconds();
            }

            if let Ok(child_entities) = children.get(player) {
                for child_entity in child_entities.iter() {
                    if let Ok(mut transform) = transforms.get_mut(*child_entity) {
                        if model.get(*child_entity).is_ok() {
                            if keyboard_input.pressed(KeyCode::A) {
                                transform.rotation = Quat::from_rotation_y(PI);
                            }
                            if keyboard_input.pressed(KeyCode::D) {
                                transform.rotation = Quat::from_rotation_y(0.0);
                            }
                            if keyboard_input.pressed(KeyCode::W) {
                                transform.rotation = Quat::from_rotation_y(PI / 2.0);
                            }
                            if keyboard_input.pressed(KeyCode::S) {
                                transform.rotation = Quat::from_rotation_y(-PI / 2.0);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn rotate_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut controller_query: Query<&mut PlayerController>,
    mut mouse_events: EventReader<MouseMotion>,
    mut player_query: Query<&mut Transform, With<Camera3d>>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    time: Res<Time>,
) {
    let mut delta: Vec2 = Vec2::ZERO;
    let mut controller = controller_query.single_mut();

    if keyboard_input.just_pressed(KeyCode::Escape) {
        controller.cursor_locked = !controller.cursor_locked;
    }

    if controller.cursor_locked {
        for mouse_move in mouse_events.iter() {
            delta += mouse_move.delta;
        }
    }

    if let Ok(mut window) = windows.get_single_mut() {
        window.cursor.visible = !controller.cursor_locked;
        window.cursor.grab_mode = if controller.cursor_locked {
            CursorGrabMode::Confined
        } else {
            CursorGrabMode::None
        };
    }

    for event in mouse_events.iter() {
        delta += event.delta;
    }

    if delta.length() > 0.0 && controller.cursor_locked {
        delta = delta * time.delta_seconds();
        for mut transform in player_query.iter_mut() {
            controller.yaw = Quat::from_rotation_y(-delta.x * 0.05);
            controller.pitch = Quat::from_rotation_x(-delta.y * 0.05);

            // Apply yaw and pitch to the camera's transform
            transform.rotation = controller.yaw * transform.rotation;
            transform.rotation = transform.rotation * controller.pitch;
        }
    }
}
