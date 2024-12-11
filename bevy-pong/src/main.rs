mod custom_sensor;
mod player;
mod plugins;

use plugins::ball::BallPlugin;
use plugins::border::BorderPlugin;
use plugins::camera::CameraPlugin;
use plugins::paddle::PaddlePlugin;
use plugins::score::ScorePlugin;

use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;

pub const FONT_SIZE: f32 = 80.0;

fn main() {
    let mut app = App::new();

    #[cfg(debug_assertions)]
    app.add_plugins(RapierDebugRenderPlugin::default());

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }),
        RapierPhysicsPlugin::<NoUserData>::default(),
    ))
    .insert_resource(ClearColor(Color::srgb(0.05, 0.0, 0.0)))
    .add_plugins((
        CameraPlugin,
        ScorePlugin,
        BorderPlugin,
        PaddlePlugin,
        BallPlugin,
    ))
    .run();
}
