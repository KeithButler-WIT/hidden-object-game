//! Prints mouse button events.

use crate::camera::MainCamera;

use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseButtonInput},
    prelude::*,
};

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (click_sprite, mouse_click_system, mouse_move_system),
        );
    }
}

fn handle_mouse_clicks(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    window_query: Query<&Window, With<MainCamera>>,
) {
    let win = window_query.single().unwrap();
    if mouse_button_input.pressed(MouseButton::Left) {
        info!("click at {:?}", win.cursor_position());
    }
}

fn click_sprite(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    sprites: Query<&Sprite>,
) {
    for event in mouse_button_input_events.read() {
        info!("{:?}", event);
    }
}

// This system prints messages when you press or release the left mouse button:
fn mouse_click_system(mouse_button_input: Res<ButtonInput<MouseButton>>) {
    if mouse_button_input.pressed(MouseButton::Left) {
        info!("left mouse currently pressed");
    }

    if mouse_button_input.just_pressed(MouseButton::Left) {
        info!("left mouse just pressed");
    }

    if mouse_button_input.just_released(MouseButton::Left) {
        info!("left mouse just released");
    }
}

// This system prints messages when you finish dragging or scrolling with your mouse
fn mouse_move_system(
    accumulated_mouse_motion: Res<AccumulatedMouseMotion>,
    accumulated_mouse_scroll: Res<AccumulatedMouseScroll>,
) {
    if accumulated_mouse_motion.delta != Vec2::ZERO {
        let delta = accumulated_mouse_motion.delta;
        info!("mouse moved ({}, {})", delta.x, delta.y);
    }
    if accumulated_mouse_scroll.delta != Vec2::ZERO {
        let delta = accumulated_mouse_scroll.delta;
        info!("mouse scrolled ({}, {})", delta.x, delta.y);
    }
}
