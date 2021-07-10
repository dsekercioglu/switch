use bevy::prelude::*;
use crate::world::{MaterialResource, MouseLoc, Velocity, Target, Force, DefaultSize, ObjectMarker, Location, Counter, CharType};
use crate::walls::WallDeathMarker;
use crate::bullet::new_bullet;
use std::collections::VecDeque;

const POWER: f32 = 175f32;

const PLAYER_SIZE: f32 = 20f32;

const TEAM: u8 = 0;

const COOL_DOWN: f32 = 0.2f32;

pub struct PlayerMarker;

pub struct Switch(bool);

pub struct CoolDown(f32);

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
    cool_down: CoolDown,
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
        cool_down: CoolDown(0f32),
    };
    commands.spawn_bundle(player_bundle);
}

pub fn mouse_click(mut commands: Commands,
                   mouse_input: Res<Input<MouseButton>>,
                   mouse_loc: Res<MouseLoc>,
                   materials: Res<MaterialResource>,
                   mut query: Query<(&mut PlayerMarker, &ObjectMarker, &mut Location, &mut Force, &mut Target, &mut Switch, &mut CoolDown)>) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if let Ok((_, marker, location, mut force, mut target, mut switch, mut cool_down)) = query.single_mut() {
            if cool_down.0 <= 0.0 {
                cool_down.0 = COOL_DOWN;
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
}

pub fn update_cool_down(
    time: Res<Time>,
    mut cool_down: Query<&mut CoolDown>) {
    for mut cool_down in cool_down.iter_mut() {
        cool_down.0 -= time.delta_seconds();
    }
}