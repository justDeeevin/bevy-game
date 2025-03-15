pub mod event;
pub mod update;

use bevy::prelude::*;

use crate::components::{bird::BirdBundle, ui::ScoreText};

pub fn startup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(BirdBundle::new(asset_server.load("birdx100.png")));

    commands.spawn(ScoreText);
}
