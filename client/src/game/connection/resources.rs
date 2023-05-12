use bevy::animation::AnimationClip;
use bevy::asset::Handle;
use bevy::ecs::system::Resource;

#[derive(Resource)]
pub struct Animations1(pub Vec<Handle<AnimationClip>>);
