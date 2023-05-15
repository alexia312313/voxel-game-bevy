
use bevy::prelude::*;

use super::systems::*;

use crate::{AppState, game::materials::mob_materials::CustomStandardMaterial};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            // On Enter State
            .add_systems((
                setup, 
                init_system,
            ).in_schedule(OnEnter(AppState::Game)))
            //Add plugins
            .add_plugin(MaterialPlugin::<CustomStandardMaterial>::default())
            // Systems
            .add_systems(
                (
                    move_player,
                    change_cam,
                    rotate_camera,
                    equip_weapon,
                    lose_health,
                    attack_sword,
                    mob_red, 
                    swap_mob_material
                )
                    .in_set(OnUpdate(AppState::Game)),
            );
    }
}


