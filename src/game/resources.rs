use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Health{
    pub value: u32,
}

impl Default for Health{
    fn default()-> Health{
        Health{
            value:3
        }
    }
}

#[derive(Resource)]
pub struct Score{
    pub value: u32,
}

impl Default for Score{
    fn default()-> Score{
        Score{
            value:0
        }
    }
}