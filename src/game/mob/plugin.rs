use bevy::prelude::*;
use big_brain::prelude::*;

use crate::AppState;

use super::mob::*;

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::Game)))
            .add_system(aggro_system.in_set(OnUpdate(AppState::Game)))
            .add_system(aggro_action_system.in_set(BigBrainSet::Actions))
            .add_system(aggro_scorer_system.in_set(BigBrainSet::Scorers));
    }
}
