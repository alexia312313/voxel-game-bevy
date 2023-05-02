use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

use game::plugin::GamePlugin;
use main_menu::plugin::{MainMenuPlugin, SettingsPlugin};

use bevy::window::WindowMode;

pub mod game;
pub mod main_menu;
mod systems;

use systems::*;
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Voxel game".into(),
                        mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    filter: "info,wgpu_core=warn,wgpu_hal=off,rechannel=warn".into(),
                    level: bevy::log::Level::DEBUG,
                }),
        )
        .add_state::<AppState>()
        .add_state::<CamState>()
        // Asset Loading
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Next),
        )
        .add_collection_to_loading_state::<_, MyAssets>(GameState::AssetLoading)
        // Plugins
        //   .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        // My Plugins
        .add_plugin(MainMenuPlugin)
        .add_plugin(SettingsPlugin)
        .add_plugin(GamePlugin)
        // Systems
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(transition_to_options_state)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Settings,
    Game,
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum CamState {
    #[default]
    CamFirst,
    CamThird,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    AssetLoading,
    Next,
}

#[derive(AssetCollection, Resource)]
pub struct MyAssets {
    #[asset(path = "mereo.gltf#Scene0")]
    player: Handle<Scene>,
    #[asset(path = "purple-sword.gltf#Scene0")]
    sword: Handle<Scene>,
    #[asset(path = "slime.gltf#Scene0")]
    slime: Handle<Scene>,
    #[asset(path = "mereo.gltf#Animation0")]
    player_animation_hit: Handle<AnimationClip>,
    #[asset(path = "mereo.gltf#Animation2")]
    player_animation_walking: Handle<AnimationClip>,
    #[asset(path = "mereo.gltf#Animation1")]
    player_animation_idle: Handle<AnimationClip>,
    #[asset(path = "slime.gltf#Animation0")]
    slime_animation_walking: Handle<AnimationClip>,
}
