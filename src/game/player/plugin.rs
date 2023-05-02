use bevy::prelude::*;

use super::systems::*;


use crate::AppState;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter State
            .add_system(setup.in_schedule(OnEnter(AppState::Game)))
            .add_system(init_system.in_schedule(OnEnter(AppState::Game)))

            // Systems

            .add_systems(
                (
                    move_player,
                    change_cam,
                    rotate_camera,
                    equip_weapon,
                    check_collider,
                    read_result_system,
                   
                    lose_health,
                )
                    .in_set(OnUpdate(AppState::Game)),

            );
    }
}
