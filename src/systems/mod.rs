pub mod event;
pub mod update;

use bevy::prelude::*;

use crate::components::{bird::Bird, ui::ScoreText};

pub fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
    commands.spawn(Bird);
    commands.spawn(ScoreText);
}
