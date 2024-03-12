use std::f32::consts::PI;

use bevy::prelude::*;
mod fly_cam;
mod post_process;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(242, 208, 196)))
        .insert_resource(Msaa::Off)
        .add_plugins((
            DefaultPlugins,
            fly_cam::FlyCamPlugin,
            post_process::PostProcessPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_light)
        .add_systems(Update, apply_rotation)
        .run();
}

#[derive(Component)]
struct Rotates;

fn setup(
    mut cmd: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    cmd.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(100.0, 100.0)),
        material: materials.add(Color::SILVER),
        ..default()
    });

    // cube
    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::default().mesh().scaled_by(Vec3::splat(3.))),
            material: materials.add(Color::rgb_u8(89, 108, 217)),
            transform: Transform::from_xyz(0.0, 2.0, 0.0),
            ..default()
        },
        Rotates,
    ));

    // sphere
    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(
                Sphere::default()
                    .mesh()
                    .ico(9)
                    .unwrap()
                    .scaled_by(Vec3::splat(3.0)),
            ),
            material: materials.add(Color::rgb_u8(132, 119, 217)),
            transform: Transform::from_xyz(-5.0, 3.0, 0.0),
            ..default()
        },
        Rotates,
    ));

    // Torus
    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(Torus::default()),
            material: materials.add(Color::rgb_u8(217, 143, 170)),
            transform: Transform::from_xyz(5.0, 3.0, 0.0)
                .with_rotation(Quat::from_rotation_x(PI / 4.0)),
            ..default()
        },
        Rotates,
    ));
}

fn spawn_light(mut cmd: Commands) {
    cmd.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1_000_000.0,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0.0, 10.0, 0.0)),
        ..default()
    });
}

fn apply_rotation(time: Res<Time>, mut query: Query<&mut Transform, With<Rotates>>) {
    for mut transform in &mut query {
        transform.rotate_y(0.1 * time.delta_seconds())
    }
}
