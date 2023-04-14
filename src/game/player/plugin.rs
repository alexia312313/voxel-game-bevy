use bevy::prelude::*;

use super::player::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(move_player)
            .add_system(rotate_camera);
    }
}
