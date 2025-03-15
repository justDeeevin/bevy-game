use avian2d::prelude::*;
use bevy::prelude::*;

const JUMP_SPEED: f32 = 550.0;
pub const BIRD_HEIGHT: f32 = 100.0;
pub const BIRD_WIDTH: f32 = BIRD_HEIGHT * 1.2;

#[derive(Component)]
#[require(
    RigidBody(|| RigidBody::Static),
    Collider(|| Collider::rectangle(BIRD_WIDTH, BIRD_HEIGHT)),
    Sprite,
)]
pub struct Bird;

pub fn jump(v: &mut LinearVelocity) {
    debug!("Jumping");
    *v = LinearVelocity(Vec2::new(0.0, JUMP_SPEED));
}

#[derive(Bundle)]
pub struct BirdBundle {
    sprite: Sprite,
    marker: Bird,
}

impl BirdBundle {
    pub fn new(image: Handle<Image>) -> Self {
        let mut sprite = Sprite::from_image(image);
        sprite.custom_size = Some(Vec2::new(BIRD_WIDTH, BIRD_HEIGHT));
        Self {
            sprite,
            marker: Bird,
        }
    }
}
