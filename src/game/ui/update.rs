use bevy::prelude::*;
use bevy_inspector_egui::egui::Key;

use crate::game::resources::{Score,Health};

use crate::game::ui::components::{ScoreText,HealthText};

pub fn update_score_text(mut text_query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", score.value.to_string());
        }
    }
}

pub fn update_health_text(mut text_query: Query<&mut Text, With<HealthText>>, health: Res<Health>) {
    if health.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", health.value.to_string());
        }
    }
}

//for testing 

pub fn add_health_score(
    mut health_query: Query<&mut Text, With<HealthText>>,
    mut score_query: Query<&mut Text, With <ScoreText>>,
    keyboard_input:Res<Input<KeyCode>>,
){
    if keyboard_input.just_pressed(KeyCode::Q) {
   println!("Health: {:?}", health_query);
   println!("Score: {:?}", score_query)
}
}