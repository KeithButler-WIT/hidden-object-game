use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_sprite_count);
    }
}

#[derive(Deref, DerefMut)]
struct PrintingTimer(Timer);

impl Default for PrintingTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}
// System for printing the number of sprites on every tick of the timer
fn print_sprite_count(time: Res<Time>, mut timer: Local<PrintingTimer>, sprites: Query<&Sprite>) {
    timer.tick(time.delta());

    if timer.just_finished() {
        info!("Sprites: {}", sprites.iter().count());
    }
}
