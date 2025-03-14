use avian2d::prelude::*;
use bevy::prelude::*;
use rand::random_range;

use super::bird::BIRD_SIZE;

const MIN_PIPE_HEIGHT: f32 = 200.0;
const MIN_GAP: f32 = BIRD_SIZE * 2.5;
const PIPE_SPEED: f32 = 250.0;

#[derive(Component)]
pub struct Pipe;

#[derive(Bundle)]
pub struct PipeBundle {
    sprite: Sprite,
    transform: Transform,
    body: RigidBody,
    collider: Collider,
    sensor: Sensor,
    velocity: LinearVelocity,
    marker: Pipe,
}

enum PipePlacement {
    Top,
    Bottom,
}

impl PipePlacement {
    pub const fn signum(&self) -> f32 {
        match self {
            PipePlacement::Top => 1.0,
            PipePlacement::Bottom => -1.0,
        }
    }
}

impl PipeBundle {
    fn new(height: f32, placement: PipePlacement, window_size: Vec2) -> Self {
        Self {
            sprite: Sprite::from_color(Color::srgb(1.0, 0.0, 0.0), Vec2::new(50.0, height)),
            transform: Transform::from_translation(Vec3::new(
                window_size.x / 2.0 + 25.0,
                placement.signum() * ((window_size.y / 2.0) - height / 2.0),
                0.0,
            )),
            body: RigidBody::Kinematic,
            collider: Collider::rectangle(50.0, height),
            sensor: Sensor,
            velocity: LinearVelocity(Vec2::new(-PIPE_SPEED, 0.0)),
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
}
