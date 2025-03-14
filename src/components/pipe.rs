use avian2d::prelude::*;
use bevy::prelude::*;

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
            velocity: LinearVelocity(Vec2::new(-100.0, 0.0)),
            marker: Pipe,
        }
    }
}

pub fn spawn_pair(commands: &mut Commands, window: &Window) {
    let window_size = window.size();
    commands.spawn(PipeBundle::new(500.0, PipePlacement::Top, window_size));
}
