use crate::world::{MaterialResource, ARENA_SIZE};
use crate::GameState;
use bevy::prelude::*;
use bevy::text::Text2dSize;
use bevy::window::WindowMode;

pub fn set_windows(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_title("switch".to_string());
}

pub struct DefaultFontSize(pub f32);

pub struct Fonts {
    font: Handle<Font>,
}

pub fn init_fonts(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("ThaleahFat.ttf");
    commands.insert_resource(Fonts { font })
}

pub struct Chrono(pub f32);

pub struct GameStartTimer {
    pub current_time: Chrono,
}

pub struct TimerUIMarker;

#[derive(Bundle)]
pub struct TimerUI {
    pub marker: TimerUIMarker,
    pub size: DefaultFontSize,
    #[bundle]
    pub text: Text2dBundle,
}

pub fn init_timer(mut commands: Commands, fonts: Res<Fonts>) {
    let text = Text::with_section(
        "".to_string(),
        TextStyle {
            font_size: 0.0,
            color: Color::Rgba {
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 0.2,
            },
            font: fonts.font.clone(),
        },
        TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        },
    );
    let text_bundle = Text2dBundle {
        text,
        ..Default::default()
    };
    commands.spawn_bundle(TimerUI {
        marker: TimerUIMarker,
        size: DefaultFontSize(80.0),
        text: text_bundle,
    });
    commands.insert_resource(GameStartTimer {
        current_time: Chrono(0f32),
    });
}

pub fn remove_timer(mut commands: Commands, timer_ui: Query<(Entity, &TimerUIMarker)>) {
    if let Ok((entity, _)) = timer_ui.single() {
        commands.entity(entity).despawn_recursive();
    }
    commands.remove_resource::<GameStartTimer>();
}

pub fn timer(
    time: Res<Time>,
    mut game_time: ResMut<GameStartTimer>,
    mut timer_ui: Query<(&TimerUIMarker, &mut Text)>,
    mut best_time: ResMut<BestTime>,
) {
    game_time.current_time.0 += time.delta_seconds();
    best_time.0 = game_time.current_time.0.max(best_time.0);
    if let Ok((_, mut text)) = timer_ui.single_mut() {
        text.sections[0].value = format!("{:.1} / {:.1}", game_time.current_time.0, best_time.0);
    }
}

pub struct LeftClickToPlayMarker;

#[derive(Bundle)]
pub struct LeftClickToPlay {
    marker: LeftClickToPlayMarker,
    pub size: DefaultFontSize,
    #[bundle]
    text: Text2dBundle,
}

pub fn init_press_space_to_play(
    mut commands: Commands,
    fonts: Res<Fonts>,
    best_time: Option<ResMut<BestTime>>,
) {
    let mut text = "Click To Play".to_string();
    if let Some(best_time) = best_time {
        text += &format!(" (Best: {:.1})", best_time.0);
    }
    let text = Text::with_section(
        text,
        TextStyle {
            font: fonts.font.clone(),
            font_size: 0.0,
            color: Color::WHITE,
        },
        TextAlignment {
            vertical: VerticalAlign::Center,
            horizontal: HorizontalAlign::Center,
        },
    );
    let text_bundle = Text2dBundle {
        text,
        ..Default::default()
    };
    commands.spawn_bundle(LeftClickToPlay {
        marker: LeftClickToPlayMarker,
        size: DefaultFontSize(40.0),
        text: text_bundle,
    });
}

pub fn update_left_click_to_play(
    mut game_state: ResMut<State<GameState>>,
    mouse: Res<Input<MouseButton>>,
) {
    if let &GameState::Menu = game_state.current() {
        if mouse.just_released(MouseButton::Left) {
            game_state.set(GameState::Game).unwrap();
        }
    }
}

pub fn remove_left_click_to_play(
    mut commands: Commands,
    space_to_play: Query<(Entity, &LeftClickToPlayMarker)>,
) {
    if let Ok((entity, _)) = space_to_play.single() {
        commands.entity(entity).despawn_recursive();
    }
}

pub struct BestTime(pub f32);

pub fn ui_scaling(windows: Res<Windows>, mut q: Query<(&DefaultFontSize, &mut Text)>) {
    let window = windows.get_primary().unwrap();

    let min = window.width().min(window.height());
    for (font_size, mut text) in q.iter_mut() {
        for section in text.sections.iter_mut() {
            section.style.font_size = font_size.0 * min / ARENA_SIZE;
        }
    }
}

pub struct UiBackgroundMarker;

#[derive(Bundle)]
pub struct UiBackground {
    marker: UiBackgroundMarker,
    #[bundle]
    sprite: SpriteBundle,
}

pub fn init_ui_background(mut commands: Commands, material: Res<MaterialResource>) {
    let sprite = SpriteBundle {
        material: material.ui_background_material.clone(),
        transform: Transform::from_xyz(0.0, 0.0, -0.02),
        ..Default::default()
    };
    commands.spawn_bundle(UiBackground {
        marker: UiBackgroundMarker,
        sprite,
    });
}

pub fn ui_background_scaling(
    windows: Res<Windows>,
    mut background: Query<(&UiBackgroundMarker, &mut Sprite)>,
) {
    let window = windows.get_primary().unwrap();

    if let Ok((_, mut sprite)) = background.single_mut() {
        sprite.size = Vec2::new(window.width(), window.height())
    }
}
