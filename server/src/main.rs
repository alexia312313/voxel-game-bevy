use bevy::{ecs::event, log::LogPlugin};

use bevy_renet::{
    renet::{RenetServer, ServerConfig},
    RenetServerPlugin,
};
use local_ip_address::local_ip;

pub use bevy::prelude::*;
pub use bevy_renet::renet::*;
pub use bevy_renet::*;
use serde::{Deserialize, Serialize};

use common::player::things::{
    server_connection_config, ClientChannel, NetworkedEntities, Player, PlayerCommand, PlayerInput,
    ServerChannel, ServerMessages, PRIVATE_KEY, PROTOCOL_ID,
};

use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    process::id,
    time::{Duration, SystemTime},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,wgpu_core=warn,wgpu_hal=off,rechannel=warn".into(),
            level: bevy::log::Level::DEBUG,
        }))
        .insert_resource(create_renet_server())
        .add_plugin(RenetServerPlugin::default())
        .add_system(server_events)
        .add_system(server_update_system)
        .run();
}

fn create_renet_server() -> RenetServer {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = server_connection_config();
    let server_config =
        ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut server: ResMut<RenetServer>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _user_data) => {
                info!("CONNECTED! update_systems{}!", id);
                let message: Vec<u8> =
                    bincode::serialize(&ServerMessages::PlayerJoined { id: *id }).unwrap();
                server.send_message(*id, ServerChannel::ServerMessages, message);
            }
            ServerEvent::ClientDisconnected(id) => {
                info!("DISCONNECTED :C {}!", id);
            } //Event receiveMessage
        }
    }
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Command) {
            println!("ENTREM EN EL MATCH");
            let command: PlayerCommand = bincode::deserialize(&message).unwrap();
            match command {
                PlayerCommand::InfoDump { string } => {
                    println!("{}: {}", client_id, string);
                }
            }
        }
    }
}
fn server_events(mut events: EventReader<ServerEvent>, mut server: ResMut<RenetServer>) {
    for event in events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _user_data) => {
                info!("CONNECTED! server_events {}!", id)
            }
            ServerEvent::ClientDisconnected(id) => info!("DISCONNECTED :C {}!", id),
        }
    }
    for client_id in server.clients_id().into_iter() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Command) {
            let command: PlayerCommand = bincode::deserialize(&message).unwrap();
            match command {
                PlayerCommand::InfoDump { string } => {
                    println!("{}: {}", client_id, string);
                }
            }
        }
    }
}
