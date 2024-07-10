use crate::components::movement::Movement;
use crate::components::velocity::Velocity;
use bevy::math::Vec3;
use bevy::prelude::*;

pub fn keyboard_input(
    mut entitie: Query<(&Movement, &mut Velocity)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (mov, mut vel) in entitie.iter_mut() {
        vel.0 = Vec3::default();
        if keyboard.pressed(KeyCode::KeyW) {
            vel.0.y += mov.speed;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            vel.0.x -= mov.speed;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            vel.0.y -= mov.speed;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            vel.0.x += mov.speed;
        }
    }
}

pub fn z_index(mut entitie: Query<&mut Transform, Without<Camera>>) {
    for mut transform in entitie.iter_mut() {
        transform.translation.z = -1.0 * transform.translation.y;
    }
}

pub fn moviment(mut entitie: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (vel, mut transform) in entitie.iter_mut() {
        transform.translation += vel.0 * time.delta_seconds();
        if vel.0.x > 0.0 {
            transform.scale = Vec3::new(1.0, 1.0, 1.0);
        } else if vel.0.x < 0.0 {
            transform.scale = Vec3::new(-1.0, 1.0, 1.0)
        }

        if vel.0.length() > 0.0 {
            let phase = (time.elapsed_seconds() * 20.0).sin();
            transform.scale.y = phase.remap(-1.0, 1.0, 0.9, 1.0)
        }
    }
}
