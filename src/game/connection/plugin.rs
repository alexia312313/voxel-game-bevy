use bevy::prelude::*;
use bevy_renet::RenetClientPlugin;

use super::systems::*;

use crate::AppState;
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetClientPlugin::default())
            .insert_resource(create_renet_client());
    }
}
