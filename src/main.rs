use bevy::{prelude::*, window::PrimaryWindow};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DebugCursorPickingPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        //.add_system(click_to_move)
        .run();
}

#[derive(Component)]
pub struct Player {
    destination: Vec2,
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

    // Spawn the primary window
    commands.spawn(PrimaryWindow::default());

    // Spawn the scene
    commands.spawn((
        SceneBundle {
            scene: my_gltf,
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        Player {
            destination: Vec2::ZERO,
            speed: 20.0,
        },
    ));
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
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(50.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(PickableBundle::default());
    // camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(PickingCameraBundle::default());
}

/* TODO Move the player towards the cursor position when the right mouse button is pressed
fn click_to_move(
    mut events: EventReader<PickingEvent>,
    mut player: Query<(&mut Transform, &mut Player)>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    btn: Res<Input<MouseButton>>,
    time: Res<Time>,
) {

    for (mut transform, mut player) in player.iter_mut() {
        // If the right mouse button is pressed, set the destination to the cursor position
        if btn.just_pressed(MouseButton::Right) {
            if let Ok(window) = primary_window.get_single() {
                player.destination = window.cursor_position().unwrap().normalize();
                println!("Destination: {:?}", player.destination);
                println!("Transform: {:?}", transform.translation);
            };
        }
        // Move the player towards the destination

        if transform.translation.x > player.destination.x {
            transform.translation.x -= player.speed * time.delta_seconds();
        }
        if transform.translation.x < player.destination.x {
            transform.translation.x += player.speed * time.delta_seconds();
        }
        if transform.translation.y > player.destination.y {
            transform.translation.y -= player.speed * time.delta_seconds();
        }
        if transform.translation.y < player.destination.y {
            transform.translation.y += player.speed * time.delta_seconds();
        }

    }

}
*/
