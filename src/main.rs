// GOAL: clicker version of the hidden object cat finding games
mod camera;
mod mouse;

use bevy::{
    ecs::system::entity_command::{observe, remove},
    prelude::*,
    sprite,
};
use rand::Rng;
use std::{fmt::Debug, ops::Deref};

use camera::CameraPlugin;
use mouse::MousePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(CameraPlugin)
        .add_plugins(MousePlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
pub struct Clickable;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::rng();

    let tile_size = Vec2::splat(64.0);
    let map_size = Vec2::splat(320.0);

    let half_x = (map_size.x / 2.0) as i32;
    let half_y = (map_size.y / 2.0) as i32;

    let sprite_handle = asset_server.load("branding/bevy_bird_dark.png");

    // Builds and spawns the sprites
    // let mut sprites = vec![];
    for y in -half_y..half_y {
        for x in -half_x..half_x {
            let position = Vec2::new(x as f32, y as f32);
            let translation = (position * tile_size).extend(rng.random::<f32>());
            let rotation = Quat::from_rotation_z(rng.random::<f32>());
            let scale = Vec3::splat(rng.random::<f32>() * 2.0);

            // Spawns half the time
            if rng.random::<u32>() % 2 == 0 {
                // sprites.push((
                //     Sprite {
                //         image: sprite_handle.clone(),
                //         custom_size: Some(tile_size),
                //         ..default()
                //     },
                //     Transform {
                //         translation,
                //         rotation,
                //         scale,
                //     },
                //     Clickable,
                //     Pickable::default(),
                //     observe(delete_on::<Pointer<Pressed>>(commands)),
                // ));

                commands
                    .spawn((
                        Sprite {
                            image: sprite_handle.clone(),
                            custom_size: Some(tile_size),
                            ..default()
                        },
                        Transform {
                            translation,
                            rotation,
                            scale,
                        },
                        Clickable,
                        Pickable::default(),
                    ))
                    .observe(delete_on::<Pointer<Pressed>>());
            }
        }
    }

    // commands.spawn_batch(sprites);
}

// An observer listener that despawns the target entity
fn delete_on<E: Debug + Clone + Reflect>()
-> impl Fn(Trigger<E>, Query<Entity, With<Clickable>>, Commands) {
    move |ev, mut sprites, mut commands| {
        let Ok(sprite) = sprites.get_mut(ev.target()) else {
            return;
        };
        commands.entity(sprite).despawn();
    }
}
