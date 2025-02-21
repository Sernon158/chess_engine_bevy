use bevy::prelude::*;

#[derive(Component)]
pub struct BoardComponent;

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