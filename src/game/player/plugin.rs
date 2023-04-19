use bevy::prelude::*;

use super::systems::*;

use crate::game::plugin::SimulationState;
use crate::AppState;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter State
            //.add_startup_system(setup)
            .add_system(setup.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (link_animations, testing_scheduler).in_schedule(OnEnter(SimulationState::Setup)),
            )
            // Systems
            .add_systems((move_player, rotate_camera).in_set(OnUpdate(SimulationState::Setup)));
    }
}

fn testing_scheduler() {
    println!("works!");
}
