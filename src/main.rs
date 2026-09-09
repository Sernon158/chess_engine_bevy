use bevy::ecs::system::NonSendMarker;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy::winit::WINIT_WINDOWS;
use bevy_tweening::TweeningPlugin;
use chess_engine::bevy::scenes::game::{game_scene, idle_animator, piece_move_animator};
use chess_engine::engine::game::StandardGame;
use chess_engine::bevy::constants::*;
use winit::window::Icon;

fn main() {
    let game = StandardGame::default();

    App::new()
        .add_systems(
            Startup, 
            (set_window_icon, game_scene.spawn())
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

pub fn set_window_icon(_marker: NonSendMarker) {

    WINIT_WINDOWS.with_borrow_mut(|winit| {
        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open("./assets/textures/icon_big.png")
                .expect("Failed to open icon path")
                .into_rgba8();

            let (width, height) = image.dimensions();
            let rgba = image.into_raw();

            (rgba, width, height)
        };

        let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

        for window in winit.windows.values() {
            window.set_window_icon(Some(icon.clone()));
        }
    });

}