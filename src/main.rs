mod audio;
mod bullet;
mod enemies;
mod player;
mod ui;
mod walls;
mod world;


use crate::enemies::{homing_mine_spin, move_bouncing_enemy, update_mines};
use crate::player::{mouse_click, new_player, update_cool_down};
use crate::ui::{init_fonts, init_press_space_to_play, init_timer, init_ui_background, remove_left_click_to_play, remove_timer, timer, ui_background_scaling, ui_scaling, update_left_click_to_play, BestTime, set_windows};
use crate::walls::handle_walls;
use crate::world::{
    clear_world, cursor_system, handle_bounce, handle_object_collision, init_background,
    init_material, init_spawn, position_translation, remove_background, setup, setup_mouse,
    setup_walls, size_scaling, slide_move, spawn_system, update_counters,
};
use bevy::input::system::exit_on_esc_system;
use bevy::prelude::*;
use bevy::core::{CorePlugin, FixedTimestep};
use bevy::app::Events;
use bevy::window::WindowResized;
use bevy::ecs::schedule::ShouldRun;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Init,
    Menu,
    Game,
}

fn main() {
    #[cfg(target_arch = "wasm32")]
        console_error_panic_hook::set_once();

    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    // when building for Web, use WebGL2 rendering
    #[cfg(target_arch = "wasm32")]
        app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.insert_resource(BestTime(0.0))
        .add_state(GameState::Init)
        .add_startup_stage(
            "init",
            SystemStage::parallel()
                .with_system(setup.system())
                .with_system(init_material.system())
                .with_system(setup_mouse.system())
                .with_system(init_fonts.system())
                .with_system(set_windows.system()),
        )
        .add_startup_stage(
            "app_start",
            SystemStage::parallel()
                .with_system(enter_menu.system())
                .with_system(init_ui_background.system()),
        )
        .add_system(exit_on_esc_system.system())
        .add_system(mouse_click.system())
        .add_system(cursor_system.system())
        .add_system(ui_scaling.system())
        .add_system(ui_background_scaling.system())
        .add_system_set(
            SystemSet::on_enter(GameState::Menu).with_system(init_press_space_to_play.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Menu).with_system(update_left_click_to_play.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Menu).with_system(remove_left_click_to_play.system()),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::Game)
                .with_system(new_player.system())
                .with_system(setup_walls.system())
                .with_system(init_spawn.system())
                .with_system(init_timer.system())
                .with_system(init_background.system())
        )
        .add_system_set(
            SystemSet::on_update(GameState::Game)
                .with_system(slide_move.system())
                .with_system(handle_walls.system())
                .with_system(move_bouncing_enemy.system())
                .with_system(handle_bounce.system())
                .with_system(handle_object_collision.system())
                .with_system(update_counters.system())
                .with_system(spawn_system.system())
                .with_system(homing_mine_spin.system())
                .with_system(update_mines.system())
                .with_system(timer.system())
                .with_system(update_cool_down.system())
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Game)
                .with_system(remove_timer.system())
                .with_system(clear_world.system())
                .with_system(remove_background.system()),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(size_scaling.system())
                .with_system(position_translation.system()),
        ).run();
}

fn enter_menu(mut game_state: ResMut<State<GameState>>) {
    game_state.set(GameState::Menu).unwrap();
}
