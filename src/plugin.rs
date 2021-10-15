use bevy::{
    input::{
        InputSystem,
    },
    prelude::*,
};
use crate::TimeScale;

pub struct TimePlugin;
impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TimeScale(1.));
    }
}
