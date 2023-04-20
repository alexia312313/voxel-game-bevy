use bevy::prelude::*;

use crate::AppState;

use super::systems::interactions::*;
use super::systems::layout::*;
use super::option_menu::layout::*;


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
                    interact_with_options_button,
                )
                    .in_set(OnUpdate(AppState::MainMenu)),
            )
            // OnExit State Systems
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}

pub struct OptionMenuPlugin;

impl Plugin for OptionMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_options_menu.in_schedule(OnEnter(AppState::OptionsMenu)))
        //.add_systems(().in_set(OnUpdate(AppState::OptionsMenu)),)
        .add_system(despawn_options_menu.in_schedule(OnExit(AppState::OptionsMenu)));
    }
}