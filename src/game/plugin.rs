use bevy::prelude::*;

use super::systems::*;
use super::{
    mob::plugin::MobPlugin, player::plugin::PlayerPlugin, ui::plugin::UiPlugin,
    world::plugin::WorldPlugin,
};

pub struct GamePlugin;

use crate::AppState;
use crate::GameOver;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldPlugin)
            .add_plugin(UiPlugin)
            // Events
            .add_event::<GameOver>()
            // States
            .add_state::<SimulationState>()
            // OnEnter Systems
            .add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            .add_plugin(MobPlugin)
            .add_plugin(PlayerPlugin)
            // Systems
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)))
            // Exit State Systems
            .add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
