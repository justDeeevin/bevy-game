use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    components::{
        bird::{jump, Bird},
        pipe::spawn_pair,
        ui::ScoreText,
    },
    GameState, Score, SpawnTimer,
};

pub fn try_jump(
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<&mut LinearVelocity, With<Bird>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if !(keyboard.any_just_pressed([KeyCode::Space, KeyCode::ArrowUp])
        || mouse.just_pressed(MouseButton::Left))
    {
        return;
    }

    if **state != GameState::Start {
        jump(&mut query.single_mut());
    }

    match **state {
        GameState::Start => next_state.set(GameState::Playing),
        GameState::Dead => next_state.set(GameState::Clearing),
        _ => {}
    }
}

pub fn try_spawn_pipe(
    mut timer: ResMut<SpawnTimer>,
    mut commands: Commands,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if !timer.tick(time.delta()).just_finished() {
        return;
    }
    let window = window_query.single();
    spawn_pair(&mut commands, window);
}

pub fn check_collisions(
    mut commands: Commands,
    mut events: EventReader<Collision>,
    mut next_state: ResMut<NextState<GameState>>,
    mut score: ResMut<Score>,
) {
    for Collision(contacts) in events.read() {
        if contacts.is_sensor {
            commands.entity(contacts.entity2).despawn();
            **score += 1;
        } else {
            next_state.set(GameState::Dead);
        }
    }
}

pub fn update_score_text(score: Res<Score>, mut text: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        text.single_mut().0 = format!("Score: {}", **score);
    }
}

pub fn try_exit(
    keyboard: Res<ButtonInput<KeyCode>>,
    window: Query<Entity, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        commands.entity(window.single()).despawn();
    }
}
