use bevy::prelude::*;

use super::ui::health::*;
use crate::AppState;

pub struct UiPlugin;


impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(UiHealthPlugin)
    }
}
