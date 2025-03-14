use avian2d::prelude::*;
use bevy::prelude::*;

const JUMP_SPEED: f32 = 550.0;
pub const BIRD_SIZE: f32 = 100.0;

#[derive(Component)]
#[require(
    RigidBody(|| RigidBody::Static),
    Collider(|| Collider::rectangle(BIRD_SIZE, BIRD_SIZE)),
    Sprite(|| Sprite::from_color(Color::srgb(1.0, 1.0, 1.0),
    Vec2::new(BIRD_SIZE, BIRD_SIZE)))
)]
pub struct Bird;

pub fn jump(v: &mut LinearVelocity) {
    debug!("Jumping");
    *v = LinearVelocity(Vec2::new(0.0, JUMP_SPEED));
}
