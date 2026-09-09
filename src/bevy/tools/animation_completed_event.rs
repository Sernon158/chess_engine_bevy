use std::marker::PhantomData;
use bevy::prelude::*;

#[derive(EntityEvent)]
pub struct AnimationCompleted<T>{
    pub entity: Entity,

    #[doc(hidden)]
    _t: PhantomData<T>
}

impl<T> AnimationCompleted<T> {
    pub fn new(entity: Entity) -> Self {
        Self {
            entity,
            _t: PhantomData
        }
    }
}