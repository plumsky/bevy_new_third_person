use bevy::prelude::*;
use bevy_seedling::{pool::SamplerPool, prelude::*, sample::Sample};
use serde::{Deserialize, Serialize};

pub fn plugin(app: &mut App) {
    app.add_plugins(SeedlingPlugin::default())
        .add_systems(Startup, spawn_pools);
}

fn spawn_pools(mut cmds: Commands) {
    cmds.spawn(SamplerPool(Music));
    cmds.spawn(SamplerPool(Sfx));
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize, Reflect)]
pub struct Sound {
    pub general: f32,
    pub music: f32,
    pub sfx: f32,
}

impl Default for Sound {
    fn default() -> Self {
        Self {
            general: 1.0,
            music: 0.5,
            sfx: 0.5,
        }
    }
}

/// An organizational marker component that should be added to a spawned [`SamplePlayer`] if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(PoolLabel, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
#[reflect(Component)]
pub struct Music;

/// A music audio instance.
pub fn music(handle: Handle<Sample>, vol: f32) -> impl Bundle {
    (
        Music,
        SamplePlayer::new(handle),
        PlaybackSettings {
            volume: Volume::Linear(vol),
            ..PlaybackSettings::LOOP
        },
    )
}

/// An organizational marker component that should be added to a spawned [`SamplePlayer`] if it's in the
/// general "sound effect" category (e.g. footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.

#[derive(PoolLabel, Debug, Clone, PartialEq, Eq, Hash, Default, Reflect)]
#[reflect(Component)]
pub struct Sfx;

/// A sound effect audio instance.
pub fn sfx(handle: Handle<Sample>, vol: f32) -> impl Bundle {
    (
        Sfx,
        SamplePlayer::new(handle),
        PlaybackSettings {
            volume: Volume::Linear(vol),
            ..PlaybackSettings::REMOVE
        },
    )
}
