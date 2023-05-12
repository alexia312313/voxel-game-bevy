use crate::game::connection::components::AllyModel;
use crate::Commands;
use crate::MyAssets;
use crate::Transform;
use bevy::math::Quat;
use bevy::{app::AppExit, prelude::*};
use bevy_rapier3d::prelude::*;
use bevy_renet::renet::{ClientAuthentication, RenetClient};
use common::player::things::{
    client_connection_config, ClientChannel, Player, PlayerCommand, ServerChannel, ServerMessages,
    PROTOCOL_ID,
};
use core::f32::consts::PI;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{net::UdpSocket, time::SystemTime};
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
    mut playerQuery: Query<(&Player, &Transform)>,
    _my_assets: Res<MyAssets>,
) {
    let client_id = client.client_id();
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();

        match server_message {
            ServerMessages::PlayerJoined { id } => {
                println!("Player joined: {}", id);
                let messagetosend = format!("Player joined from Cli :D :D :D : {}", id);
                let (mut player, pos) = playerQuery.get_single_mut().unwrap();
                println!("Pos: {:?}", pos);
                let client_entity = commands
                    .spawn(SceneBundle { ..default() })
                    .insert(RigidBody::Dynamic)
                    .insert(GravityScale(1.0))
                    .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
                    .insert(Name::new("Ally"))
                    .with_children(|parent| {
                        parent
                            .spawn((
                                SceneBundle {
                                    scene: _my_assets.player.clone_weak(),
                                    transform: Transform::from_rotation(Quat::from_rotation_y(
                                        PI / 2.0,
                                    )),
                                    ..default()
                                },
                                AllyModel,
                            ))
                            .insert(Name::new("Asset ally player"));
                    });
                let messagetosend = format!("Player joined from Cli :D :D :D : {}", id);
                PlayerCommand.send(PlayerCommand::InfoDump {
                    string: messagetosend,
                });
            }
            ServerMessages::PlayerRemove { id } => {
                println!("Player removed: {}", id);
            }
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
