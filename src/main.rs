use bevy::prelude::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, move_system)
        .run();
}

#[derive(Component)]
struct Movement {
    speed: f32,
}

fn move_system(
    mut entitie: Query<(&Movement, &mut Transform)>,
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    for (obj, mut transform) in entitie.iter_mut() {
        if keyboard.pressed(KeyCode::KeyW) {
            transform.translation.y += obj.speed * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::KeyA) {
            transform.translation.x -= obj.speed * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::KeyS) {
            transform.translation.y -= obj.speed * time.delta_seconds();
        }
        if keyboard.pressed(KeyCode::KeyD) {
            transform.translation.x += obj.speed * time.delta_seconds();
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //Ajuste camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_scale(Vec3::new(0.25, 0.25, 0.25)),
        ..default()
    });

    //samurai (Personagem principal)
    commands.spawn(SpriteBundle {
        texture: asset_server.load("samurai.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    //Ninjas (inimigos)
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load("ninja.png"),
            transform: Transform::from_xyz(-30.0, 0.0, 0.0),
            ..default()
        })
        .insert(Movement { speed: 60.0 });
}
