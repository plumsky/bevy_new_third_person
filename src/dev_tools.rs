//! Development tools for the game. This plugin is only enabled in dev builds.

use crate::{
    game::input_dispatch::*,
    prelude::*,
    screens::gameplay::{MuteLabel, PauseLabel},
};
#[cfg(any(feature = "dev", feature = "dev_native"))]
use bevy::{
    dev_tools::states::log_transitions,
    prelude::*,
    ui::{Display as NodeDisplay, UiDebugOptions},
};
#[cfg(all(not(feature = "dev"), not(feature = "dev_native")))]
use bevy::{prelude::*, ui::Display as NodeDisplay};
use bevy_seedling::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(toggle_mute)
        .add_observer(toggle_pause)
        .add_observer(toggle_diagnostics);

    #[cfg(any(feature = "dev", feature = "dev_native"))]
    {
        app.add_systems(Update, log_transitions::<Screen>);
        app.add_observer(toggle_debug_ui);
    }
}

fn toggle_diagnostics(
    _: Trigger<OnDiagnosticsToggle>,
    mut settings: ResMut<Settings>,
    mut perf_ui: Query<&mut Node, With<PerfUiMarker>>,
) {
    if let Ok(mut perf_ui) = perf_ui.single_mut() {
        if perf_ui.display == NodeDisplay::None {
            perf_ui.display = NodeDisplay::Flex;
        } else {
            perf_ui.display = NodeDisplay::None;
        }
        settings.diagnostics = !settings.diagnostics;
    }
}

fn toggle_pause(
    _: Trigger<OnPauseToggle>,
    mut settings: ResMut<Settings>,
    mut time: ResMut<Time<Virtual>>,
    mut label: Query<(&mut BackgroundColor, &mut TextColor), With<PauseLabel>>,
    mut music: Query<&mut PlaybackParams, (With<Music>, Without<Sfx>)>,
    mut sfx: Query<&mut PlaybackParams, (With<Sfx>, Without<Music>)>,
) {
    if let Ok((mut bg, mut color)) = label.single_mut() {
        if time.is_paused() || settings.paused {
            time.unpause();
            *color = TextColor(WHITEISH);
            *bg = BackgroundColor(TRANSPARENT);
        } else {
            time.pause();
            *color = TextColor(GRAY);
            *bg = BackgroundColor(WHITEISH);
        }
    }

    // TODO: use seedling when it's migrated to 0.16
    settings.paused = !settings.paused;
    for mut s in music.iter_mut().chain(sfx.iter_mut()) {
        if settings.paused {
            s.pause();
        } else {
            s.play();
        }
    }
}

fn toggle_mute(
    _: Trigger<OnMuteToggle>,
    mut settings: ResMut<Settings>,
    mut label: Query<(&mut BackgroundColor, &mut TextColor), With<MuteLabel>>,
    mut music: Single<&mut VolumeNode, (With<SamplerPool<Music>>, Without<SamplerPool<Sfx>>)>,
    mut sfx: Single<&mut VolumeNode, (With<SamplerPool<Sfx>>, Without<SamplerPool<Music>>)>,
) {
    if let Ok((mut bg, mut color)) = label.single_mut() {
        if settings.muted {
            music.volume = Volume::Linear(settings.sound.general * settings.sound.music);
            sfx.volume = Volume::Linear(settings.sound.general * settings.sound.sfx);
            *color = TextColor(WHITEISH);
            *bg = BackgroundColor(TRANSPARENT);
        } else {
            music.volume = Volume::SILENT;
            sfx.volume = Volume::SILENT;
            *color = TextColor(GRAY);
            *bg = BackgroundColor(WHITEISH);
        }
    }
    settings.muted = !settings.muted;
}

#[cfg(any(feature = "dev", feature = "dev_native"))]
fn toggle_debug_ui(_: Trigger<OnDebugUiToggle>, mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
