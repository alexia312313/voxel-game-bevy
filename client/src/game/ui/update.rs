use bevy::prelude::*;

use crate::game::resources::{Health, Score};

use crate::game::ui::components::{HealthText, ScoreText};

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

// testing
pub fn add_score_health(
    mut score: ResMut<Score>,
    mut health: ResMut<Health>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    //testing
    if keyboard_input.just_pressed(KeyCode::Q) {
        //println!("Health: {:?}", health.value.to_string());
        //println!("Score: {:?}", score.value.to_string());
        //working lol idk why red its not private
        health.value -= 1;
        score.value += 1;
    }
}
