use bevy::prelude::*;
use bevy_renet::RenetClientPlugin;
use common::player::things::PlayerCommand;

use super::systems::*;
pub struct ClientPlugin;

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RenetClientPlugin::default())
            .insert_resource(create_renet_client());
        app.add_event::<PlayerCommand>();
        app.add_system(client_sync_players)
            .add_system(client_send_player_commands);
        //.add_system(disconnect_on_exit);
    }
}
