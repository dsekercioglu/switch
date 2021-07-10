use bevy::prelude::*;
use crate::world::{DefaultSize, MaterialResource, Location, CharType};
use bevy::sprite::collide_aabb::{collide, Collision};
use crate::GameState;

pub struct WallMarker;

#[derive(Bundle)]
pub struct WallBundle {
    marker: WallMarker,
    #[bundle]
    sprite: SpriteBundle,
    size: DefaultSize,
    location: Location,
}

pub fn new_wall(material: Handle<ColorMaterial>,
                translation: Vec2,
                size: Vec2) -> WallBundle {
    let size = DefaultSize {
        width: size.x,
        height: size.y,
    };
    let sprite_bundle = SpriteBundle {
        sprite: Sprite::new(Vec2::new(size.width, size.height)),
        transform: Transform::from_translation(Vec3::new(translation.x, translation.y, 0f32)),
        material,
        ..Default::default()
    };
    let wall = WallBundle {
        marker: WallMarker,
        sprite: sprite_bundle,
        location: Location(Vec2::new(translation.x, translation.y)),
        size,
    };
    wall
}

pub struct WallDeathMarker;

pub fn handle_walls(mut commands: Commands,
                    characters: Query<(&WallDeathMarker, Entity, &Location, &DefaultSize, &CharType)>,
                    walls: Query<(&WallMarker, &Location, &DefaultSize)>,
                    mut state: ResMut<State<GameState>>) {
    for (_, wall_location, wall_size) in walls.iter() {
        for (_, entity, location, size, char_type) in characters.iter() {
            if collide(Vec3::new(wall_location.0.x, wall_location.0.y, 0f32),
                       Vec2::new(size.width, size.height),
                       Vec3::new(location.0.x, location.0.y, 0f32),
                       Vec2::new(wall_size.width, wall_size.height)).is_some() {
                if *char_type == CharType::Player {
                    state.set(GameState::Menu).unwrap_or(());
                }
                commands.entity(entity).despawn_recursive();
            }
        }
    }
}