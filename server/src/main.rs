use bevy::log::LogPlugin;

use bevy_renet::{
    renet::{RenetServer, ServerConfig},
    RenetServerPlugin,
};
use local_ip_address::local_ip;

pub use bevy::prelude::*;
pub use bevy_renet::renet::*;
pub use bevy_renet::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    Pong,
}

pub const PROTOCOL_ID: u64 = 1000;

use std::{
    net::{SocketAddr, UdpSocket},
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
        .run();
}

fn create_renet_server() -> RenetServer {
    let current_time: Duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let server_addr = SocketAddr::new(local_ip().unwrap(), 7010);
    print!("Creating Server {:?}", server_addr);
    let server_config =
        ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);

    let connection_config = RenetConnectionConfig::default();

    let inbound_server_addr = SocketAddr::new(local_ip().unwrap(), 7010);
    let socket = UdpSocket::bind(inbound_server_addr).unwrap();

    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

fn server_events(mut events: EventReader<ServerEvent>) {
    for event in events.iter() {
        match event {
            ServerEvent::ClientConnected(id, _user_data) => info!("CONNECTED! {}!", id),
            ServerEvent::ClientDisconnected(id) => info!("DISCONNECTED :C {}!", id),
        }
    }
}
