use asset_loading::AudioSources;
use audio::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use models::*;
use ui::*;

pub mod camera;
pub mod dev_tools;
pub mod scene;
pub mod settings;
pub mod sound;

pub fn plugin(app: &mut App) {
    app.insert_resource(Score(0));
    app.add_plugins((
        models::plugin,
        settings::plugin,
        camera::plugin,
        scene::plugin,
        sound::plugin,
        dev_tools::plugin,
    ));
}

#[derive(Default, Resource)]
pub struct Score(pub i32);
