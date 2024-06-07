use bevy::{prelude::*};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, z_index_system)
        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, moviment_system.after(keyboard_input_system))
        .run();
}

#[derive(Component)]
struct Movement {
    speed: f32,
}

#[derive(Component, Default)]
struct Velocity(Vec3);

fn keyboard_input_system(
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

fn z_index_system(mut entitie: Query<&mut Transform, Without<Camera>>) {
    for mut transform in entitie.iter_mut() {
        transform.translation.z = -1.0 * transform.translation.y;
    }
}

fn moviment_system(mut entitie: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (vel, mut transform) in entitie.iter_mut() {
        transform.translation += vel.0 * time.delta_seconds();
        if vel.0.x > 0.0 {
            transform.scale = Vec3::new(1.0, 1.0, 1.0);
        } else if vel.0.x < 0.0 {
            transform.scale = Vec3::new(-1.0, 1.0, 1.0)
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Ola pesosal");

    //Ajuste camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec3::new(0.25, 0.25, 0.25)),
        ..default()
    });

    //samurai (Personagem principal)
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("samurai.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Velocity::default())
        .insert(Movement { speed: 40.0 });
    // Zombie village
    commands.spawn(SpriteBundle {
        texture: asset_server.load("zombie_village.png"),
        transform: Transform::from_xyz(-40.0, 0.0, 0.0),
        ..default()
    });

    //Ninjas (inimigos)
    commands.spawn(SpriteBundle {
        texture: asset_server.load("ninja.png"),
        transform: Transform::from_xyz(-30.0, 0.0, 0.0),
        ..default()
    });
}
