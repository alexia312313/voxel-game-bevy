use bevy::prelude::*;

use crate::game::resources::{Score,Health};

use crate::game::ui::components::{ScoreText,HealthText};

pub fn update_score_text(
    mut text_query: Query<&mut Text, With<ScoreText>>, 
    mut score: ResMut<Score>,
    keyboard_input:Res<Input<KeyCode>>,
) {

    if score.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", score.value.to_string());
        }
    }

     //testing

    if keyboard_input.just_pressed(KeyCode::Q) {
        println!("Score: {:?}", score.value.to_string());
     
     //working lol idk why red its not private 

        score.value +=1;
     }
}

pub fn update_health_text(
    mut text_query: Query<&mut Text, With<HealthText>>,
    mut health: ResMut<Health>,
    keyboard_input:Res<Input<KeyCode>>,
) {

    if health.is_changed() {
        for mut text in text_query.iter_mut() {
            text.sections[0].value = format!("{}", health.value.to_string());
        }
    }

     //testing
     if keyboard_input.just_pressed(KeyCode::Q) {
        println!("Health: {:?}", health.value.to_string());

     //working lol idk why red its not private 
     health.value +=1;
     }
}

