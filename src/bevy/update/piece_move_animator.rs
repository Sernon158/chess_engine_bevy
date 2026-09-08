use bevy::prelude::*;

use crate::bevy::components::PieceMoveAnimator;
use crate::bevy::events::AnimationCompleted;
use crate::bevy::structs::Ease;

pub fn piece_move_animator(
    mut query: Query<(Entity, &mut Transform, &mut PieceMoveAnimator)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut transform, mut animator) in &mut query {
        animator.timer.tick(time.delta());

        let t = (animator.timer.elapsed_secs() / animator.timer.duration().as_secs_f32()).clamp(0.0, 1.0);
        let ease_t = Ease::in_out(t);
        transform.translation = animator.start.lerp(animator.end, ease_t);

        if animator.timer.is_finished() {
            commands.entity(entity)
                .trigger(AnimationCompleted::<PieceMoveAnimator>::new);
        }
    }
}