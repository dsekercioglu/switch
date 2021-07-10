use bevy::prelude::*;
use crate::world::{Velocity, MaterialResource, DefaultSize, ObjectMarker, BounceMarker, Location, Force, CharType};
use crate::walls::{WallMarker, WallDeathMarker};
use bevy::sprite::collide_aabb::{collide, Collision};
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use crate::player::PlayerMarker;

const BOUNCING_ENEMY_SIZE: f32 = 20f32;
const BOUNCING_ENEMY_VELOCITY: f32 = 225f32;
const TEAM: u8 = 1;


const HOMING_MINE_SIZE: f32 = 20f32;
const HOMING_MINE_FORCE: f32 = 125f32;
const MINE_HOME_DISTANCE: f32 = 250f32;
const MINE_SPIN_SPEED: f32 = std::f32::consts::PI * 2f32;

#[derive(Bundle)]
pub struct BouncingEnemyBundle {
    marker: BounceMarker,
    object_marker: ObjectMarker,
    type_marker: CharType,
    size: DefaultSize,
    location: Location,
    #[bundle]
    sprite: SpriteBundle,
    velocity: Velocity,
}

pub fn new_bouncing_enemy(material: Handle<ColorMaterial>, location: Vec2) -> BouncingEnemyBundle {
    let sprite_bundle = SpriteBundle {
        sprite: Sprite::new(Vec2::new(BOUNCING_ENEMY_SIZE, BOUNCING_ENEMY_SIZE)),
        material: material.clone(),
        ..Default::default()
    };

    let uniform = Uniform::new(0f32, std::f32::consts::PI * 2f32);
    let angle = thread_rng().sample(uniform);
    let sin_cos = angle.sin_cos();
    let bouncing_enemy_bundle = BouncingEnemyBundle {
        marker: BounceMarker,
        object_marker: ObjectMarker(TEAM),
        type_marker: CharType::Enemy,
        size: DefaultSize {
            width: BOUNCING_ENEMY_SIZE,
            height: BOUNCING_ENEMY_SIZE,
        },
        sprite: sprite_bundle,
        velocity: Velocity(Vec2::new(sin_cos.0 * BOUNCING_ENEMY_VELOCITY,
                                     sin_cos.1 * BOUNCING_ENEMY_VELOCITY)),
        location: Location(location),
    };
    bouncing_enemy_bundle
}

pub fn move_bouncing_enemy(
    time: Res<Time>,
    mut bouncing_enemy: Query<(&BounceMarker, &mut Location, &Velocity)>,
) {
    let delta_seconds = time.delta_seconds();
    for (_, mut location, velocity) in bouncing_enemy.iter_mut() {
        let move_amt = velocity.0 * delta_seconds;
        location.0.x += move_amt.x;
        location.0.y += move_amt.y;
    }
}


pub struct HomingMineMarker;

#[derive(Bundle)]
pub struct HomingMineBundle {
    marker: HomingMineMarker,
    obj_marker: ObjectMarker,
    wall_death_marker: WallDeathMarker,
    type_marker: CharType,
    size: DefaultSize,
    location: Location,
    force: Force,
    velocity: Velocity,
    #[bundle]
    sprite: SpriteBundle,
}

pub fn new_homing_mine(material: Handle<ColorMaterial>, location: Vec2) -> HomingMineBundle {
    let sprite_bundle = SpriteBundle {
        sprite: Sprite::new(Vec2::new(HOMING_MINE_SIZE, HOMING_MINE_SIZE)),
        material: material.clone(),
        ..Default::default()
    };
    let homing_mine_bundle = HomingMineBundle {
        marker: HomingMineMarker,
        obj_marker: ObjectMarker(TEAM),
        wall_death_marker: WallDeathMarker,
        type_marker: CharType::Enemy,
        size: DefaultSize {
            width: HOMING_MINE_SIZE,
            height: HOMING_MINE_SIZE,
        },
        location: Location(location),
        force: Force(Vec2::default()),
        velocity: Velocity(Vec2::default()),
        sprite: sprite_bundle,
    };
    homing_mine_bundle
}

pub fn update_mines(mut homing_mines: Query<(&HomingMineMarker, &Location, &mut Force)>,
                    mut player: Query<(&PlayerMarker, &Location)>) {
    if let Ok((_, player_loc)) = player.single_mut() {
        for (_, mine_loc, mut force) in homing_mines.iter_mut() {
            if mine_loc.0.distance_squared(player_loc.0) < MINE_HOME_DISTANCE * MINE_HOME_DISTANCE {
                force.0 = (player_loc.0 - mine_loc.0).normalize() * HOMING_MINE_FORCE;
            }
        }
    }
}

pub fn homing_mine_spin(time: Res<Time>,
                        mut homing_mines: Query<(&HomingMineMarker, &mut Transform)>) {
    let delta_seconds = time.delta_seconds();
    for (_, mut transform) in homing_mines.iter_mut() {
        transform.rotate(Quat::from_rotation_z(delta_seconds * MINE_SPIN_SPEED));
    }
}