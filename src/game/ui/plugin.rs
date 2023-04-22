use bevy::prelude::*;

use crate::game::ui::health::plugin::UiHealthPlugin;

pub struct UiPlugin;


impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(UiHealthPlugin);
    }
}
