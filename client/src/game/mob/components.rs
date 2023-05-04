use bevy::prelude::*;
use big_brain::prelude::*;

#[derive(Component)]
pub struct Mob {}

#[derive(Component, Debug)]
pub struct Aggro {
    pub per_second: f32,
    pub aggro: f32,
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Attack {
    pub until: f32,
    pub per_second: f32,
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Aggroed;
