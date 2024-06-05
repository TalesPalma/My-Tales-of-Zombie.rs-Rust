use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, move_system)
        .run();
}

#[derive(Component)]
struct Moveable {
    speed: f32,
}

fn move_system(mut entties: Query<(&Moveable, &mut Transform)>, time: Res<Time>) {
    for (ninja, mut transform) in entties.iter_mut() {
        transform.translation.x += ninja.speed * time.delta_seconds();
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
        .insert(Moveable { speed: 1.0 });

    //Ninjas (inimigos)
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("ninja.png"),
            transform: Transform::from_xyz(-30.0, 0.0, 0.0),
            ..default()
        })
        .insert(Moveable { speed: 1.0 });
}
