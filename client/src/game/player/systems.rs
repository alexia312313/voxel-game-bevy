use super::components::*;
use super::resources::*;

use crate::game::mob::components::Mob;
use crate::game::mob::resources::MobHealth;
use crate::game::resources::Health;

use crate::game::resources::AnimationEntityLink;
use crate::CamState;

use crate::MyAssets;
use bevy::render::camera::Projection::Perspective;
use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode, window::PrimaryWindow};
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

pub fn attack_sword(
    rapier_context: Res<RapierContext>,
    mob_query: Query<Entity, With<Mob>>,
    weapon_collider: Query<Entity, With<WeaponCollider>>,
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    mut mob_health: ResMut<MobHealth>,
) {
    for weapon in weapon_collider.iter() {
        for mob in mob_query.iter() {
            if rapier_context.intersection_pair(weapon, mob) == Some(true) {
                println!("The colliders {:?} and {:?} are intersecting!", weapon, mob);

                if mouse_input.pressed(MouseButton::Left) {
                    if mob_health.value > 0 {
                        mob_health.value -= 1;
                        
                    }

                    if mob_health.value == 0 {
                        commands.entity(mob).despawn_recursive()
                    }
                }
            }
        }
    }
}

pub fn mob_red(
    mob_health: ResMut<MobHealth>,
    StandardMaterial: Query<Entity, With<Handle<StandardMaterial>>>,
    mut change: ResMut<Assets<StandardMaterial>>,
) {
    
        for material1 in StandardMaterial.iter() {
            for asset in change.iter_mut() {
                asset.1.base_color = Color::RED;
                println!("Color to red");

            }
        }
    }


pub fn attack_sword_v2(
    ball_model: Query<Entity, With<WeaponCollider>>,
    rapier_context: Res<RapierContext>,
) {
    for weapon in ball_model.iter() {
        for (collider1, collider2, intersecting) in rapier_context.intersections_with(weapon) {
            if intersecting {
                println!(
                    "The entities {:?} and {:?} have intersecting colliders!",
                    collider1, collider2
                );
            }
        }
    }
}

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
    let mut index = 0;
    for animation_entity in query.iter_mut() {
        index += 1;
        if index == 1 {
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

                            //I suspect diffents pcs will run this differently, should probably use a delta time

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
                                //println!("jump");
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
    }
}

pub fn rotate_camera(
    keyboard_input: Res<Input<KeyCode>>,
    mut controller_query: Query<&mut PlayerController>,
    mut mouse_events: EventReader<MouseMotion>,
    player_query: Query<Entity, With<Player>>,
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

pub fn change_cam(
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<Entity, With<Player>>,
    mut transforms: Query<&mut Transform>,
    children: Query<&Children>,
    camera: Query<&Camera3d>,
    cam_state: Res<State<CamState>>,
    mut cam_state_next_state: ResMut<NextState<CamState>>,
) {
    for player in player_query.iter() {
        if keyboard_input.just_pressed(KeyCode::F5) {
            if cam_state.0 == CamState::CamFirst {
                println!("Entrem a cambi: {:?}", cam_state.0);
                if let Ok(child_entities) = children.get(player) {
                    for child_entity in child_entities.iter() {
                        if let Ok(mut transform) = transforms.get_mut(*child_entity) {
                            if camera.get(*child_entity).is_ok() {
                                cam_state_next_state.set(CamState::CamThird);
                                transform.translation = Vec3::new(1.5, 7.0, 4.5);
                                println!("mode: {:?}", cam_state.0);
                            }
                        }
                    }
                }
            } else if cam_state.0 == CamState::CamThird {
                print!("Entrem a cambi: {:?}", cam_state.0);
                if let Ok(child_entities) = children.get(player) {
                    println!("Entrem a mode: {:?}", cam_state.0);
                    for child_entity in child_entities.iter() {
                        if let Ok(mut transform) = transforms.get_mut(*child_entity) {
                            if camera.get(*child_entity).is_ok() {
                                cam_state_next_state.set(CamState::CamFirst);
                                transform.translation = Vec3::new(0.0, 1.4, 0.8);
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn equip_weapon(
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
                    parent
                        .spawn((
                            SceneBundle {
                                scene: _my_assets.sword.clone_weak(),
                                transform: Transform::from_xyz(0.0, -0.8, -0.2)
                                    .with_rotation(Quat::from_rotation_y(-0.2)),
                                ..default()
                            },
                            WeaponModel {},
                        ))
                        .insert(Name::new("Weapon model"));
                    parent
                        //y de height,
                        .spawn((Collider::cuboid(1.0, 1.0, 1.0), WeaponCollider {}))
                        .insert(Sensor)
                        // y positiva hacia arriba,
                        .insert(Transform::from_xyz(0.0, 0.0, 0.0))
                        .insert(Name::new("Weapon Collider"));
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

    commands
        .spawn(PlayerController::default())
        .insert(Name::new("player controller"));
    commands
        .spawn(SceneBundle { ..default() })
        .insert(RigidBody::Dynamic)
        .insert(GravityScale(1.0))
        .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
        .insert(Name::new("Player"))
        .with_children(|parent| {
            parent
                .spawn(Camera3dBundle {
                    projection: Perspective(PerspectiveProjection {
                        fov: PI / 2.,
                        far: 2048.0,
                        ..default()
                    }),
                    transform: Transform::from_xyz(0.0, 1.4, 0.8)
                        .with_rotation(Quat::from_rotation_x(-0.5)),
                    camera: Camera {
                        order: (1),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new("Camera 3d player"));
            parent
                .spawn((
                    SceneBundle {
                        scene: _my_assets.player.clone_weak(),
                        transform: Transform::from_rotation(Quat::from_rotation_y(PI / 2.0)),
                        ..default()
                    },
                    PlayerModel,
                ))
                .insert(Name::new("Asset player"));
        })
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(0.2, 0.5, 0.2))
                .insert(KinematicCharacterController {
                    translation: Some(Vec3::new(1.0, 1.0, 1.0)),
                    ..default()
                })
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(TransformBundle {
                    local: Transform::from_xyz(0.0, 0.6, 0.0),
                    global: Default::default(),
                });
        })
        .insert(Player { mode: 1 })
        .insert(Name::new("player collider"));
}

pub fn check_collider(mut collider: Query<&ActiveEvents, With<Player>>) {
    for active_event in collider.iter_mut() {
        println!("{:?}", active_event);
    }
}

pub fn lose_health(
    mut health: ResMut<Health>,
    mob: Query<&Mob>,
    mut collision_events: EventReader<CollisionEvent>,
) {
    for (_e1, e2) in collision_events
        .iter()
        .filter_map(|event| {
            if let CollisionEvent::Started(e1, e2, _) = event {
                Some([(e1, e2), (e2, e1)])
            } else {
                None
            }
        })
        .flatten()
    {
        let contact_with_mob: bool;

        // is entity 2 a mob?
        if let Ok(_mob) = mob.get(*e2) {
            //  print!("contactWithMob= true");
            contact_with_mob = true;
        } else {
            contact_with_mob = false;
            //  print!("contactWithMob= false")
        }

        if contact_with_mob == true {
            health.value -= 1;
            //  print!("lose health")
        }
    }
}

pub fn init_system(mut commands: Commands) {
    commands
        .spawn(RigidBody::KinematicPositionBased)
        .insert(Collider::ball(0.5))
        .insert(KinematicCharacterController::default());
}

pub fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
    for (entity, output) in controllers.iter() {
        println!(
            "Entity {:?} moved by {:?} and touches the ground: {:?}",
            entity, output.effective_translation, output.grounded
        );
    }
}
