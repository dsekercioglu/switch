use bevy::prelude::*;
use crate::world::{BounceMarker, Velocity, ObjectMarker, DefaultSize, MaterialResource, Location, Counter, CharType};

const SIZE: f32 = 7.5f32;
const BULLET_VELOCITY: f32 = 275f32;
const TIME: f32 = 5f32;

#[derive(Bundle)]
pub struct BulletBundle {
    marker: BounceMarker,
    obj_marker: ObjectMarker,
    type_marker: CharType,
    #[bundle]
    sprite: SpriteBundle,
    size: DefaultSize,
    location: Location,
    velocity: Velocity,
    counter: Counter,
}

pub fn new_bullet(target: Vec2, source: Vec2, team: u8, material: Handle<ColorMaterial>) -> BulletBundle {
    BulletBundle {
        marker: BounceMarker,
        obj_marker: ObjectMarker(team),
        type_marker: CharType::Bullet,
        sprite: SpriteBundle {
            sprite: Sprite::new(Vec2::new(SIZE, SIZE)),
            transform: Transform::from_translation(Vec3::new(source.x, source.y, 0f32)),
            material,
            ..Default::default()
        },
        size: DefaultSize {
            width: SIZE,
            height: SIZE,
        },
        location: Location(Vec2::new(source.x, source.y)),
        counter: Counter(TIME),
        velocity: Velocity((target - source).normalize() * BULLET_VELOCITY),
    }
}