use crate::pieces::{ Piece, Color };

#[derive(Clone, Copy)]
pub struct Board(pub [[Piece; 8]; 8]);

impl Board {

    pub fn new() -> Self {
        use Piece::*;
        use Color::*;
        
        Self([
            [Rook(White, 1), Knight(White, 2), Bishop(White, 3), Queen(White, 4), King(White, 5), Bishop(White, 6), Knight(White, 7), Rook(White, 8)],
            [Pawn(White, 9), Pawn(White, 10), Pawn(White, 11), Pawn(White, 12), Pawn(White, 13), Pawn(White, 14), Pawn(White, 15), Pawn(White, 16)],
            [None(17), None(18), None(19), None(20), None(21), None(22), None(23), None(24)],
            [None(25), None(26), None(27), None(28), None(29), None(30), None(31), None(32)],
            [None(33), None(34), None(35), None(36), None(37), None(38), None(39), None(40)],
            [None(41), None(42), None(43), None(44), None(45), None(46), None(47), None(48)],
            [Pawn(Black, 49), Pawn(Black, 50), Pawn(Black, 51), Pawn(Black, 52), Pawn(Black, 53), Pawn(Black, 54), Pawn(Black, 55), Pawn(Black, 56)],
            [Rook(Black, 57), Knight(Black, 58), Bishop(Black, 59), Queen(Black, 60), King(Black, 61), Bishop(Black, 62), Knight(Black, 63), Rook(Black, 64)]
        ])
    }

    pub fn get(&self, x: usize, y: usize) -> &Piece {
        self.0
            .get(y).expect(format!("Invalid Y: {y}.").as_str())
            .get(x).expect(format!("Invalid X: {x}.").as_str())
    }

    pub fn set(&mut self, x: usize, y: usize, new_piece: Piece) {
        self.get(x, y);
        
        self.0[y][x] = new_piece;
    }

    pub fn concat(&self) -> Vec<Piece> {
        self.0.concat()
    }

    pub fn get_piece_by_id(&self, id: u8) -> Option<Piece> {
        let state = self.concat();

        state.into_iter().find(|piece| {
            let (_, piece_id) = piece.get_data();

            *piece_id == id
        })
    }

    pub fn get_piece_position(&self, piece: &Piece) -> (usize, usize) {
        let mut line_index = 0;
        let mut column_index = 0;

        'line_loop: for line in self.0.iter() {
            for board_piece in line {
                let (_, id1) = piece.get_data();
                let (_, id2) = board_piece.get_data();
                
                if id1 == id2 { break 'line_loop };

                column_index += 1;
            }

            column_index = 0;
            line_index += 1;
        }

        (column_index, line_index)
    }

}