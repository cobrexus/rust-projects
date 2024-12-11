use crate::{custom_sensor::CustomSensor, player::Player, WINDOW_HEIGHT, WINDOW_WIDTH};

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const BORDER_THICKNESS: f32 = 6.0;

pub struct BorderPlugin;

impl Plugin for BorderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_borders);
    }
}

fn spawn_borders(mut commands: Commands) {
    // Bottom border
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, WINDOW_HEIGHT / 2.0, 0.0)),
        RigidBody::Fixed,
        Collider::cuboid(WINDOW_WIDTH / 2.0, BORDER_THICKNESS / 2.0),
    ));

    // Top border
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, -WINDOW_HEIGHT / 2.0, 0.0)),
        RigidBody::Fixed,
        Collider::cuboid(WINDOW_WIDTH / 2.0, BORDER_THICKNESS / 2.0),
    ));

    // Right border
    commands.spawn((
        Transform::from_translation(Vec3::new(WINDOW_WIDTH / 2.0, 0.0, 0.0)),
        RigidBody::Fixed,
        Collider::cuboid(BORDER_THICKNESS / 2.0, WINDOW_HEIGHT / 2.0),
        Player::Player1,
        CustomSensor,
    ));

    // Left border
    commands.spawn((
        Transform::from_translation(Vec3::new(-WINDOW_WIDTH / 2.0, 0.0, 0.0)),
        RigidBody::Fixed,
        Collider::cuboid(BORDER_THICKNESS / 2.0, WINDOW_HEIGHT / 2.0),
        Player::Player2,
        CustomSensor,
    ));
}
