use avian2d::prelude::*;
use bevy::prelude::*;

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
            sprite: Sprite::from_color(Color::srgb(1.0, 1.0, 1.0), Vec2::new(100.0, 100.0)),
            body: RigidBody::Static,
            collider: Collider::rectangle(100.0, 100.0),
            marker: Bird,
        }
    }
}

pub fn jump(v: &mut LinearVelocity) {
    debug!("Jumping");
    *v = LinearVelocity(Vec2::new(0.0, 400.0));
}
