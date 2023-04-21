use bevy::prelude::*;
use super ::health::layout::*;

pub struct UiHealthPlugin;

impl Plugin for UiHealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_ui_health.in_schedule(OnEnter(AppState::Game)))
            .add_system(despawn_ui_health.in_schedule(OnExit(AppState::Game)));
    }
}
