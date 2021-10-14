use bevy::{ecs::system::SystemParam, prelude::*};
use std::{marker::PhantomData, time::Duration};

pub struct TimeScale(pub f32);

pub trait ScaledTimeFields {
    fn time(&self) -> &Time;
    fn time_scale(&self) -> &TimeScale;
}

pub trait ScaledTimeDelta {
    fn scale(&self) -> f32;
    fn delta(&self) -> Duration;
    fn delta_seconds(&self) -> f32;
    fn scaled_delta(&self) -> Duration;
    fn scaled_delta_seconds(&self) -> f32;
}

#[derive(SystemParam)]
pub struct ScaledTime<'w, 's> {
    pub time: Res<'w, Time>,
    pub time_scale: Res<'w, TimeScale>,

    #[system_param(ignore)]
    _phantom: PhantomData<&'s ()>
}

#[derive(SystemParam)]
pub struct ScaledTimeMut<'w, 's> {
    pub time: Res<'w, Time>,
    pub time_scale: ResMut<'w, TimeScale>,

    #[system_param(ignore)]
    _phantom: PhantomData<&'s ()>
}

impl ScaledTimeFields for ScaledTime<'_,  '_> {
    fn time(&self) -> &Time {
        &self.time
    }

    fn time_scale(&self) -> &TimeScale {
        &self.time_scale
    }
}

impl ScaledTimeFields for ScaledTimeMut<'_,  '_> {
    fn time(&self) -> &Time {
        &self.time
    }

    fn time_scale(&self) -> &TimeScale {
        &self.time_scale
    }
}

impl<T: ScaledTimeFields> ScaledTimeDelta for T {
    fn scale(&self) -> f32 {
        self.time_scale().0
    }

    fn delta(&self) -> Duration {
        self.time().delta()
    }

    fn scaled_delta(&self) -> Duration {
        self.delta().mul_f32(self.time_scale().0)
    }

    fn delta_seconds(&self) -> f32 {
        self.delta().as_secs_f32()
    }

    fn scaled_delta_seconds(&self) -> f32 {
        self.scaled_delta().as_secs_f32()
    }
}

impl ScaledTimeMut<'_,  '_> {
    pub fn set_time_scale(&mut self, scale: f32) {
        self.time_scale.0 = scale;
    }

    pub fn time_scale_mut(&mut self) -> &mut TimeScale {
        &mut self.time_scale
    }
}
