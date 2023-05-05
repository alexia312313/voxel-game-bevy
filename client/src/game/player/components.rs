use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub mode: i32,
}

#[derive(Component)]
pub struct PlayerModel;

#[derive(Component)]
pub struct WeaponModel;

#[derive(Default, Component)]
pub struct PlayerController {
    pub yaw: Quat,
    pub pitch: Quat,
    pub cursor_locked: bool,
}


#[derive(Component)]
pub struct WeaponCollider;
