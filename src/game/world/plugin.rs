use bevy::prelude::*;

use super::world::*;
use crate::AppState;
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter State
            .add_system(setup.in_schedule(OnEnter(AppState::Game)))
            .add_system(animate_light_direction.in_set(OnUpdate(AppState::Game)));
    }
}
