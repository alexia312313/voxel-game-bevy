use bevy::prelude::*;

use super::{
    mob::plugin::MobPlugin, player::plugin::PlayerPlugin, ui::plugin::UiPlugin,
    world::plugin::WorldPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldPlugin)
            .add_plugin(UiPlugin)
            .add_plugin(MobPlugin)
            .add_plugin(PlayerPlugin);
    }
}
