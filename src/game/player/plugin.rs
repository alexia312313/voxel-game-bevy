use bevy::prelude::*;

use super::systems::*;

use crate::game::plugin::SimulationState;
use crate::AppState;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter State
            .add_system(setup.in_schedule(OnEnter(AppState::Game)))
            // Systems
            .add_systems(
                (move_player, rotate_camera, link_animations).in_set(OnUpdate(AppState::Game)),
            );
    }
}
