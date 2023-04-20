use bevy::prelude::*;

use super::world::*;
use crate::AppState;
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::Game)));
    }
}
