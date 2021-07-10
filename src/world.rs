use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use crate::{walls, GameState};
use bevy::sprite::collide_aabb::{collide, Collision};
use crate::walls::WallMarker;
use crate::player::PlayerMarker;
use rand::{thread_rng, Rng};
use rand::distributions::Uniform;
use crate::enemies::{BouncingEnemyBundle, new_bouncing_enemy, new_homing_mine};

pub const ARENA_SIZE: f32 = 600f32;
const ARENA_MARGIN: f32 = 20f32;

const DECAY: f32 = 0.01;

pub struct MainCamera;

pub fn setup(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);
}

pub struct MouseLoc {
    pub location: Vec2,
}

pub fn setup_mouse(mut commands: Commands) {
    commands.insert_resource(MouseLoc {
        location: Vec2::default()
    })
}

pub fn cursor_system(
    windows: Res<Windows>,
    cam: Query<&Transform, With<MainCamera>>,
    mut mouse_resource: ResMut<MouseLoc>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(pos) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let p = pos - size / 2.0;
        let camera_transform = cam.single().unwrap();
        let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
        mouse_resource.location = Vec2::new(pos_wld.x, pos_wld.y);
    }
}

pub struct MaterialResource {
    pub ui_background_material: Handle<ColorMaterial>,
    pub background_material: Handle<ColorMaterial>,
    pub player_material: Handle<ColorMaterial>,
    pub bouncing_enemy_material: Handle<ColorMaterial>,
    pub wall_material: Handle<ColorMaterial>,
    pub bullet_material: Handle<ColorMaterial>,
}

pub fn init_material(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let ui_background_material = materials.add(
        ColorMaterial::color(Color::rgb(0.3, 0.3, 0.3))
    );
    let background_material = materials.add(
        ColorMaterial::color(Color::rgb(0.5, 0.5, 0.5))
    );
    let player_material = materials.add(
        ColorMaterial::color(Color::rgb(0.7, 0.7, 0.7))
    );
    let bouncing_enemy_material = materials.add(
        ColorMaterial::color(Color::rgb(0.7, 0.0, 0.0))
    );
    let wall_material = materials.add(
        ColorMaterial::color(Color::rgb(0.2, 0.2, 0.2))
    );
    let bullet_material = materials.add(
        ColorMaterial::color(Color::rgb(0.0, 0.7, 0.7))
    );
    commands.insert_resource(
        MaterialResource {
            ui_background_material,
            background_material,
            player_material,
            bouncing_enemy_material,
            wall_material,
            bullet_material,
        }
    )
}

pub struct Force(pub Vec2);

pub struct Target(pub Vec2);

pub struct Velocity(pub Vec2);

pub struct DefaultSize {
    pub width: f32,
    pub height: f32,
}

pub struct Location(pub Vec2);

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CharType {
    Player,
    Bullet,
    Enemy,
}

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&DefaultSize, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();

    let min = window.width().min(window.height());
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size.x = sprite_size.width / ARENA_SIZE as f32 * min;
        sprite.size.y = sprite_size.height / ARENA_SIZE as f32 * min;
    }
}

pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Location, &mut Transform)>) {
    let window = windows.get_primary().unwrap();

    let ratio = window.width().min(window.height()) / ARENA_SIZE;
    for (loc, mut transform) in q.iter_mut() {
        transform.translation.x = loc.0.x * ratio;
        transform.translation.y = loc.0.y * ratio;
    }
}


pub fn setup_walls(mut commands: Commands,
                   materials: Res<MaterialResource>) {
    let left_wall = walls::new_wall(materials.wall_material.clone(),
                                    Vec2::new(-300f32, 0f32),
                                    Vec2::new(40f32, 600f32));

    let right_wall = walls::new_wall(materials.wall_material.clone(),
                                     Vec2::new(300f32, 0f32),
                                     Vec2::new(40f32, 600f32));

    let down_wall = walls::new_wall(materials.wall_material.clone(),
                                    Vec2::new(0f32, -300f32),
                                    Vec2::new(600f32, 40f32));

    let up_wall = walls::new_wall(materials.wall_material.clone(),
                                  Vec2::new(-0f32, 300f32),
                                  Vec2::new(600f32, 40f32));

    commands.spawn_bundle(left_wall);
    commands.spawn_bundle(right_wall);
    commands.spawn_bundle(down_wall);
    commands.spawn_bundle(up_wall);
}

pub struct ObjectMarker(pub u8);

pub fn handle_object_collision(mut commands: Commands,
                               characters: Query<(&ObjectMarker, Entity, &Location, &DefaultSize, &CharType)>,
                               mut state: ResMut<State<GameState>>) {
    for (marker_0, entity_0, location_0, size_0, type_0) in characters.iter() {
        for (marker_1, entity_1, location_1, size_1, type_1) in characters.iter() {
            if entity_0 != entity_1 {
                if marker_0.0 != marker_1.0 {
                    if collide(Vec3::new(location_0.0.x, location_0.0.y, 0f32),
                               Vec2::new(size_0.width, size_0.height),
                               Vec3::new(location_1.0.x, location_1.0.y, 0f32),
                               Vec2::new(size_1.width, size_1.height)).is_some() {
                        commands.entity(entity_0).despawn_recursive();
                        commands.entity(entity_1).despawn_recursive();
                        if *type_0 == CharType::Player || *type_1 == CharType::Player {
                            state.set(GameState::Menu).unwrap_or(());
                            return;
                        }
                    }
                }
            }
        }
    }
}

pub struct BounceMarker;

