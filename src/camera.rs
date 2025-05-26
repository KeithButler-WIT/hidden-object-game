use bevy::prelude::*;

const CAMERA_DISTANCE: f32 = 80.0;
const CAMERA_SPEED: f32 = 20.0;

pub struct CameraPlugin;

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true, // HDR is required for the bloom effect
            ..default()
        },
        MainCamera,
    ));
}
