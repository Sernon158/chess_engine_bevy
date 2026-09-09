use crate::engine::pieces::{ Color, Piece, PieceKind };

#[derive(Clone, Copy)]
pub struct Board(pub [[Piece; 8]; 8]);

impl Board {

    pub fn new() -> Self {
        use PieceKind::*;
        use Color::*;

        macro_rules! board {
            ($( $color:ident => [ $( $piece:ident ),* ] ),*) => {{
                let mut id = 0;

                Self([ $([ $({
                    id += 1;
                    Piece::new(id, $piece, $color)
                }),* ]),* ])
            }}
        }

        board![
            White => [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook],
            White => [Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn],
            Empty => [None, None, None, None, None, None, None, None],
            Empty => [None, None, None, None, None, None, None, None],
            Empty => [None, None, None, None, None, None, None, None],
            Empty => [None, None, None, None, None, None, None, None],
            Black => [Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn, Pawn],
            Black => [Rook, Knight, Bishop, Queen, King, Bishop, Knight, Rook]
        ]
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

        state.into_iter().find(|piece| piece.id == id)
    }

    pub fn get_piece_position(&self, piece: &Piece) -> (usize, usize) {
        let mut line_index = 0;
        let mut column_index = 0;

        'line_loop: for line in self.0.iter() {
            for board_piece in line {
                if piece.id == board_piece.id { break 'line_loop };

                column_index += 1;
            }

            column_index = 0;
            line_index += 1;
        }

        (column_index, line_index)
    }

}