pub fn handle_bounce(mut bouncing_enemy: Query<(&BounceMarker, &DefaultSize, &Location, &mut Velocity)>,
                     walls: Query<(&WallMarker, &Location, &DefaultSize)>) {
    for (_, size, location, mut velocity) in bouncing_enemy.iter_mut() {
        for (_, wall_location, wall_size) in walls.iter() {
            if let Some(collision) = collide(Vec3::new(location.0.x, location.0.y, 0f32),
                                             Vec2::new(size.width * 1.1, size.height * 1.1),
                                             Vec3::new(wall_location.0.x, wall_location.0.y, 0f32),
                                             Vec2::new(wall_size.width, wall_size.height)) {
                match collision {
                    Collision::Left => {
                        if velocity.0.x > 0f32 {
                            velocity.0.x *= -1f32;
                        }
                    }
                    Collision::Right => {
                        if velocity.0.x < 0f32 {
                            velocity.0.x *= -1f32;
                        }
                    }
                    Collision::Top => {
                        if velocity.0.y < 0f32 {
                            velocity.0.y *= -1f32;
                        }
                    }
                    Collision::Bottom => {
                        if velocity.0.y > 0f32 {
                            velocity.0.y *= -1f32;
                        }
                    }
                }
            }
        }
    }
}

pub struct Counter(pub f32);

pub fn update_counters(mut commands: Commands,
                       time: Res<Time>,
                       mut query: Query<(Entity, &mut Counter)>) {
    let delta_seconds = time.delta_seconds();
    for (entity, mut counter) in query.iter_mut() {
        counter.0 -= delta_seconds;
        if counter.0 < 0f32 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

const MAX_DIFFICULTY: f32 = 30f32;
const DIFFICULTY_BASE: f32 = 0.5f32;
const DIFFICULTY_MULTIPLIER: f32 = 0.1f32;

const PLAYER_DISTANCE: f32 = 150f32;

pub struct SpawnSystem {
    time_since_start: f32,
    time_since_last_spawn: f32,
}

pub fn init_spawn(mut commands: Commands) {
    commands.insert_resource(SpawnSystem {
        time_since_start: 0f32,
        time_since_last_spawn: 0f32,
    })
}

pub fn spawn_system(mut commands: Commands,
                    time: Res<Time>,
                    mut spawn_system: ResMut<SpawnSystem>,
                    player: Query<(&PlayerMarker, &Location)>,
                    material: Res<MaterialResource>) {
    let delta_seconds = time.delta_seconds();
    spawn_system.time_since_start += delta_seconds;
    spawn_system.time_since_last_spawn += delta_seconds;
    let difficulty = MAX_DIFFICULTY - spawn_system.time_since_start.sqrt().min(MAX_DIFFICULTY);
    if let Ok((_, location)) = player.single() {
        if spawn_system.time_since_last_spawn > DIFFICULTY_BASE + difficulty * DIFFICULTY_MULTIPLIER {
            spawn_system.time_since_last_spawn = 0f32;
            let distribution = Uniform::new(-ARENA_SIZE * 0.5f32 + ARENA_MARGIN * 2f32, ARENA_SIZE * 0.5f32 - ARENA_MARGIN * 2f32);
            let mut position;
            loop {
                let x = rand::thread_rng().sample(distribution);
                let y = rand::thread_rng().sample(distribution);
                position = Vec2::new(x, y);
                if position.distance_squared(location.0) > PLAYER_DISTANCE * PLAYER_DISTANCE {
                    break;
                }
            }
            match rand::thread_rng().gen_range(0..2) {
                0 => {
                    commands.spawn_bundle(new_bouncing_enemy(material.bouncing_enemy_material.clone(), position));
                }
                1 => {
                    commands.spawn_bundle(new_homing_mine(material.bouncing_enemy_material.clone(), position));
                }
                _ => {}
            }
        }
    }
}

pub fn slide_move(time: Res<Time>,
                  mut query: Query<(&mut Location, &Force, &mut Velocity)>) {
    let delta_seconds = time.delta_seconds();
    for (mut location, force, mut velocity) in query.iter_mut() {
        velocity.0 += force.0 * delta_seconds;
        velocity.0 *= 1f32 - DECAY;
        let move_amt = velocity.0 * delta_seconds;
        location.0.x += move_amt.x;
        location.0.y += move_amt.y;
    }
}

pub fn clear_world(mut commands: Commands,
                   objects: Query<(Entity, &ObjectMarker)>,
                   walls: Query<(Entity, &WallMarker)>) {
    for (entity, _) in objects.iter() {
        commands.entity(entity).despawn_recursive();
    }
    for (entity, _) in walls.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct BackgroundMarker;

#[derive(Bundle)]
pub struct Background {
    marker: BackgroundMarker,
    size: DefaultSize,
    location: Location,
    #[bundle]
    sprite: SpriteBundle,
}

pub fn init_background(mut commands: Commands, material: Res<MaterialResource>) {
    let sprite = SpriteBundle {
        sprite: Default::default(),
        material: material.background_material.clone(),
        transform: Transform::from_xyz(0.0, 0.0, -0.01),
        ..Default::default()
    };
    commands.spawn_bundle(Background {
        marker: BackgroundMarker,
        size: DefaultSize { width: ARENA_SIZE, height: ARENA_SIZE },
        location: Location(Vec2::new(0.0, 0.0)),
        sprite,
    });
}

pub fn remove_background(mut commands: Commands,
                         background: Query<(Entity, &BackgroundMarker)>) {
    if let Ok((entity, _)) = background.single() {
        commands.entity(entity).despawn_recursive();
    }
}