pub mod game;


/*-------------------- STATES --------------------*/
use bevy::prelude::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameCurrentScene {
    StartingMenu,
    Game
}