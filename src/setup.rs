use bevy::{prelude::*, window::PrimaryWindow};

pub fn window_start(mut commands: Commands) {
    commands.spawn(PrimaryWindow::default());
}
