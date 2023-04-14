use std::thread::spawn;
use big_brain::prelude::*;
use bevy::utils::tracing::{debug, trace};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;


#[derive(Component, Debug)]
pub struct Aggro {
    pub per_second: f32,
    pub aggro: f32,
}

impl Aggro {
    pub fn new(aggro: f32, per_second: f32) -> Self {
        Self { aggro, per_second }
    }
}

pub fn aggro_system(time: Res<Time>, mut aggros: Query<&mut Aggro>) {
    for mut aggro in &mut aggros {
        aggro.aggro += aggro.per_second * (time.delta().as_micros() as f32 / 1_000_000.0);
        if aggro.aggro >= 100.0 {
            aggro.aggro = 100.0;
        }
        trace!("Aggro: {}", aggro.aggro);
    }
}

#[derive(Clone, Component, Debug, ActionBuilder)]
pub struct Attack {
    until: f32,
    per_second: f32,
}

pub fn aggro_action_system(
    time: Res<Time>,
    mut aggros: Query<&mut Aggro>,
    // We execute actions by querying for their associated Action Component
    // (Drink in this case). You'll always need both Actor and ActionState.
    mut query: Query<(&Actor, &mut ActionState, &Attack, &ActionSpan)>,
) {
    for (Actor(actor), mut state, attack, span) in &mut query {
        // This sets up the tracing scope. Any `debug` calls here will be
        // spanned together in the output.
        let _guard = span.span().enter();

        // Use the attack_action's actor to look up the corresponding Aggro Component.
        if let Ok(mut aggro) = aggros.get_mut(*actor) {
            match *state {
                ActionState::Requested => {
                    print!("Time to attack some player!");
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    trace!("Attacking...");
                    aggro.aggro -=
                        attack.per_second * (time.delta().as_micros() as f32 / 1_000_000.0);
                    if aggro.aggro <= attack.until {
                        // To "finish" an action, we set its state to Success or
                        // Failure.
                        print!("Done attacking player!");
                        *state = ActionState::Success;
                    }
                }
                // All Actions should make sure to handle cancellations!
                ActionState::Cancelled => {
                    print!("Action was cancelled. Considering this a failure.");
                    *state = ActionState::Failure;
                } _ => {}
            }
        }

    }
}

#[derive(Clone, Component, Debug, ScorerBuilder)]
pub struct Aggroed;

// Looks familiar? It's a lot like Actions!
pub fn aggro_scorer_system(
    aggros: Query<&Aggro>,
    // Same dance with the Actor here, but now we use look up Score instead of ActionState.
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<Aggroed>>,
) {
    for (Actor(actor), mut score, span) in &mut query {
        if let Ok(aggro) = aggros.get(*actor) {
            // This is really what the job of a Scorer is. To calculate a
            // generic "Utility" score that the Big Brain engine will compare
            // against others, over time, and use to make decisions. This is
            // generally "the higher the better", and "first across the finish
            // line", but that's all configurable using Pickers!
            //
            // The score here must be between 0.0 and 1.0.
            score.set(aggro.aggro / 100.0);
            if aggro.aggro >= 80.0 {
                span.span().in_scope(|| {
                    print!("Aggro above threshold! Score: {}", aggro.aggro / 100.0)
                });
            }
        }
    }
}

pub fn setup(
    mut commands: Commands,
    ass: Res<AssetServer>,
){
    print!("Hem creat un slime");
    commands.spawn(SceneBundle {
        scene: ass.load("slime.gltf#Scene0"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()        
    })
    .with_children(|parent| {
        parent.spawn((
            Aggro::new(75.0, 2.0),
            Thinker::build()
                .label("My Thinker")
                .picker(FirstToScore { threshold: 0.8 })
                // Technically these are supposed to be ActionBuilders and
                // ScorerBuilders, but our Clone impls simplify our code here.
                .when(
                    Aggroed,
                    Attack {
                        until: 70.0,
                        per_second: 5.0,
                    },
                ),
        ));
    });
}