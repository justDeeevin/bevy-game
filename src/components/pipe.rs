use avian2d::prelude::*;
use bevy::prelude::*;
use rand::random_range;

use super::bird::BIRD_HEIGHT;

const PIPE_WIDTH: f32 = 100.0;
const MIN_PIPE_HEIGHT: f32 = 200.0;
const MIN_GAP: f32 = BIRD_HEIGHT * 3.5;
const PIPE_SPEED: f32 = 250.0;

#[derive(Component)]
#[require(
    RigidBody(|| RigidBody::Kinematic),
    LinearVelocity(|| LinearVelocity(Vec2::new(-PIPE_SPEED, 0.0))),
    Sprite,
    Transform,
    Collider,
)]
pub struct Pipe;

#[derive(Bundle)]
struct PipeBundle {
    sprite: Sprite,
    transform: Transform,
    collider: Collider,
    marker: Pipe,
}

#[derive(Component)]
#[require(
    RigidBody(|| RigidBody::Kinematic),
    LinearVelocity(|| LinearVelocity(Vec2::new(-PIPE_SPEED, 0.0))),
    Sensor,
    Collider,
    Transform,
)]
pub struct Scorer;

#[derive(Bundle)]
struct ScorerBundle {
    collider: Collider,
    transform: Transform,
    marker: Scorer,
}

impl ScorerBundle {
    fn new(height: f32, pos: Vec2) -> Self {
        Self {
            marker: Scorer,
            collider: Collider::rectangle(0.0, height),
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
        }
    }
}

enum PipePlacement {
    Top,
    Bottom,
}

impl PipePlacement {
    const fn signum(&self) -> f32 {
        match self {
            PipePlacement::Top => 1.0,
            PipePlacement::Bottom => -1.0,
        }
    }
}

impl PipeBundle {
    fn new(height: f32, placement: PipePlacement, window_size: Vec2) -> Self {
        Self {
            sprite: Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::new(PIPE_WIDTH, height)),
            transform: Transform::from_translation(Vec3::new(
                window_size.x / 2.0 + (PIPE_WIDTH / 2.0),
                placement.signum() * ((window_size.y / 2.0) - height / 2.0),
                0.0,
            )),
            collider: Collider::rectangle(PIPE_WIDTH, height - 20.0),
            marker: Pipe,
        }
    }
}

pub fn spawn_pair(commands: &mut Commands, window: &Window) {
    debug!("Spawning pipe pair");
    let window_size = window.size();
    let bottom_height = random_range(MIN_PIPE_HEIGHT..(window_size.y - MIN_PIPE_HEIGHT - MIN_GAP));
    commands.spawn(PipeBundle::new(
        bottom_height,
        PipePlacement::Bottom,
        window_size,
    ));
    let top_height = random_range(MIN_PIPE_HEIGHT..(window_size.y - bottom_height - MIN_GAP));
    commands.spawn(PipeBundle::new(top_height, PipePlacement::Top, window_size));

    let scorer_height = window_size.y - bottom_height - top_height;
    // in between the two pipes
    let scorer_pos = (bottom_height + (window_size.y - top_height)) / 2.0;
    commands.spawn(ScorerBundle::new(
        scorer_height,
        Vec2::new(
            (window_size.x / 2.0) + (PIPE_WIDTH / 2.0),
            scorer_pos - (window_size.y / 2.0),
        ),
    ));
}
