use std::{
    net::{SocketAddr, UdpSocket},
    time::{Duration, SystemTime},
};

use bevy::{transform::{commands, self}, prelude::{ResMut, Resource}};
use bevy_renet::renet::*;
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use bevy::prelude::*;
use crate::Transform;
use crate::Vec3;
use bevy::math::Quat;
use crate::Commands;
use crate::PbrBundle;
use bevy::render::color::Color;
use bevy::log::LogPlugin;

use common::player::things::{
    Player,
    PlayerInput,
    PlayerCommand,
    ClientChannel,
    ServerChannel,
    ServerMessages,
    NetworkedEntities,
    PROTOCOL_ID,
    PRIVATE_KEY,
};


#[derive(Debug, Serialize, Deserialize)]
#[derive(Default, Resource)]
struct NetworkMapping(HashMap<Entity, Entity>);


#[derive(Debug)]
struct PlayerInfo {
    client_entity: Entity,
    server_entity: Entity,
}


#[derive(Debug, Default, Resource)]
struct ClientLobby {
    players: HashMap<u64, PlayerInfo>,
}

pub fn create_renet_client() -> RenetClient {
    let current_time: Duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let client_id: u64 = current_time.as_millis() as u64;
    let connection_config: RenetConnectionConfig = RenetConnectionConfig::default();
    let server_addr: SocketAddr = SocketAddr::new(local_ip().unwrap(), 7010);
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };

    RenetClient::new(current_time, socket, connection_config, authentication).unwrap()
}



fn client_sync_players(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut lobby: ResMut<ClientLobby>,
    mut network_mapping: ResMut<NetworkMapping>,
    mut playerQuery: Query<(Entity, &Player, &Transform)>,
){ 
    let client_id = client.client_id();
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerJoined { id } => {
                let (entity, player, transform) = playerQuery.get_mut(lobby.players[&id].client_entity).unwrap();
            }
            ServerMessages::PlayerRemove { id } => todo!(),
        }
    
    }
}
