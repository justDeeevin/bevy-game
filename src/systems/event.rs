use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    components::{
        bird::{jump, Bird},
        pipe::{spawn_pair, Pipe},
    },
    GameState, SpawnTimer,
};

pub fn start(
    mut body: Query<&mut RigidBody, With<Bird>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut velocity: Query<&mut LinearVelocity, With<Bird>>,
) {
    debug!("Starting");
    *body.single_mut() = RigidBody::Dynamic;
    spawn_pair(&mut commands, window.single());
    jump(&mut velocity.single_mut());
}

pub fn kill(
    mut body: Query<&mut RigidBody, With<Bird>>,
    mut velocities: Query<&mut LinearVelocity, With<Pipe>>,
) {
    debug!("Killing");
    *body.single_mut() = RigidBody::Static;
    for mut v in &mut velocities {
        *v = LinearVelocity(Vec2::ZERO);
    }
}

pub fn clear(
    pipes: Query<Entity, With<Pipe>>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut transform: Query<&mut Transform, With<Bird>>,
    mut timer: ResMut<SpawnTimer>,
) {
    for pipe in &pipes {
        commands.entity(pipe).despawn();
    }

    next_state.set(GameState::Playing);

    transform.single_mut().translation = Vec3::ZERO;
    timer.reset();
}
