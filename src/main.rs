use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_tweening::TweeningPlugin;
use chess_engine::constants::STANDARD_GAME_MINUTES;
use chess_engine::game::StandardGame;
use chess_engine::pieces::Color;
use chess_engine::player::Player;
use chess_engine::bevy::{
    constants::{ WINDOW_SIZE, WINDOW_TITLE },
    startup::{ set_window_icon, spawn_board, spawn_camera, spawn_pieces },
    update::{ idle_animator, piece_move_animator }
};

fn main() {
    let game = create_game();

    App::new()
        .add_systems(
            Startup, 
            (set_window_icon, spawn_camera, spawn_board, spawn_pieces)
        )
        .add_systems(
            Update, 
            (idle_animator, piece_move_animator)
        )
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(
                            WINDOW_SIZE.0,
                            WINDOW_SIZE.1
                        ),
                        title: WINDOW_TITLE.to_string(),
                        resizable: false,
                        // TODO: Add window icon
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),

            TweeningPlugin
        ))
        .insert_resource(SpritePickingSettings {
            require_markers: false,
            picking_mode: SpritePickingMode::BoundingBox
        })
        .insert_resource(game)
        .run();
}

fn create_game() -> StandardGame {
    StandardGame::new((
        Player::new(
            None,
            Color::White,
            STANDARD_GAME_MINUTES
        ),
        Player::new(
            None,
            Color::Black,
            STANDARD_GAME_MINUTES
        )
    ))
}