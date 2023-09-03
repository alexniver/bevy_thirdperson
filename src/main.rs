use bevy::{prelude::*, window::close_on_esc};
use bevy_third_person_camera::ThirdPersonCameraPlugin;
use thirdperson::{camera::CameraPlugin, player::PlayerPlugin, world::WorldPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ThirdPersonCameraPlugin,
            CameraPlugin,
            WorldPlugin,
            PlayerPlugin,
        ))
        .add_systems(Update, close_on_esc)
        .run();
}
