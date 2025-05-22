use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_light, spawn_floor, spawn_objects));
    }
}

fn spawn_light(mut commands: Commands) {
    let light = (
        PointLightBundle {
            point_light: PointLight {
                color: Color::rgba(1.0, 0.78, 0.0, 1.0),
                intensity: 100.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
            ..default()
        },
        Name::new("Main Light"),
    );

    commands.spawn(light);
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.0))),
            material: materials.add(Color::DARK_GREEN.into()),
            ..default()
        },
        Name::new("Floor"),
    );

    commands.spawn(floor);
}

fn spawn_objects(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut create_cube =
        |size: f32, color: Color, transform: Transform, name: &'static str| -> (PbrBundle, Name) {
            (
                PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube::new(size))),
                    material: materials.add(color.into()),
                    transform,
                    ..default()
                },
                Name::new(name),
            )
        };

    commands.spawn(create_cube(
        4.0,
        Color::BLUE,
        Transform::from_xyz(-5.0, 2.0, 5.0),
        "Blue Cube",
    ));

    commands.spawn(create_cube(
        2.0,
        Color::RED,
        Transform::from_xyz(6.0, 1.0, -6.0),
        "Red Cube",
    ));
}
