// GOAL: clicker version of the hidden object cat finding games
mod camera;
mod hidden_object;
mod mouse;

use bevy::prelude::*;

use camera::CameraPlugin;
use hidden_object::HiddenObjectPlugin;
use mouse::MousePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(CameraPlugin)
        .add_plugins(HiddenObjectPlugin)
        // .add_plugins(MousePlugin)
        // .add_systems(Startup, setup)
        .run();
}
