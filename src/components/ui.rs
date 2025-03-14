use bevy::prelude::*;

#[derive(Component)]
#[require(
    Text(|| "Score: 0"),
    TextFont(|| TextFont {font_size: 60.0, ..default()})
)]
pub struct ScoreText;
