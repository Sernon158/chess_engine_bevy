use bevy::ecs::system::Resource;

use crate::board::Board;
use crate::logger::Logger;
use crate::pieces::{ Color, Piece, PieceKind };
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

    /// Checks if king is checked after a piece moves (not necesarely the king).
    pub fn is_king_checked_after_move(
        &self,
        king_color: Color,
        (curr_x, curr_y): (usize, usize),
        (x, y): (usize, usize)
    ) -> bool {
        let mut cloned_game = self.clone();
        let piece_to_move = *cloned_game.board.get(curr_x, curr_y);

        cloned_game.board.set(curr_x, curr_y, Piece::none(u8::MAX));
        cloned_game.board.set(x, y, piece_to_move);

        // Go through all pieces in the board and find the king
        // of the color specified.
        cloned_game.board.0.iter().flatten().any(|piece| {
            if piece.kind != PieceKind::King
            || king_color != piece.color
            { return false };

            let king_pos = cloned_game.board.get_piece_position(piece);

            // Once we find the king, go through the board again and check every piece
            // to see if it can go to the slot the king is in. If any
            // (that isn't of the same color as the king) can, return false.
            cloned_game.board.0.iter().flatten().any(|p| {
                if p.color == king_color { return false };
                
                p.can_move(
                    king_pos,
                    cloned_game.board.get_piece_position(p),
                    &cloned_game,
                    false
                )
            })
        })
    }

    pub fn is_king_checked_at(&self, color: Color, king_pos: (usize, usize)) -> bool {
        self.board.0.iter().flatten().any(|p| {
            if p.color == color { return false };
            
            p.can_move(
                king_pos,
                self.board.get_piece_position(p),
                self,
                false
            )
        })
    }

}