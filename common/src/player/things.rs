use bevy::ecs as bevy_ecs;
use bevy::ecs::entity::Entity;
use bevy::prelude::{shape::Icosphere, *};
use bevy_ecs::prelude::Component;
use bevy_rapier3d::prelude::*;
use bevy_renet::renet::{
    ChannelConfig, ReliableChannelConfig, RenetConnectionConfig, UnreliableChannelConfig,
    NETCODE_KEY_BYTES,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"an example very very secret key."; // 32-bytes
pub const PROTOCOL_ID: u64 = 7;

#[derive(Debug, Component)]
pub struct Player {
    pub id: u64,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum PlayerCommand {
    //BasicAttack { cast_at: Vec3 },
    InfoDump { string: String },
}

pub enum ClientChannel {
    Input,
    Command,
}

pub enum ServerChannel {
    ServerMessages,
    NetworkedEntities,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerJoined { id: u64 },
    PlayerRemove { id: u64 },
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NetworkedEntities {
    pub entities: Vec<Entity>,
    pub translations: Vec<[f32; 3]>,
}

impl From<ClientChannel> for u8 {
    fn from(channel_id: ClientChannel) -> Self {
        match channel_id {
            ClientChannel::Command => 0,
            ClientChannel::Input => 1,
        }
    }
}

impl ClientChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![
            ReliableChannelConfig {
                channel_id: Self::Input.into(),
                message_resend_time: Duration::ZERO,
                ..Default::default()
            }
            .into(),
            ReliableChannelConfig {
                channel_id: Self::Command.into(),
                message_resend_time: Duration::ZERO,
                ..Default::default()
            }
            .into(),
        ]
    }
}

impl From<ServerChannel> for u8 {
    fn from(channel_id: ServerChannel) -> Self {
        match channel_id {
            ServerChannel::NetworkedEntities => 0,
            ServerChannel::ServerMessages => 1,
        }
    }
}

impl ServerChannel {
    pub fn channels_config() -> Vec<ChannelConfig> {
        vec![
            UnreliableChannelConfig {
                channel_id: Self::NetworkedEntities.into(),
                sequenced: true, // We don't care about old positions
                ..Default::default()
            }
            .into(),
            ReliableChannelConfig {
                channel_id: Self::ServerMessages.into(),
                message_resend_time: Duration::from_millis(200),
                ..Default::default()
            }
            .into(),
        ]
    }
}

pub fn client_connection_config() -> RenetConnectionConfig {
    RenetConnectionConfig {
        send_channels_config: ClientChannel::channels_config(),
        receive_channels_config: ServerChannel::channels_config(),
        ..Default::default()
    }
}

pub fn server_connection_config() -> RenetConnectionConfig {
    RenetConnectionConfig {
        send_channels_config: ServerChannel::channels_config(),
        receive_channels_config: ClientChannel::channels_config(),
        ..Default::default()
    }
}
