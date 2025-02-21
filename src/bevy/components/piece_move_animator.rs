use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct PieceMoveAnimator {
    pub timer: Timer,
    pub start: Vec3,
    pub end: Vec3
}

impl PieceMoveAnimator {
    pub fn new(start: Vec3, end: Vec3) -> Self {
        Self {
            timer: Timer::new(
                Duration::from_secs_f32(0.25),
                TimerMode::Once
            ),
            start,
            end
        }
    }
}