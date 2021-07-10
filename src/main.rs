mod world;
mod player;
mod enemies;
mod walls;
mod bullet;
mod ui;
mod audio;

use bevy::prelude::*;
use crate::world::{init_material, setup, setup_mouse, cursor_system, size_scaling, setup_walls, handle_object_collision, handle_bounce, position_translation, update_counters, spawn_system, init_spawn, slide_move, clear_world, init_background, remove_background};
use crate::player::{mouse_click, new_player, update_cool_down};
use crate::walls::handle_walls;
use crate::enemies::{move_bouncing_enemy, homing_mine_spin, update_mines};
use bevy::input::system::exit_on_esc_system;
use crate::ui::{init_timer, init_press_space_to_play, init_fonts, remove_timer, update_press_space_to_play, remove_press_space_to_play, timer, ui_scaling, BestTime, ui_background_scaling, init_ui_background};
use bevy::audio::AudioPlugin;
use crate::audio::{load_audio, play_audio};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Init,
    Menu,
    Game,
}


fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(BestTime(0.0))
        .add_state(GameState::Init)
        .add_startup_stage("init", SystemStage::parallel()
            .with_system(setup.system())
            .with_system(init_material.system())
            .with_system(setup_mouse.system())
            .with_system(init_fonts.system()),
                           //.with_system(load_audio.system())
        )
        .add_startup_stage("app_start",
                           SystemStage::parallel()
                               .with_system(enter_menu.system())
                               .with_system(init_ui_background.system()),
                           //.with_system(play_audio.system())
        )
        .add_system(exit_on_esc_system.system())
        .add_system(mouse_click.system())
        .add_system(cursor_system.system())
        .add_system(ui_scaling.system())
        .add_system(ui_background_scaling.system())
        .add_system_set(
            SystemSet::on_enter(GameState::Menu).
                with_system(init_press_space_to_play.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Menu).
                with_system(update_press_space_to_play.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Menu)
                .with_system(remove_press_space_to_play.system()),
        )
        .add_system_set(SystemSet::on_enter(GameState::Game)
            .with_system(new_player.system())
            .with_system(setup_walls.system())
            .with_system(init_spawn.system())
            .with_system(init_timer.system())
            .with_system(init_background.system()))
        .add_system_set(SystemSet::on_update(GameState::Game)
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
            .with_system(update_cool_down.system()))
        .add_system_set(SystemSet::on_exit(GameState::Game)
            .with_system(remove_timer.system())
            .with_system(clear_world.system())
            .with_system(remove_background.system()))
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(size_scaling.system())
                .with_system(position_translation.system()))
        .run();
}

fn enter_menu(mut game_state: ResMut<State<GameState>>) {
    game_state.set(GameState::Menu).unwrap();
}