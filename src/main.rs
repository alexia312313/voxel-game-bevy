use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

use game::plugin::GamePlugin;
use main_menu::plugin::MainMenuPlugin;

pub mod game;
pub mod main_menu;
mod systems;

use systems::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        // Asset Loading
        .add_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading).continue_to_state(GameState::Next),
        )
        .add_collection_to_loading_state::<_, MyAssets>(GameState::AssetLoading)
        // Plugins
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        // My Plugins
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        // Systems
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(transition_to_options_state)
        .add_system(exit_game)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Options,
    Game,
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
    #[asset(path = "mereo.gltf#Animation0")]
    player_animation_hit: Handle<AnimationClip>,
    #[asset(path = "mereo.gltf#Animation2")]
    player_animation_walking: Handle<AnimationClip>,
    #[asset(path = "mereo.gltf#Animation1")]
    player_animation_idle: Handle<AnimationClip>,
}
