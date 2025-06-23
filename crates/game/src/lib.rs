#![feature(duration_millis_float)]

use asset_loading::*;
use audio::*;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;
use bevy_tnua::prelude::*;
use models::*;
use scene::*;

pub mod camera;
#[cfg(feature = "dev_native")]
pub mod dev_tools;
pub mod player;
pub mod sound;

pub use camera::*;

pub fn plugin(app: &mut App) {
    app.insert_resource(Score(0));
    app.add_plugins((
        models::plugin,
        camera::plugin,
        scene::plugin,
        player::plugin,
        sound::plugin,
        #[cfg(feature = "dev_native")]
        dev_tools::plugin,
    ));
}

#[derive(Default, Resource)]
pub struct Score(pub i32);
