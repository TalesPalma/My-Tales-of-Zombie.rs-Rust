use bevy::math::Vec3;
use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);
