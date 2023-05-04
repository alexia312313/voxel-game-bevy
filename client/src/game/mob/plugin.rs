use bevy::prelude::*;
use big_brain::prelude::*;

use super::{resources::MobHealth, systems::*};
use crate::AppState;

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(setup.in_schedule(OnEnter(AppState::Game)))
            .init_resource::<MobHealth>()
            .add_system(aggro_system.in_set(OnUpdate(AppState::Game)))
            .add_system(
                aggro_action_system
                    .run_if(in_state(AppState::Game))
                    .in_set(BigBrainSet::Actions),
            )
            .add_system(
                aggro_scorer_system
                    .run_if(in_state(AppState::Game))
                    .in_set(BigBrainSet::Scorers),
            );
    }
}
