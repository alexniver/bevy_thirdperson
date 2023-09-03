use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_floor, setup_light_dir, setup_box));
    }
}

fn setup_floor(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshs.add(shape::Plane::from_size(20.0).into()),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    });
}

fn setup_light_dir(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 5000.0,
            ..default()
        },
        transform: Transform::from_xyz(1.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         color: Color::WHITE,
    //         intensity: 3000.0,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(0.0, 3.0, 0.0),
    //     ..default()
    // });
}

fn setup_box(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut box_spawn = |color: Color, pos: (f32, f32, f32)| -> PbrBundle {
        PbrBundle {
            mesh: meshs.add(shape::Cube::default().into()),
            material: materials.add(color.into()),
            transform: Transform::from_xyz(pos.0, pos.1, pos.2),
            ..default()
        }
    };
    commands.spawn(box_spawn(Color::RED, (2.0, 0.5, 2.0)));
    commands.spawn(box_spawn(Color::GRAY, (-2.0, 0.5, -2.0)));
}
