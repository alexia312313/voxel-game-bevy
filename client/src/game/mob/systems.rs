use crate::game::resources::AnimationEntityLink;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{Collider, RigidBody};
use big_brain::prelude::*;

use super::components::*;
use super::resources::*;

use crate::{game::player::components::Player, MyAssets};

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
                    // print!("Time to attack some player!");
                    *state = ActionState::Executing;
                }
                ActionState::Executing => {
                    trace!("Attacking...");

                    aggro.aggro -=
                        attack.per_second * (time.delta().as_micros() as f32 / 1_000_000.0);
                    aggro.aggro = 200.0; //TEST
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
                }
                _ => {}
            }
        }
    }
}

// Looks familiar? It's a lot like Actions!
pub fn aggro_scorer_system(
    aggros: Query<&Aggro>,
    mobs: Query<Entity, With<Mob>>,
    player: Query<Entity, With<Player>>,
    mut transforms: Query<&mut Transform>,
    time: Res<Time>,
    mut animation_players: Query<&mut AnimationPlayer>,
    mut animation_link: Query<&AnimationEntityLink>,
    animations: Res<MobAnimations>,
    // Same dance with the Actor here, but now we use look up Score instead of ActionState.
    mut query: Query<(&Actor, &mut Score, &ScorerSpan), With<Aggroed>>,
    mut done: Local<bool>,
) {
    let mut index = 0;
    for animation_entity in animation_link.iter_mut() {
        index += 1;
        if index == 2 {
            if let Ok(mut player_animation) = animation_players.get_mut(animation_entity.0) {
                for (Actor(actor), mut score, span) in &mut query {
                    if let Ok(aggro) = aggros.get(*actor) {
                        // This is really what the job of a Scorer is. To calculate a
                        // generic "Utility" score that the Big Brain engine will compare
                        // against others, over time, and use to make decisions. This is
                        // generally "the higher the better", and "first across the finish
                        // line", but that's all configurable using Pickers!
                        //
                        // The score here must be between 0.0 and 1.0.
                        // We'll just use the aggro value directly.
                        score.set(aggro.aggro / 100.0);
                        if aggro.aggro >= 80.0 {
                            span.span().in_scope(|| {
                                //print!("{:?}" , mobs);
                                let mut player_pos = Vec3::ZERO;
                                if let Ok(creature) = player.get_single() {
                                    if let Ok(trans_player) = transforms.get(creature) {
                                        player_pos = trans_player.translation
                                    }
                                }
                                for mob in mobs.iter() {
                                    let mut direction = Vec3::ZERO;
                                    if let Ok(mut trans_mob) = transforms.get_mut(mob) {
                                        if player_pos.z < trans_mob.translation.z {
                                            direction -= Vec3::new(0.0, 0.0, 0.1);
                                        }
                                        if player_pos.z > trans_mob.translation.z {
                                            direction += Vec3::new(0.0, 0.0, 0.1);
                                        }
                                        if player_pos.x < trans_mob.translation.x {
                                            direction -= Vec3::new(0.1, 0.0, 0.0);
                                        }
                                        if player_pos.x > trans_mob.translation.x {
                                            direction += Vec3::new(0.1, 0.0, 0.0);
                                        }
                                        //if player_pos.y<trans_mob.translation.y{
                                        //    direction+=Vec3::new(0.0,0.1,0.0);
                                        //}
                                        //print!("{:?}", direction);

                                        if direction.length() > 0.0 {
                                            direction = direction.normalize();
                                            if !*done {
                                                player_animation
                                                    .play(animations.0[0].clone_weak())
                                                    .repeat();
                                                *done = true;
                                            }
                                        } else {
                                            //player_animation.stop_repeating();
                                            *done = false;
                                        }

                                        trans_mob.translation +=
                                            direction * 1.0 * time.delta_seconds();
                                    }
                                }
                            });
                        }
                    }
                }
            }
        }
    }
}

pub fn setup(mut commands: Commands, _my_assets: Res<MyAssets>) {
    commands.insert_resource(MobAnimations(vec![_my_assets
        .slime_animation_walking
        .clone_weak()]));

    commands
        .spawn(SceneBundle {
            scene: _my_assets.slime.clone_weak(),
            transform: Transform::from_xyz(5.0, 5.0, 0.0),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::cuboid(0.1, 0.1, 0.1))
        .insert(Mob {})
        .insert(Name::new("slime mob"))
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

pub fn mob_lose_health (
    mut mob_health: ResMut<MobHealth>,
    commands:  Commands,
    mob_query: Query<Entity, With<Mob>>, 
 ){
     if mob_health.value>0 {
         mob_health.value -=1;
     }

     if mob_health.value ==0{

         mob_despawn(commands, mob_query)
     }
 }

pub fn mob_despawn(
   mut commands: Commands,
    mob_query: Query<Entity, With<Mob>>,
) {
        for mob_entity in mob_query.into_iter() {
           commands.entity(mob_entity).despawn_recursive()
        }
    } 
