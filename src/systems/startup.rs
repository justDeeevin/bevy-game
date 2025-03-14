use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::components::bird::Bird;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn spawn_bird(mut commands: Commands) {
    commands.spawn(Bird);
}

pub fn spawn_floor(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window_size = window.single().size();
    commands
        .spawn(Collider::rectangle(window_size.x, 0.0))
        .insert(Transform::from_xyz(0.0, (-window_size.y / 2.0) - 1.0, 0.0))
        .insert(RigidBody::Static);
}
