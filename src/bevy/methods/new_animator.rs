use bevy::prelude::*;

use bevy_tweening::Animator;
use bevy_tweening::EaseMethod;
use bevy_tweening::Lens;
use bevy_tweening::Tween;

use std::time::Duration;

use crate::bevy::structs::AnimationLoop;

pub fn new_animator<C: Component, L: Lens<C> + Send + Sync + 'static>(
    transform_lens: L,
    seconds: f32,
    ease: EaseMethod,
    autostart: bool,
    animation_loop: AnimationLoop,
)
    -> Animator<C>
{
    let tween = Tween::new(
        ease,
        Duration::from_secs_f32(seconds),
        transform_lens,
    )
        .with_repeat_count(animation_loop.count)
        .with_repeat_strategy(animation_loop.strategy);
    
    let mut animator = Animator::new(tween);
    if !autostart { animator.set_speed(0.) };

    animator
}