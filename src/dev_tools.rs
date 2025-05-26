//! Development tools for the game. This plugin is only enabled in dev builds.

use crate::{
    game::triggers::*,
    prelude::*,
    screens::gameplay::{MuteLabel, PauseLabel},
};
use bevy::{
    dev_tools::states::log_transitions,
    prelude::*,
    ui::{Display as NodeDisplay, UiDebugOptions},
};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(toggle_mute)
        .add_observer(toggle_pause)
        .add_observer(toggle_diagnostics)
        .add_systems(Update, log_transitions::<Screen>);

    #[cfg(feature = "dev_native")]
    app.add_observer(toggle_debug_ui);
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
    mut music: Query<&mut AudioSink, (With<Music>, Without<SoundEffect>)>,
    mut sfx: Query<&mut AudioSink, (With<SoundEffect>, Without<Music>)>,
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
    for s in music.iter_mut().chain(sfx.iter_mut()) {
        s.toggle_playback();
    }
    settings.paused = !settings.paused;
}

fn toggle_mute(
    _: Trigger<OnMuteToggle>,
    mut settings: ResMut<Settings>,
    mut label: Query<(&mut BackgroundColor, &mut TextColor), With<MuteLabel>>,
    mut music: Query<&mut AudioSink, (With<Music>, Without<SoundEffect>)>,
    mut sfx: Query<&mut AudioSink, (With<SoundEffect>, Without<Music>)>,
) {
    if let Ok((mut bg, mut color)) = label.single_mut() {
        if settings.muted {
            *color = TextColor(WHITEISH);
            *bg = BackgroundColor(TRANSPARENT);
        } else {
            *color = TextColor(GRAY);
            *bg = BackgroundColor(WHITEISH);
        }
        // TODO: use seedling when it's migrated to 0.16
        // s.set_volume(Volume::Linear(settings.sound.general * settings.sound.sfx));
        for mut s in music.iter_mut().chain(sfx.iter_mut()) {
            s.toggle_mute();
        }
    }
    settings.muted = !settings.muted;
}

#[cfg(feature = "dev_native")]
fn toggle_debug_ui(_: Trigger<OnDebugUiToggle>, mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
