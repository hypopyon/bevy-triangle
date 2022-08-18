#![feature(let_else)]

mod render;

use bevy::prelude::*;
use bevy_flycam::{FlyCam, NoCameraPlayerPlugin};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(NoCameraPlayerPlugin)
        .add_plugin(render::CustomRenderPlugin::default())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    let _camera = commands.spawn()
        .insert_bundle(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 5.0)),
            ..default()
        })
        .insert(FlyCam);
}