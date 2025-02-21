use bevy::prelude::*;

use crate::bevy::components::BoardComponent;

pub fn spawn_board(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("textures/board.png"),
            custom_size: Some(Vec2::new(600., 600.)),
            ..default()
        },

        BoardComponent
    ));
}