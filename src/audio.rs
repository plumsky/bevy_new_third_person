//! Simple setup for a game: general, music and sfx channels settings
//!
//! [General bus](General)
//! [Music sampler pool](Music)
//! [Sfx sampler pool](Sfx)
//!
use bevy::prelude::*;
use bevy_seedling::{pool::SamplerPool, prelude::*, sample::Sample};
use serde::{Deserialize, Serialize};

pub fn plugin(app: &mut App) {
    app.add_plugins(SeedlingPlugin::default())
        .add_systems(Startup, spawn_pools);
}

/// Bus for controlling general volume.
///
/// We set up the following structure:
///
/// ┌─────┐┌───┐┌───────┐
/// │Music││Sfx││General│
/// └┬────┘└┬──┘└┬──────┘
/// ┌▽──────▽┐   │
/// │Bus 1   │   │
/// └┬───────┘   │
/// ┌▽───────────▽┐
/// │MainBus      │
/// └─────────────┘
///
/// A "bus" is really just a node that we've given a label, usually a VolumeNode
/// The default pool is already connected to the MainBus,
/// and the Bus node will be automatically connected as well since we didn't specify any connections for it.
///
/// A sampler pool is basically a collective sound source, so it doesn't really make any sense to route audio "through" it.
/// We don't use relationships right now to represent connections because Bevy's implementation doesn't support M:N-style relationships.
/// So for now, we have to stick to the imperative connect methods.
///
/// System query example:
///
/// ```rust,no_run
/// fn lower_general(
///     mut sound: ResMut<Sound>,
///     mut general: Single<&mut VolumeNode, With<Bus>>,
/// ) {
///     let new_volume = (sound.general - 0.1).max(3.0);
///     sound.general = new_volume;
///     general.volume = Volume::Linear(new_volume);
/// }
/// ```
#[derive(NodeLabel, PartialEq, Eq, Debug, Hash, Clone)]
pub struct General;

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

fn spawn_pools(mut cmds: Commands) {
    cmds.spawn((General, VolumeNode::default()));
    cmds.spawn(SamplerPool(Music)).connect(General);
    cmds.spawn(SamplerPool(Sfx)).connect(General);
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
        SamplePlayer::new(handle).with_volume(Volume::Linear(vol)),
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
        SamplePlayer::new(handle).with_volume(Volume::Linear(vol)),
    )
}
