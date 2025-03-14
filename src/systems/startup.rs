use bevy::prelude::*;

use crate::components::bird::BirdBundle;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn spawn_bird(mut commands: Commands) {
    commands.spawn(BirdBundle::new());
}
