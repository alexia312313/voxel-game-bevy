use bevy::prelude::*;

use crate::AppState;

use super::systems::interactions::*;
use super::systems::layout::*;
use super::settings::layout::*;


pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // OnEnter State Systems
            .add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            // Systems
            .add_systems(
                (
                    interact_with_play_button,
                    interact_with_quit_button,
                    interact_with_settings_button,
                )
                    .in_set(OnUpdate(AppState::MainMenu)),
            )
            // OnExit State Systems
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_options_menu.in_schedule(OnEnter(AppState::Settings)))
        //.add_systems(().in_set(OnUpdate(AppState::Settings)),)
        .add_system(despawn_options_menu.in_schedule(OnExit(AppState::Settings)));
    }
}