use bevy::prelude::*;

use crate::bevy::components::IdleAnimator;

pub fn idle_animator(
    mut query: Query<(&mut Transform, &mut IdleAnimator)>,
    time: Res<Time>
) {
    for (mut transform, mut animator) in &mut query {
        animator.elapsed_time += time.delta_secs();

        transform.translation.y += 2.
            * time.delta_secs()
            * (
                (animator.elapsed_time + animator.random_offset)
                * std::f32::consts::TAU / 5.
            ).sin();
    }
}