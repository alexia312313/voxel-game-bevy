use bevy::prelude::*;

use crate::game::ui::health::plugin::UiHealthPlugin;
use crate::game::resources::*;
use crate::game::ui::update::*;

pub struct UiPlugin;


impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            //resources
            .init_resource::<Score>()
            //plugins
            .add_plugin(UiHealthPlugin)
            //systems
            //.add_system(update_health_text)
            //.add_system(update_score_text)
            



            ;
    }
}
