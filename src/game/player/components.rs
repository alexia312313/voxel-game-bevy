use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct PlayerModel;

#[derive(Default, Component)]
pub struct PlayerController {
    pub yaw: Quat,
    pub pitch: Quat,
    pub cursor_locked: bool,
}

#[derive(Component)]
pub struct AnimationEntityLink(pub Entity);
