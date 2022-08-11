#![feature(let_else)]

mod render;

use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugin(render::CustomRenderPlugin::default())
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    let _camera = commands.spawn()
        .insert_bundle(Camera2dBundle::default());
}