use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Clone)]
pub struct IdleAnimator {
    pub elapsed_time: f32,
    pub random_offset: f32
}

impl Default for IdleAnimator {
    fn default() -> Self {
        let mut rng = rand::rng();
        let random_offset = rng.random_range(0.0..2.0);

        Self {
            elapsed_time: 0.,
            random_offset
        }
    }
}