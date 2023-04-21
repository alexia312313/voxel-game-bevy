use super::components::*;

use super::resources::*;
use crate::MyAssets;
use bevy::render::camera::Projection::Perspective;
use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

pub fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    player_query: Query<Entity, With<Player>>,
    children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
    model: Query<&PlayerModel>,
    animations: Res<Animations>,
    mut animation_players: Query<&mut AnimationPlayer>,
    mut query: Query<&AnimationEntityLink>,
    time: Res<Time>,
    mut done: Local<bool>,
) {
    for animation_entity in query.iter_mut() {
        if let Ok(mut player_animation) = animation_players.get_mut(animation_entity.0) {
            for player in player_query.iter() {
                if let Ok(mut transform) = transforms.get_mut(player) {
                    if mouse_input.pressed(MouseButton::Left) {
                        player_animation.play(animations.0[0].clone_weak());
                    } else {
                        let mut jump = Vec3::ZERO;
                        let mut direction = Vec3::ZERO;
                        let tr = transform.right();
                        let tf = transform.forward();

                        if keyboard_input.pressed(KeyCode::A) {
                            direction -= Vec3::new(tr.x, 0.0, tr.z);
                        }
                        if keyboard_input.pressed(KeyCode::D) {
                            direction += Vec3::new(tr.x, 0.0, tr.z);
                        }
                        if keyboard_input.pressed(KeyCode::W) {
                            direction += Vec3::new(tf.x, 0.0, tf.z);
                        }
                        if keyboard_input.pressed(KeyCode::S) {
                            direction -= Vec3::new(tf.x, 0.0, tf.z);
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
                            player_animation
                                .play(animations.0[2].clone_weak())
                                .set_speed(0.1)
                                .repeat();

                            *done = false;
                        }

                        transform.translation += direction * 3.0 * time.delta_seconds();
                        transform.translation += jump * 3.0 * time.delta_seconds();
                    }
                }
                /* Rotate
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
                */
            }
        }
    }
}

pub fn rotate_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut controller_query: Query<&mut PlayerController>,
    mut mouse_events: EventReader<MouseMotion>,
    player_query: Query<Entity, With<Player>>,
    camera: Query<&Camera3d>,
    children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
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

        for player in player_query.iter() {
            if let Ok(mut transform) = transforms.get_mut(player) {
                controller.yaw = Quat::from_rotation_y(-delta.x * 0.05);
                transform.rotation = controller.yaw * transform.rotation;
            }
            /* Pitch
            if let Ok(child_entities) = children.get(player) {
                for child_entity in child_entities.iter() {
                    if let Ok(mut transform) = transforms.get_mut(*child_entity) {
                        if camera.get(*child_entity).is_ok() {
                            controller.pitch = Quat::from_rotation_x(-delta.y * 0.05);
                            transform.rotation = transform.rotation * controller.pitch;
                        }
                    }
                }
            }
            */
        }
    }
}

pub fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    // Get all the Animation players which can be deep and hidden in the heirachy
    for entity in player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);

        // If the top parent has an animation config ref then link the player to the config
        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Problem with multiple animationsplayers for the same top parent");
        } else {
            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity.clone()));
        }
    }
}

fn get_top_parent(mut curr_entity: Entity, parent_query: &Query<&Parent>) -> Entity {
    //Loop up all the way to the top parent
    loop {
        if let Ok(parent) = parent_query.get(curr_entity) {
            curr_entity = parent.get();
        } else {
            break;
        }
    }
    curr_entity
}

pub fn equip_weapon(
    time: Res<Time>,
    player_model: Query<Entity, With<PlayerModel>>,
    weapon_model: Query<Entity, With<WeaponModel>>,
    children: Query<&Children>,
    _my_assets: Res<MyAssets>,
    mut commands: Commands,
) {
    if weapon_model.iter().count() > 0 {
        return;
    }
    for entity in &player_model {
        let mut index: u16 = 0;
        for child in children.iter_descendants(entity) {
            index += 1;
            if index == 4 {
                commands.entity(child).with_children(|parent| {
                    parent.spawn((
                        SceneBundle {
                            scene: _my_assets.sword.clone_weak(),
                            transform: Transform::from_xyz(0.0, -0.8, -0.2)
                                .with_rotation(Quat::from_rotation_y(-0.2)),
                            ..default()
                        },
                        WeaponModel {},
                    ));
                });
            }
        }
    }
}

pub fn setup(mut commands: Commands, _my_assets: Res<MyAssets>) {
    commands.insert_resource(Animations(vec![
        _my_assets.player_animation_hit.clone_weak(),
        _my_assets.player_animation_walking.clone_weak(),
        _my_assets.player_animation_idle.clone_weak(),
    ]));

    commands.spawn(PlayerController::default());
    commands
        .spawn(SceneBundle { ..default() })
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(1.0))
        .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
        .with_children(|parent| {
            parent.spawn(Camera3dBundle {
                projection: Perspective(PerspectiveProjection {
                    fov: PI / 2.,
                    far: 2048.0,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, 2.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..default()
            });
            parent.spawn((
                SceneBundle {
                    scene: _my_assets.player.clone_weak(),
                    transform: Transform::from_rotation(Quat::from_rotation_y(PI / 2.0)),
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
        .insert(Player {});
}
