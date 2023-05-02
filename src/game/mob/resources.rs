use bevy::prelude::*;

#[derive(Resource)]
pub struct MobAnimations(pub Vec<Handle<AnimationClip>>);
