use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(move_player)
        //.add_system(rotate_camera)
        .add_system(setup_scene_once_loaded)
        //.add_system(move_scene_entities)
        //.add_system(rotate_model)
        .run();
}

#[derive(Component)]
pub struct Player {
    speed: f32,
}

#[derive(Component)]
struct PlayerModel;

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);

#[derive(Default, Component)]
pub struct PlayerController {
    yaw: Quat,
    pitch: Quat,
    cursor_locked: bool,
}

fn setup(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(Animations(vec![ass.load("mereo.gltf#Animation0")]));

    commands.spawn(PrimaryWindow::default());
    commands.spawn(SceneBundle {
        scene: ass.load("slime.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9999.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9999.0,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(-4.0, 8.0, -4.0),
        ..default()
    });
    commands.spawn(PlayerController::default());

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(50.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Collider::cuboid(50.0, 0.1, 50.0));

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

fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            player.play(animations.0[0].clone_weak()).repeat();
            *done = true;
        }
    }
}

fn move_scene_entities(
    moved_scene: Query<Entity, With<Player>>,
    children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
    camera: Query<&Camera3d>,
) {
    for moved_scene_entity in moved_scene.iter() {
        if let Ok(child_entities) = children.get(moved_scene_entity) {
            for child_entity in child_entities.iter() {
                if camera.get(*child_entity).is_err() {
                    // This child_entity is NOT a Camera3d, it's just an Entity
                    if let Ok(mut transform) = transforms.get_mut(*child_entity) {
                        transform.rotation = Quat::from_rotation_y(0.05) * transform.rotation;
                    }
                }
            }
        }
    }
}

fn rotate_model(time: Res<Time>, mut player_query: Query<(&mut Transform, &PlayerModel)>) {
    for (mut transform, player) in player_query.iter_mut() {
        transform.rotation *= Quat::from_rotation_y(time.elapsed_seconds() / 2.);
    }
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<Entity, With<Player>>,
    children: Query<&Children>,
    mut transforms: Query<&mut Transform>,
    camera: Query<&Camera3d>,
    time: Res<Time>,
) {
    for player in player_query.iter() {
        if let Ok(child_entities) = children.get(player) {
            for child_entity in child_entities.iter() {
                if let Ok(mut transform) = transforms.get_mut(*child_entity) {
                    let mut direction = Vec3::ZERO;
                    let mut jump = Vec3::ZERO;

                    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
                        direction -= Vec3::new(0.1, 0.0, 0.0);
                    }
                    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D)
                    {
                        direction += Vec3::new(0.1, 0.0, 0.0);
                    }
                    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
                        direction += Vec3::new(0.0, 0.0, 0.1);
                    }
                    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
                        direction -= Vec3::new(0.0, 0.0, 0.1);
                    }
                    if keyboard_input.pressed(KeyCode::Space) {
                        jump += Vec3::new(0.0, 1.0, 0.0);
                    }

                    if direction.length() > 0.0 {
                        direction = direction.normalize();
                    }

                    transform.translation += direction * 3.0 * time.delta_seconds();
                    transform.translation += jump * 3.0 * time.delta_seconds();

                    if camera.get(*child_entity).is_err() {
                        transform.rotate_y(direction.x)
                    }
                }
            }
        }
    }
}

fn rotate_camera(
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
