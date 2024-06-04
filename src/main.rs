use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Ajuste camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec3::new(0.25, 0.25, 0.25)),
        ..default()
    });

    //samurai (Personagem principal)
    commands.spawn(SpriteBundle {
        texture: asset_server.load("little_heroes.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    //Ninjas (inimigos)
    commands.spawn(SpriteBundle {
        texture: asset_server.load("ninja.png"),
        transform: Transform::from_xyz(-30.0, 0.0, 0.0),
        ..default()
    });
}
