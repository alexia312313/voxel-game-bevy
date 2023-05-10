use std::{
    net::{SocketAddr, UdpSocket},
    time::{Duration, SystemTime},
};

use crate::Commands;
use crate::PbrBundle;
use crate::Transform;
use crate::Vec3;
use bevy::log::LogPlugin;
use bevy::math::Quat;
use bevy::render::color::Color;
use bevy::{
    app::AppExit,
    prelude::*,
    transform::{self, commands},
};
use bevy_renet::{
    renet::{ClientAuthentication, RenetClient, RenetError},
    RenetClientPlugin,
};
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use common::player::things::{
    client_connection_config, ClientChannel, NetworkedEntities, Player, PlayerCommand, PlayerInput,
    ServerChannel, ServerMessages, PRIVATE_KEY, PROTOCOL_ID,
};

#[derive(Debug, Serialize, Deserialize, Default, Resource)]
struct NetworkMapping(HashMap<Entity, Entity>);

#[derive(Debug)]
struct PlayerInfo {
    client_entity: Entity,
    server_entity: Entity,
}

pub fn create_renet_client() -> RenetClient {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let connection_config = client_connection_config();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    RenetClient::new(current_time, socket, connection_config, authentication).unwrap()
}

pub fn client_sync_players(
    mut PlayerCommand: EventWriter<PlayerCommand>,
    mut client: ResMut<RenetClient>,
    mut commands: Commands,
) {
    let client_id = client.client_id();
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerJoined { id } => {
                println!("Player joined: {}", id);
                let messagetosend = format!("Player joined from Cli :D :D :D : {}", id);
                PlayerCommand.send({
                    PlayerCommand::InfoDump {
                        string: messagetosend,
                    }
                });
            }
            ServerMessages::PlayerRemove { id } => todo!(),
        }
    }
}
pub fn client_send_player_commands(
    mut player_commands: EventReader<PlayerCommand>,
    mut client: ResMut<RenetClient>,
) {
    for command in player_commands.iter() {
        let command_message = bincode::serialize(command).unwrap();
        client.send_message(ClientChannel::Command, command_message);
    }
}

pub fn disconnect_on_exit(exit: EventReader<AppExit>, mut client: ResMut<RenetClient>) {
    if !exit.is_empty() && client.is_connected() {
        client.disconnect();
    }
}
