use bevy::prelude::*;

use crate::AppState;

use super::systems::interactions::*;
use super::systems::layout::*;

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
