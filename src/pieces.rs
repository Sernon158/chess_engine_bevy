use crate::{board::Board, game::StandardGame};

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Piece {
    Pawn(Color, u8),
    Rook(Color, u8),
    Knight(Color, u8),
    Bishop(Color, u8),
    Queen(Color, u8),
    King(Color, u8),
    None(u8)
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Color {
    White,
    Black,
    Empty
}

impl Color {
    pub fn get_opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
            Color::Empty => Color::Empty
        }
    }

    #[inline(always)]
    pub fn is_white(&self) -> bool { matches!(self, Color::White) }
    #[inline(always)]
    pub fn is_black(&self) -> bool { matches!(self, Color::Black) }
    #[inline(always)]
    pub fn is_empty(&self) -> bool { matches!(self, Color::Empty) }
}

impl Piece {


    pub fn can_move(
        &self,
        target_pos: (usize, usize),
        current_pos: (usize, usize),
        game: &StandardGame,
        look_for_checks: bool
    ) -> bool {
        let (x, y) = target_pos;
        let (curr_x, curr_y) = current_pos;

        if x > 7 || y > 7 { return false };
        if target_pos == current_pos { return false };

        let (&color, _) = self.get_data();

        // Closure to sum `plus` to `slot`. If it's a black piece,
        // substract the value from it instead to invert it.
        let slotsum = |slot: usize, plus: i8| match color {
            Color::White => (slot as i8 + plus) as usize,
            Color::Black => (slot as i8 - plus) as usize,
            _ => 0
        };
        
        let piece_can_move = || match self {
            Piece::Pawn(..) => {
                let if_color = |white: usize, black: usize| match color {
                    Color::White => white,
                    Color::Black => black,
                    _ => 0
                };

                //// En passant
                //if let Some((x, y)) = game.en_passant {
                //    return (x == curr_x - 1 || x == curr_x - 1) && (y == curr_y + 1 || y == curr_y - 1)
                //}

                // Capture
                if (x == curr_x.wrapping_sub(1) || x == curr_x + 1)
                && y == slotsum(curr_y, 1)
                {
                    let target = game.board.get(x, y);
                    if target.is_none() { return false };

                    let (&target_color, _) = target.get_data();

                    return target_color != color
                }

                // Normal Pawn Movement
                if curr_y == if_color(1, 6) && y == if_color(3, 4) {
                    let prev_target = self
                        .is_passable(&game.board, (x, slotsum(y, -1)));

                    let target = self.is_passable(&game.board, (x, y));
                
                    return x == curr_x
                        && prev_target.passable()
                        && target.passable()
                } else {
                    let one_step = self.is_passable(&game.board, target_pos);

                    return x == curr_x
                        && y == slotsum(curr_y, 1)
                        && one_step.passable()
                }
            },

            Piece::Rook(..) => {
                let x_diff = x as i8 - curr_x as i8;
                let y_diff = y as i8 - curr_y as i8;

                if y == curr_y && x != curr_x {
                    let range = 1..x_diff.abs();

                    for coord in range {
                        let new_x = match x_diff >= 0 {
                            true => curr_x + coord as usize,
                            false => curr_x - coord as usize
                        };

                        let target = self.is_passable(&game.board, (new_x, y));
                        if !target.passable() { return false };
                    }
                } else if x == curr_x && y != curr_y {
                    let range = 1..y_diff.abs();

                    for coord in range {
                        let new_y = match y_diff >= 0 {
                            true => curr_y + coord as usize,
                            false => curr_y - coord as usize
                        };

                        let target = self.is_passable(&game.board, (x, new_y));
                        if !target.passable() { return false };
                    }
                } else {
                    return false
                };

                let target = self.is_passable(&game.board, (x, y));
                if !target.to_bool() { return false };
                
                return true
            },

            Piece::Knight(..) => {
                let target = self.is_passable(&game.board, target_pos);
                let x_diff = (x as i8 - curr_x as i8).abs();
                let y_diff = (y as i8 - curr_y as i8).abs();

                return target.to_bool()
                    && ((x_diff == 2 && y_diff == 1) || (x_diff == 1 && y_diff == 2))
            },

            Piece::Bishop(..) => {
                let x_diff = (x as i8 - curr_x as i8).abs();
                let y_diff = (y as i8 - curr_y as i8).abs();

                if x_diff != y_diff { return false };
            
                let dx = if x > curr_x { 1 } else { -1 };
                let dy = if y > curr_y { 1 } else { -1 };
            
                for coord in 1..x_diff {
                    let new_x = (curr_x as i8 + coord * dx) as usize;
                    let new_y = (curr_y as i8 + coord * dy) as usize;
                    
                    let target = self.is_passable(&game.board, (new_x, new_y));
                    if !target.passable() { return false };
                }

                let dest = self.is_passable(&game.board, (x, y));
                if !dest.to_bool() { return false };
            
                return true
            },

            Piece::Queen(..) => {
                let x_diff = (x as i8 - curr_x as i8).abs();
                let y_diff = (y as i8 - curr_y as i8).abs();

                // Horizontal / Vertical
                if (x == curr_x && y != curr_y) || (x != curr_x && y == curr_y) {
                    // Delegate the check to the Rook piece
                    return Piece::Rook(color, 255)
                        .can_move(target_pos, current_pos, game, false)
                }
                // Diagonal
                else if x_diff == y_diff {
                    // Delegate the check to the Bishop piece
                    return Piece::Bishop(color, 255)
                        .can_move(target_pos, current_pos, game, false)
                }

                return false
            },

            Piece::King(..) => {
                let x_diff = (x as i8 - curr_x as i8).abs();
                let y_diff = (y as i8 - curr_y as i8).abs();
                let dest = self.is_passable(&game.board, (x, y));

                return x_diff <= 1 && y_diff <= 1 && dest.to_bool()
            },

            Piece::None(_) => return false
        };

        piece_can_move() && (
               !look_for_checks
            || (look_for_checks && !game.is_king_checked_after_move(color, (curr_x, curr_y), (x, y)))
        )

        // TODO: Add a check for the king checked. If the king (of same
        // color) is still checked after this `match`, it returns false.
    }


