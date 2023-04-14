use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use big_brain::BigBrainPlugin;

pub mod game;
pub mod setup;

use setup::window_start;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(BigBrainPlugin)
        .add_plugin(game::plugin::GamePlugin)
        .add_startup_system(window_start)
        .run();
}
