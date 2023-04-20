use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;

pub mod game;
pub mod main_menu;
mod systems;

use systems::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        // My Plugins
        .add_plugin(main_menu::plugin::MainMenuPlugin)
        .add_plugin(game::plugin::GamePlugin)
        // Startup Systems
        .add_startup_system(window_start)
        // Systems
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .add_system(transition_to_options_state)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Options,
    Game,
    GameOver,
}

pub struct GameOver {
    pub score: u32,
}
