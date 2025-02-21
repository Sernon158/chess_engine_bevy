use bevy::prelude::*;

use crate::pieces::Piece;

#[derive(Component, Clone)]
pub struct PieceComponent(pub Piece);