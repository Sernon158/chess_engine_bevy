use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Event)]
pub struct AnimationCompleted<T>(PhantomData<T>);

impl<T> AnimationCompleted<T> {
    pub fn new() -> Self {
        Self(PhantomData)
    }
}