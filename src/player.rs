use bevy::prelude::*;
use crate::world::{MaterialResource, MouseLoc, Velocity, Target, Force, DefaultSize, ObjectMarker, Location, Counter, CharType};
use crate::walls::WallDeathMarker;
use crate::bullet::new_bullet;
use std::collections::VecDeque;

const POWER: f32 = 175f32;

const PLAYER_SIZE: f32 = 20f32;

const TEAM: u8 = 0;

pub struct PlayerMarker;

pub struct Switch(bool);

#[derive(Bundle)]
pub struct PlayerBundle {
    marker: PlayerMarker,
    wall_marker: WallDeathMarker,
    object_marker: ObjectMarker,
    type_marker: CharType,
    #[bundle]
    sprite: SpriteBundle,
    size: DefaultSize,
    location: Location,

    force: Force,
    target: Target,
    velocity: Velocity,
    switch: Switch,
}

pub fn new_player(mut commands: Commands, resource: Res<MaterialResource>) {
    let sprite_bundle = SpriteBundle {
        sprite: Sprite::new(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
        material: resource.player_material.clone(),
        ..Default::default()
    };
    let player_bundle = PlayerBundle {
        marker: PlayerMarker,
        wall_marker: WallDeathMarker,
        object_marker: ObjectMarker(TEAM),
        type_marker: CharType::Player,
        sprite: sprite_bundle,
        size: DefaultSize {
            width: PLAYER_SIZE,
            height: PLAYER_SIZE,
        },
        location: Location(Vec2::default()),
        target: Target(Vec2::default()),
        force: Force(Vec2::default()),
        velocity: Velocity(Vec2::default()),
        switch: Switch(true),
    };
    commands.spawn_bundle(player_bundle);
}

pub fn mouse_click(mut commands: Commands,
                   mouse_input: Res<Input<MouseButton>>,
                   mouse_loc: Res<MouseLoc>,
                   materials: Res<MaterialResource>,
                   mut query: Query<(&mut PlayerMarker, &ObjectMarker, &mut Location, &mut Force, &mut Target, &mut Switch)>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok((_, marker, location, mut force, mut target, mut switch)) = query.single_mut() {
            if switch.0 {
                target.0 = mouse_loc.location;
                force.0 = Vec2::new(
                    mouse_loc.location.x - location.0.x,
                    mouse_loc.location.y - location.0.y,
                ).normalize() * POWER;
            } else {
                let source = Vec2::new(
                    location.0.x,
                    location.0.y,
                );
                commands.spawn_bundle(new_bullet(mouse_loc.location, source, marker.0, materials.bullet_material.clone()));
            }
            switch.0 = !switch.0;
        }
    }
}