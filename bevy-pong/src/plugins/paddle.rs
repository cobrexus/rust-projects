use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const GAP_BETWEEN_PADDLES_AND_WALL: f32 = 20.0;
const PADDLE_WIDTH: f32 = 20.0;
const PADDLE_HEIGHT: f32 = 128.0;
const PADDLE_MOVEMENT_SPEED: f32 = 1000.0;

pub struct PaddlePlugin;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_paddles)
            .add_systems(Update, move_paddle);
    }
}

#[derive(Component)]
struct Paddle {
    move_up: KeyCode,
    move_down: KeyCode,
}

fn spawn_paddles(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("paddle.png"),
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            (-WINDOW_WIDTH / 2.0) + GAP_BETWEEN_PADDLES_AND_WALL,
            0.0,
            1.0,
        )),
        Paddle {
            move_up: KeyCode::KeyW,
            move_down: KeyCode::KeyS,
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(PADDLE_WIDTH / 2.0, PADDLE_HEIGHT / 2.0),
    ));

    commands.spawn((
        Sprite {
            image: asset_server.load("paddle.png"),
            custom_size: Some(Vec2::new(PADDLE_WIDTH, PADDLE_HEIGHT)),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            (WINDOW_WIDTH / 2.0) - GAP_BETWEEN_PADDLES_AND_WALL,
            0.0,
            1.0,
        )),
        Paddle {
            move_up: KeyCode::ArrowUp,
            move_down: KeyCode::ArrowDown,
        },
        RigidBody::KinematicPositionBased,
        Collider::cuboid(PADDLE_WIDTH / 2.0, PADDLE_HEIGHT / 2.0),
    ));
}

fn move_paddle(
    mut paddles: Query<(&mut Transform, &Paddle)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, paddle) in &mut paddles {
        if input.pressed(paddle.move_up) {
            transform.translation.y += PADDLE_MOVEMENT_SPEED * time.delta_secs();
            transform.translation.y = transform.translation.y.clamp(
                -(WINDOW_HEIGHT / 2.0) + (PADDLE_HEIGHT / 2.0) + GAP_BETWEEN_PADDLES_AND_WALL,
                (WINDOW_HEIGHT / 2.0) - (PADDLE_HEIGHT / 2.0) - GAP_BETWEEN_PADDLES_AND_WALL,
            );
        }

        if input.pressed(paddle.move_down) {
            transform.translation.y -= PADDLE_MOVEMENT_SPEED * time.delta_secs();
            transform.translation.y = transform.translation.y.clamp(
                -(WINDOW_HEIGHT / 2.0) + (PADDLE_HEIGHT / 2.0) + GAP_BETWEEN_PADDLES_AND_WALL,
                (WINDOW_HEIGHT / 2.0) - (PADDLE_HEIGHT / 2.0) - GAP_BETWEEN_PADDLES_AND_WALL,
            );
        }
    }
}
