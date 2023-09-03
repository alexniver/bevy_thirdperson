use bevy::prelude::*;
use bevy_third_person_camera::ThirdPersonCameraTarget;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, player_move);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Speed {
    pub value: f32,
}

impl Speed {
    pub fn new(value: f32) -> Self {
        Self { value }
    }
}

fn setup_player(
    mut commands: Commands,
    mut meshs: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshs.add(shape::Cube::default().into()),
            material: materials.add(Color::BLUE.into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0).with_scale(Vec3::new(0.5, 1.0, 0.5)),
            ..default()
        },
        Player,
        Speed::new(5.0),
        ThirdPersonCameraTarget,
    ));
}

fn player_move(
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
    camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut move_delta = Vec3::ZERO;

    let (mut player_transform, player_speed) = player_query.single_mut();
    let camera_transform = camera_query.single();
    if keyboard.pressed(KeyCode::A) {
        move_delta += camera_transform.left();
    }
    if keyboard.pressed(KeyCode::D) {
        move_delta += camera_transform.right();
    }
    if keyboard.pressed(KeyCode::W) {
        move_delta += camera_transform.forward();
    }
    if keyboard.pressed(KeyCode::S) {
        move_delta += camera_transform.back();
    }

    move_delta.y = 0.0;
    player_transform.translation += move_delta * player_speed.value * time.delta_seconds();
}
