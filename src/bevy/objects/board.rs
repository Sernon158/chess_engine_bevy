use bevy::prelude::*;
use crate::bevy::objects::PieceComponent;
use crate::engine::board::Board;

#[derive(SceneComponent, Clone, Default, FromTemplate)]
pub struct BoardComponent;


impl BoardComponent {
    pub fn scene() -> impl Scene {
        let default_board = Board::new();
        let mut all_pieces = Vec::with_capacity(64);

        for piece in default_board.concat() {
            let piece_pos = default_board.get_piece_position(&piece);
            
            all_pieces.push(bsn! {
                @PieceComponent { @piece, @piece_pos }
            });
        }

        bsn! {
            #ChessBoard
            Self
            Sprite {
                image: "textures/board.png",
                custom_size: Vec2::new(600., 600.)
            }
            
            Children [ {all_pieces} ]
        }
    }
}


impl BoardComponent {
    const STARTING_COORDINATES: Vec2 = Vec2::new(-300., -300.);
    
    pub fn get_index_coords(index: (usize, usize)) -> Vec2 {
        let mut coords = Self::STARTING_COORDINATES;

        coords.x += ((index.0 + 1) as f32 * 75.) - 37.5;
        coords.y += ((index.1 + 1) as f32 * 75.) - 37.5;

        coords
    }

    pub fn get_coords_index(coords: Vec3) -> (usize, usize) {
        let index_x = ((coords.x - Self::STARTING_COORDINATES.x + 37.5) / 75.0 - 1.0).round() as usize;
        let index_y = ((coords.y - Self::STARTING_COORDINATES.y + 37.5) / 75.0 - 1.0).round() as usize;
        
        (index_x, index_y)
    }
}