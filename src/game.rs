use bevy::ecs::system::Resource;

use crate::board::Board;
use crate::logger::Logger;
use crate::pieces::{ Color, Piece };
use crate::player::Player;

#[derive(Clone, Resource)]
pub struct StandardGame {
    /// The game's board.
    pub board: Board,

    /// The game's logger.
    pub logger: Logger,

    /// The current turn.
    pub current_turn: Color,

    /// The players.
    pub players: (Player, Player),

    /// En passant target.
    pub en_passant: Option<(usize, usize)>,

    /// Piece selected
    pub selected_piece: Option<Piece>,

    /// Is a piece animation playing?
    pub animation_playing: bool
}

impl StandardGame {

    pub fn new(players: (Player, Player)) -> Self {
        Self {
            board: Board::new(),
            logger: Logger::new(),
            current_turn: Color::White,
            players,
            en_passant: None,
            selected_piece: None,
            animation_playing: false
        }
    }

    pub fn is_king_checked(&self, color: Color) -> bool {
        self.board.0.iter().flatten().any(|piece| {
            match piece {
                Piece::King(king_color, _) => {
                    if *king_color != color { return false };

                    let king_pos = self.board.get_piece_position(piece);

                    self.board.0.iter().flatten().any(|p| p.can_move(
                        king_pos,
                        self.board.get_piece_position(p),
                        self
                    ))
                },
                _ => false
            }
        })
    }

}