use bevy::prelude::*;
use bevy_third_person_camera::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let flashlight = (
        SpotLightBundle {
            spot_light: SpotLight {
                color: Color::rgba(1.0, 0.96, 0.37, 1.0),
                intensity: 2000.0,
                outer_angle: 0.6,
                inner_angle: 0.5,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, -0.5),
            ..default()
        },
        Name::new("Flashlight"),
    );

    let player = (
        SceneBundle {
            scene: assets.load("Player.gltf#Scene0"),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Player,
        ThirdPersonCameraTarget,
        Speed(2.5),
        Name::new("Player"),
    );

    commands.spawn(player).with_children(|parent| {
        parent.spawn(flashlight);
    });
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
    cam_q: Query<&Transform, (With<Camera3d>, Without<Player>)>,
) {
    for (mut player_transform, player_speed) in player_q.iter_mut() {
        let cam = match cam_q.get_single() {
            Ok(c) => c,
            Err(e) => Err(format!("error retrieving camera: {}", e)).unwrap(),
        };

        let mut direction = Vec3::ZERO;

        // forward
        if keys.pressed(KeyCode::W) {
            direction += cam.forward();
        }

        // back
        if keys.pressed(KeyCode::S) {
            direction += cam.back();
        }

        // left
        if keys.pressed(KeyCode::A) {
            direction += cam.left();
        }

        // right
        if keys.pressed(KeyCode::D) {
            direction += cam.right();
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        player_transform.translation += movement;

        // rotate player to face direction of motion
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y);
        }
    }
}
