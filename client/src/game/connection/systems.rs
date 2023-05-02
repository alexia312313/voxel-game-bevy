use std::{
    net::{SocketAddr, UdpSocket},
    time::{Duration, SystemTime},
};

use bevy_renet::renet::*;
use local_ip_address::local_ip;

const PROTOCOL_ID: u64 = 1000;

pub fn create_renet_client() -> RenetClient {
    let current_time: Duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();

    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0").unwrap();

    println!("");
    println!("");
    println!("You really renet client huh?");
    println!("");
    println!("");

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
