mod components;
mod systems;
use crate::systems::moviment_systems;
use bevy::prelude::*;
use components::{movement::Movement, velocity::Velocity};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, moviment_systems::z_index)
        .add_systems(Update, moviment_systems::keyboard_input)
        .add_systems(
            Update,
            moviment_systems::moviment.after(moviment_systems::keyboard_input),
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Ajuste camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec3::new(0.25, 0.25, 0.25)),
        ..default()
    });

    //samurai (Personagem principal)
    let insert = commands
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
