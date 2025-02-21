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
}

impl Piece {


    pub fn can_move(
        &self,
        target_pos: (usize, usize),
        current_pos: (usize, usize),
        game: &StandardGame
    ) -> bool {
        let (x, y) = target_pos;
        let (curr_x, curr_y) = current_pos;

        if x > 7 || y > 7 { return false };
        if target_pos == current_pos { return false };
        
        match self {
            Piece::Pawn(color, _) => {
                if let Color::White = color {
                    //// En passant
                    //if let Some((x, y)) = game.en_passant {
                    //    return (x == curr_x - 1 || x == curr_x - 1) && (y == curr_y + 1 || y == curr_y - 1)
                    //}

                    // Capture
                    if (curr_x > 1 && x == curr_x - 1 || curr_x < 7 && x == curr_x + 1)
                       && y == curr_y + 1
                    {
                        let target = game.board.get(x, y);
                        if target.is_none() { return false };

                        let (target_color, _) = target.get_data();

                        return target_color != color
                    }

                    // Normal Pawn Movement
                    if curr_y == 1 {
                        let target = self.is_passable(&game.board, target_pos);

                        if y == 2 { return x == curr_x && target.passable() }
                        else if y == 3 {
                            let prev_target = self.is_passable(&game.board, (x, y - 1));
                        
                            return x == curr_x && target.to_bool() && prev_target.to_bool()
                        }

                        return false
                    } else {
                        let one_step = self.is_passable(&game.board, target_pos);

                        return x == curr_x && y == curr_y + 1 && one_step.to_bool()
                    }
                }
                
                else {
                    //// En passant
                    //if let Some((x, y)) = game.en_passant {
                    //    return (x == curr_x - 1 || x == curr_x - 1) && (y == curr_y + 1 || y == curr_y - 1)
                    //}

                    // Capture
                    if (curr_x > 1 && x == curr_x - 1 || curr_x < 7 && x == curr_x + 1)
                       && y == curr_y - 1
                    {
                        let target = game.board.get(x, y);
                        if target.is_none() { return false };

                        let (target_color, _) = target.get_data();

                        return target_color != color
                    }

                    // Normal Pawn Movement
                    if curr_y == 7 {
                        let target = self.is_passable(&game.board, target_pos);

                        if y == 2 { return x == curr_x && target.to_bool() }
                        else if y == 3 {
                            let prev_target = self.is_passable(&game.board, (x, y + 1));
                        
                            return x == curr_x && target.to_bool() && prev_target.to_bool()
                        }

                        return false
                    } else {
                        let one_step = self.is_passable(&game.board, target_pos);

                        return x == curr_x && y == curr_y - 1 && one_step.to_bool()
                    }
                }
            },

            //
            // TODO: Add self.is_passable conditions to ALL pieces except
            // pawn and knight, which already have it.
            //

            Piece::Rook(_, _) => {
                let x_diff = x as i8 - curr_x as i8;
                let y_diff = y as i8 - curr_y as i8;

                if y == curr_y && x != curr_x {
                    let range = 1..x_diff.abs();

                    for coord in range {
                        let new_x = if x_diff >= 0 { curr_x + coord as usize } else { curr_x - coord as usize };

                        let target = self.is_passable(&game.board, (new_x, y));
                        if !target.passable() { return false };
                    }
                }

                else if x == curr_x && y != curr_y {
                    let range = 1..y_diff.abs();

                    for coord in range {
                        let new_y = if y_diff >= 0 { curr_y + coord as usize } else { curr_y - coord as usize };

                        let target = self.is_passable(&game.board, (x, new_y));
                        if !target.passable() { return false };
                    }
                }

                else { return false };

                let target = self.is_passable(&game.board, (x, y));
                if !target.passable() && !target.capturable() { return false };
                
                true
            },

            Piece::Knight(_, _) => {
                let target = self.is_passable(&game.board, target_pos);
                let x_diff = (x as i8 - curr_x as i8).abs();
                let y_diff = (y as i8 - curr_y as i8).abs();

                target.to_bool() && ((x_diff == 2 && y_diff == 1) || (x_diff == 1 && y_diff == 2))
            },

            Piece::Bishop(_, _) => {
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
                if !dest.passable() && !dest.capturable() { return false };
            
                true
            },

            Piece::Queen(_, _) => {
                let x_diff = (x as i8 - curr_x as i8).abs();
                let y_diff = (y as i8 - curr_y as i8).abs();

                (x == curr_x && y != curr_y) || (y == curr_y && x != curr_x) || x_diff == y_diff
            },

            Piece::King(_color, _) => {
                let x_diff = (x as i8 - curr_x as i8).abs();
                let y_diff = (y as i8 - curr_y as i8).abs();

                x_diff <= 1 && y_diff <= 1
            },

            Piece::None(_) => false
        }

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

    pub fn capturable(&self) -> bool {
        match *self {
            Self::Capturable => true,
            _ => false
        }
    }

    pub fn passable(&self) -> bool {
        match *self {
            Self::Passable => true,
            _ => false
        }
    }

    pub fn unpassable(&self) -> bool {
        match *self {
            Self::Unpassable => true,
            _ => false
        }
    }

    pub fn to_bool(&self) -> bool {
        match *self {
            Self::Capturable => true,
            Self::Passable => true,
            Self::Unpassable => false
        }
    }

}