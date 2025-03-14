use avian2d::prelude::*;
use bevy::prelude::*;

const JUMP_SPEED: f32 = 550.0;
pub const BIRD_SIZE: f32 = 100.0;

#[derive(Component)]
pub struct Bird;

#[derive(Bundle)]
pub struct BirdBundle {
    sprite: Sprite,
    body: RigidBody,
    collider: Collider,
    marker: Bird,
}

impl BirdBundle {
    pub fn new() -> Self {
        Self {
            sprite: Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(BIRD_SIZE, BIRD_SIZE)),
            body: RigidBody::Static,
            collider: Collider::rectangle(100.0, 100.0),
            marker: Bird,
        }
    }
}

pub fn jump(v: &mut LinearVelocity) {
    debug!("Jumping");
    *v = LinearVelocity(Vec2::new(0.0, JUMP_SPEED));
}
