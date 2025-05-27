use bevy::prelude::*;

pub mod camera;
pub mod input_dispatch;
pub mod scene;
pub mod settings;
pub mod sound;

pub fn plugin(app: &mut App) {
    app.insert_resource(Score(0));
    app.add_plugins((
        settings::plugin,
        camera::plugin,
        scene::plugin,
        sound::plugin,
        input_dispatch::plugin,
    ));
}

#[derive(Default, Resource)]
pub struct Score(pub i32);
