use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    components::{
        bird::{jump, Bird},
        pipe::{spawn_pair, Pipe, Scorer},
    },
    GameState, MaxGap, Score, SpawnTimer,
};

pub fn start(
    mut body: Query<&mut RigidBody, With<Bird>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut velocity: Query<&mut LinearVelocity, With<Bird>>,
    max_gap: Res<MaxGap>,
) {
    debug!("Starting");
    *body.single_mut() = RigidBody::Dynamic;
    let window = window.single();
    spawn_pair(&mut commands, window, **max_gap);
    let window_size = window.size();
    jump(&mut velocity.single_mut());
    commands
        .spawn(Collider::rectangle(0.0, 0.0))
        .insert(Transform::from_xyz(0.0, (-window_size.y / 2.0) - 1.0, 0.0))
        .insert(RigidBody::Static);
    commands
        .spawn(Collider::rectangle(0.0, 0.0))
        .insert(Transform::from_xyz(0.0, window_size.y / 2.0, 0.0))
        .insert(RigidBody::Static);
}

pub fn kill(
    mut body: Query<&mut RigidBody, With<Bird>>,
    mut velocities: Query<&mut LinearVelocity, Or<(With<Pipe>, With<Scorer>)>>,
) {
    debug!("Killing");
    *body.single_mut() = RigidBody::Static;
    for mut v in &mut velocities {
        *v = LinearVelocity(Vec2::ZERO);
    }
}

pub fn clear(
    to_delete: Query<Entity, (Without<Bird>, With<Collider>)>,
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut transform: Query<&mut Transform, With<Bird>>,
    mut timer: ResMut<SpawnTimer>,
    mut score: ResMut<Score>,
    mut max_gap: ResMut<MaxGap>,
) {
    debug!("Clearing");
    for pipe in &to_delete {
        commands.entity(pipe).despawn();
    }

    next_state.set(GameState::Playing);

    let mut transform = transform.single_mut();
    transform.translation = Vec3::ZERO;
    transform.rotation = Quat::IDENTITY;
    timer.reset();
    **score = 0;
    **max_gap = 700.0;
}
