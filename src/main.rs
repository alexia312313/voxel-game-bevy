use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_system(move_player)
        .run();
}

#[derive(Component)]
pub struct Player {
    speed: f32,
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // note that we have to include the `Scene0` label
    let my_gltf = ass.load("mereo.gltf#Scene0");
    let slime = ass.load("slime.gltf#Scene0");

    // Spawn the primary window
    commands.spawn(PrimaryWindow::default());

    // Spawn the scene
    commands.spawn((
        SceneBundle {
            scene: my_gltf,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Player { speed: 20.0 },
    ));
    commands.spawn((SceneBundle {
        scene: slime,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    },));
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub const PLAYER_SPEED: f32 = 5.0;
fn  move_player (
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
){
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-0.1, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(0.1, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 0.0, 0.1);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, 0.0, -0.1);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}