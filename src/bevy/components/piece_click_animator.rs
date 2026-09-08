use bevy::prelude::*;
use bevy_tweening::TweenAnim;

#[derive(Component)]
pub struct PieceClickAnimator(pub TweenAnim);