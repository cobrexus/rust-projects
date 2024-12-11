use crate::{custom_sensor::CustomSensor, player::Player, plugins::score::GainPoint};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const BALL_RADIUS: f32 = 20.0;
const BALL_INITIAL_POSITION: Vec3 = Vec3::new(0.0, 0.0, 1.0);
const BALL_MOVEMENT_SPEED: f32 = 500.0;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(Update, detect_ball_hit);
    }
}

#[derive(Component)]
struct Ball;

fn spawn_ball(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("ball.png"),
            custom_size: Some(Vec2::new(BALL_RADIUS * 2.0, BALL_RADIUS * 2.0)),
            ..default()
        },
        Transform::from_translation(BALL_INITIAL_POSITION),
        Ball,
        RigidBody::Dynamic,
        CollidingEntities::default(),
        ActiveEvents::COLLISION_EVENTS,
        Collider::ball(BALL_RADIUS),
        Velocity::linear(Vec2::new(BALL_MOVEMENT_SPEED, 0.0)),
        Restitution {
            coefficient: 1.0,
            combine_rule: CoefficientCombineRule::Max,
        },
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
    ));
}

fn detect_ball_hit(
    balls: Query<&CollidingEntities, With<Ball>>,
    goals: Query<&Player, With<CustomSensor>>,
    mut game_event: EventWriter<GainPoint>,
) {
    for ball in &balls {
        for hit in ball.iter() {
            if let Ok(player) = goals.get(hit) {
                game_event.send(GainPoint(*player));
            }
        }
    }
}
