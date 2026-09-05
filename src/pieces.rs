use crate::{board::Board, game::StandardGame};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Piece {
    pub id: u8,
    pub kind: PieceKind,
    pub color: Color
}

impl Piece {
    pub fn pawn(id: u8, color: Color) -> Self {
        Self::new(id, PieceKind::Pawn, color)
    }
    pub fn rook(id: u8, color: Color) -> Self {
        Self::new(id, PieceKind::Rook, color)
    }
    pub fn knight(id: u8, color: Color) -> Self {
        Self::new(id, PieceKind::Knight, color)
    }
    pub fn bishop(id: u8, color: Color) -> Self {
        Self::new(id, PieceKind::Bishop, color)
    }
    pub fn queen(id: u8, color: Color) -> Self {
        Self::new(id, PieceKind::Queen, color)
    }
    pub fn king(id: u8, color: Color) -> Self {
        Self::new(id, PieceKind::King, color)
    }
    pub fn none(id: u8) -> Self {
        Self::new(id, PieceKind::None, Color::Empty)
    }

    pub fn new(id: u8, kind: PieceKind, color: Color) -> Self {
        Self { id, kind, color }
    }

    /// Could this piece, if it was (hypothetically) placed
    /// in `current_pos`, move to `target_pos`?
    pub fn could_move_to(
        kind: PieceKind,
        color: Color,
        target_pos: (usize, usize),
        current_pos: (usize, usize),
        game: &StandardGame
    ) -> bool {
        Piece::new(u8::MAX, kind, color)
            .can_move(target_pos, current_pos, game, false)
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum PieceKind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    None
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

        let color = self.color;

        // Closure to sum `plus` to `slot`. If it's a black piece,
        // substract the value from it instead to invert it.
        let slotsum = |slot: usize, plus: i8| match color {
            Color::White => (slot as i8 + plus) as usize,
            Color::Black => (slot as i8 - plus) as usize,
            _ => 0
        };
        
        let piece_can_move = || match self.kind {
            PieceKind::Pawn => {
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

                    return target.color != color
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

            PieceKind::Rook => {
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

            PieceKind::Knight => {
                let target = self.is_passable(&game.board, target_pos);
                let x_diff = (x as i8 - curr_x as i8).abs();
                let y_diff = (y as i8 - curr_y as i8).abs();

                return target.to_bool()
                    && ((x_diff == 2 && y_diff == 1) || (x_diff == 1 && y_diff == 2))
            },

            PieceKind::Bishop => {
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

            PieceKind::Queen => {
                let x_diff = (x as i8 - curr_x as i8).abs();
                let y_diff = (y as i8 - curr_y as i8).abs();

                // Horizontal / Vertical
                if (x == curr_x && y != curr_y) || (x != curr_x && y == curr_y) {
                    // Delegate the check to the Rook piece
                    return Piece::could_move_to(
                        PieceKind::Rook, color, target_pos, current_pos, game
                    )
                }
                // Diagonal
                else if x_diff == y_diff {
                    // Delegate the check to the Bishop piece
                    return Piece::could_move_to(
                        PieceKind::Bishop, color, target_pos, current_pos, game
                    )
                }

                return false
            },

            PieceKind::King => {
                let x_diff = (x as i8 - curr_x as i8).abs();
                let y_diff = (y as i8 - curr_y as i8).abs();
                let dest = self.is_passable(&game.board, (x, y));

                return x_diff <= 1 && y_diff <= 1 && dest.to_bool()
            },

            PieceKind::None => return false
        };

        piece_can_move() && (
               !look_for_checks
            || (look_for_checks && !game.is_king_checked_after_move(color, (curr_x, curr_y), (x, y)))
        )

        // TODO: Add a check for the king checked. If the king (of same
        // color) is still checked after this `match`, it returns false.
    }


    pub fn get_display(&self) -> Option<String> {
        let color = match self.color {
            Color::Black => "_black",
            _ => ""
        };

        let piece = match self.kind {
            PieceKind::Pawn => "pawn",
            PieceKind::Rook => "rook",
            PieceKind::Knight => "knight",
            PieceKind::Bishop => "bishop",
            PieceKind::Queen => "queen",
            PieceKind::King => "king",
            PieceKind::None => return None
        };

        Some(format!("{piece}{color}"))
    }

    pub fn set_id(&mut self, new_id: u8) {
        self.id = new_id;
    }

    /// Check if the board slot `(x, y)` is passable by this piece.
    pub fn is_passable(&self, board: &Board, (x, y): (usize, usize)) -> BoardSlotType {
        let board_piece = board.get(x, y);
        if board_piece.is_none() { return BoardSlotType::Passable };

        match self.color == board_piece.color {
            true => BoardSlotType::Unpassable,
            false => BoardSlotType::Capturable
        }
    }

    pub fn is_none(&self) -> bool {
        matches!(self.kind, PieceKind::None)
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