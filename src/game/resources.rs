use bevy::prelude::*;

#[derive(Resource)]
pub struct Health {
    pub value: u32,
}

impl Default for Health {
    fn default() -> Health {
        Health { value: 10000 }
    }
}

#[derive(Resource)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Component)]
pub struct AnimationEntityLink(pub Entity);
