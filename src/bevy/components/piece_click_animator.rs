use bevy::prelude::*;

use bevy_tweening::Animator;

#[derive(Component)]
pub struct PieceClickAnimator(pub Animator<Transform>);