use bevy::prelude::*;

#[derive(Resource)]
pub struct MobAnimations(pub Vec<Handle<AnimationClip>>);

#[derive(Resource)]
pub struct MobHealth {
    pub value: u32,
}

impl Default for MobHealth {
    fn default() -> MobHealth {
        MobHealth { value: 3 }
    }
}
