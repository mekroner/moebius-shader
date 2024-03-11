use bevy::{prelude::*, render::primitives::Sphere};
mod fly_cam;
mod post_process;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
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
            material: materials.add(Color::WHITE),
            transform: Transform::from_xyz(0.0, 5.0, 0.0),
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
