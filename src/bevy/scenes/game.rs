use bevy::prelude::*;
use crate::bevy::components::*;
use crate::bevy::objects::IdleAnimator;
use crate::bevy::objects::BoardComponent;
use crate::bevy::tools::AnimationCompleted;

pub fn game_scene() -> impl SceneList {
    bsn! {
        Camera2d
        @BoardComponent
    }
}


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


pub fn piece_move_animator(
    mut query: Query<(Entity, &mut Transform, &mut PieceMoveAnimator)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (entity, mut transform, mut animator) in &mut query {
        let timer = &mut animator.timer;

        timer.tick(time.delta());

        let t = (timer.elapsed_secs() / timer.duration().as_secs_f32())
            .clamp(0.0, 1.0);

        let ease_t = EaseFunction::SmoothStep.sample_clamped(t);
        transform.translation = animator.start.lerp(animator.end, ease_t);

        if animator.timer.is_finished() {
            commands.entity(entity)
                .trigger(AnimationCompleted::<PieceMoveAnimator>::new);
        }
    }
}