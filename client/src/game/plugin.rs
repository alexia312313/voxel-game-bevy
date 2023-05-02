use bevy::prelude::*;

use super::{
    connection::plugin::ClientPlugin, mob::plugin::MobPlugin, player::plugin::PlayerPlugin,
    systems::link_animations, ui::plugin::UiPlugin, world::plugin::WorldPlugin,
};

use big_brain::BigBrainPlugin;

use crate::AppState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .add_state::<SimulationState>()
            // OnEnter Systems
            //.add_system(pause_simulation.in_schedule(OnEnter(AppState::Game)))
            // My Plugins
            .add_plugin(WorldPlugin)
            .add_plugin(MobPlugin)
            .add_plugin(ClientPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(UiPlugin)
            .add_system(link_animations.in_set(OnUpdate(AppState::Game)))
            //external plugin
            .add_plugin(BigBrainPlugin);

        // Systems
        //.add_system(toggle_simulation.run_if(in_state(AppState::Game)))
        // Exit State Systems
        //.add_system(resume_simulation.in_schedule(OnExit(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
