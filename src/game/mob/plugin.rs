use bevy::prelude::*;
use big_brain::prelude::*;


use super::mob::*;

pub struct MobPlugin;

impl Plugin for MobPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(aggro_system)
            .add_system(aggro_action_system.in_set(BigBrainSet::Actions))
            .add_system(aggro_scorer_system.in_set(BigBrainSet::Scorers));
    }
}
