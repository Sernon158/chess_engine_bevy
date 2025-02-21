use bevy_tweening::RepeatCount;
use bevy_tweening::RepeatStrategy;

pub struct AnimationLoop {
    pub count: RepeatCount,
    pub strategy: RepeatStrategy
}

impl AnimationLoop {
    pub fn new(count: RepeatCount, strategy: RepeatStrategy) -> Self {
        Self {
            count,
            strategy
        }
    }

    pub fn one_run() -> Self {
        Self {
            count: RepeatCount::Finite(1),
            strategy: RepeatStrategy::MirroredRepeat
        }
    }
}