    pub fn get_display(&self) -> Option<&str> {
        match self {
            Piece::Pawn(color, _) => {
                if let Color::White = color { Some("pawn") }
                else { Some("pawn_black") }
            },

            Piece::Rook(color, _) => {
                if let Color::White = color { Some("rook") }
                else { Some("rook_black") }
            },

            Piece::Knight(color, _) => {
                if let Color::White = color { Some("knight") }
                else { Some("knight_black") }
            },

            Piece::Bishop(color, _) => {
                if let Color::White = color { Some("bishop") }
                else { Some("bishop_black") }
            },

            Piece::Queen(color, _) => {
                if let Color::White = color { Some("queen") }
                else { Some("queen_black") }
            },

            Piece::King(color, _) => {
                if let Color::White = color { Some("king") }
                else { Some("king_black") }
            },

            Piece::None(_) => None
        }
    }


    pub fn get_data(&self) -> (&Color, &u8) {
        match self {
            Piece::Pawn(color, id) => (color, id),
            Piece::Rook(color, id) => (color, id),
            Piece::Knight(color, id) => (color, id),
            Piece::Bishop(color, id) => (color, id),
            Piece::Queen(color, id) => (color, id),
            Piece::King(color, id) => (color, id),
            Piece::None(id) => (&Color::Empty, id)
        }
    }

    pub fn set_id(&mut self, new_id: u8) {
        match self {
            Piece::Pawn(_, ref mut id) => *id = new_id,
            Piece::Rook(_, ref mut id) => *id = new_id,
            Piece::Knight(_, ref mut id) => *id = new_id,
            Piece::Bishop(_, ref mut id) => *id = new_id,
            Piece::Queen(_, ref mut id) => *id = new_id,
            Piece::King(_, ref mut id) => *id = new_id,
            Piece::None(ref mut id) => *id = new_id,
        }
    }

    /// Check if the board slot `(x, y)` is passable by this piece.
    pub fn is_passable(&self, board: &Board, (x, y): (usize, usize)) -> BoardSlotType {
        let board_piece = board.get(x, y);
        if board_piece.is_none() { return BoardSlotType::Passable };

        let (self_color, _) = self.get_data();
        let (piece_color, _) = board_piece.get_data();

        if self_color != piece_color {
            return BoardSlotType::Capturable
        }

        BoardSlotType::Unpassable
    }

    pub fn is_none(&self) -> bool {
        match *self {
            Piece::None(_) => true,
            _ => false
        }
    }


}

pub enum BoardSlotType {
    Capturable,
    Passable,
    Unpassable
}

impl BoardSlotType {

    /// Return `true` if this will capture a piece
    /// upon moving to that slot.
    pub fn capturable(&self) -> bool {
        match *self {
            Self::Capturable => true,
            _ => false
        }
    }

    /// Return `true` if this will move to an empty slot.
    pub fn passable(&self) -> bool {
        match *self {
            Self::Passable => true,
            _ => false
        }
    }

    /// Return `true` if this contains a piece of
    /// its same color, so it cannot be passed through.
    pub fn unpassable(&self) -> bool {
        match *self {
            Self::Unpassable => true,
            _ => false
        }
    }

    /// Return `true` if this is [`Self::Capturable`] or [`Self::Passable`].
    pub fn to_bool(&self) -> bool {
        match *self {
            Self::Capturable => true,
            Self::Passable => true,
            Self::Unpassable => false
        }
    }

}