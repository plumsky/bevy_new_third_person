//! The settings screen accessible from the title screen.
//! We can add all manner of settings and accessibility options here.
//! For 3D, we'd also place the camera sensitivity and FOV here.
//!
use super::*;
use bevy::ui::Val::*;
use bevy_seedling::prelude::*;

const MIN_VOLUME: f32 = 0.0;
const MAX_VOLUME: f32 = 3.0;
const STEP: f32 = 0.1;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<GeneralVolumeLabel>();
    app.add_systems(OnEnter(Screen::Settings), spawn_settings_screen)
        .add_systems(
            Update,
            (
                update_general_volume_label,
                update_music_volume_label,
                update_sfx_volume_label,
            ),
        );
}

fn spawn_settings_screen(mut commands: Commands) {
    commands.spawn((StateScoped(Screen::Settings), ui()));
}

pub fn toggle_settings(
    _: Trigger<OnPress>,
    mut cmds: Commands,
    screen: Res<State<Screen>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if *screen.get() == Screen::Settings {
        next_screen.set(Screen::Title);
    } else {
        cmds.trigger(OnPopModal);
    }
}

pub fn save_settings(_: Trigger<OnPress>) {
    info!("TODO: serialize and save settings");
}

// TODO: implement keybinding
// good example with serializeable keybindings:
// <https://github.com/projectharmonia/bevy_enhanced_input/blob/master/examples/keybinding_menu.rs>
// fn keybindings() -> impl Bundle {}

// ============================ CONTROL KNOBS OBSERVERS ============================

// GENERAL
fn lower_general(
    _: Trigger<Pointer<Click>>,
    mut settings: ResMut<Settings>,
    mut general: Single<&mut VolumeNode, With<General>>,
) {
    let new_volume = (settings.sound.general - STEP).max(MIN_VOLUME);
    settings.sound.general = new_volume;
    general.volume = Volume::Linear(new_volume);
}

fn raise_general(
    _: Trigger<Pointer<Click>>,
    mut settings: ResMut<Settings>,
    mut general: Single<&mut VolumeNode, With<General>>,
) {
    let new_volume = (settings.sound.general + STEP).min(MAX_VOLUME);
    settings.sound.general = new_volume;
    general.volume = Volume::Linear(new_volume);
}

fn update_general_volume_label(
    mut label: Single<&mut Text, With<GeneralVolumeLabel>>,
    settings: Res<Settings>,
) {
    let percent = (settings.sound.general * 100.0).round();
    let mut text = format!("{percent}%");
    if text.len() < 3 {
        text.push(' ');
    }
    label.0 = text;
}

// MUSIC
fn lower_music(
    _: Trigger<Pointer<Click>>,
    mut settings: ResMut<Settings>,
    mut music: Single<&mut VolumeNode, (With<SamplerPool<Music>>, Without<SamplerPool<Sfx>>)>,
) {
    let new_volume = (settings.sound.music - STEP).max(MIN_VOLUME);
    settings.sound.music = new_volume;
    music.volume = Volume::Linear(new_volume * settings.sound.general);
}

fn raise_music(
    _: Trigger<Pointer<Click>>,
    mut settings: ResMut<Settings>,
    mut music: Single<&mut VolumeNode, (With<SamplerPool<Music>>, Without<SamplerPool<Sfx>>)>,
) {
    let new_volume = (settings.sound.music + STEP).min(MAX_VOLUME);
    settings.sound.music = new_volume;
    music.volume = Volume::Linear(new_volume * settings.sound.general);
}

fn update_music_volume_label(
    mut label: Single<&mut Text, With<MusicVolumeLabel>>,
    settings: Res<Settings>,
) {
    let percent = (settings.sound.music * 100.0).round();
    let mut text = format!("{percent}%");
    if text.len() < 3 {
        text.push(' ');
    }
    label.0 = text;
}

// SFX
fn lower_sfx(
    _: Trigger<Pointer<Click>>,
    mut settings: ResMut<Settings>,
    mut sfx: Single<&mut VolumeNode, (With<SamplerPool<Sfx>>, Without<SamplerPool<Music>>)>,
) {
    let new_volume = (settings.sound.sfx - STEP).max(MIN_VOLUME);
    settings.sound.sfx = new_volume;
    sfx.volume = Volume::Linear(new_volume * settings.sound.general);
}

fn raise_sfx(
    _: Trigger<Pointer<Click>>,
    mut settings: ResMut<Settings>,
    mut sfx: Single<&mut VolumeNode, (With<SamplerPool<Sfx>>, Without<SamplerPool<Music>>)>,
) {
    let new_volume = (settings.sound.sfx + STEP).min(MAX_VOLUME);
    settings.sound.sfx = new_volume;
    sfx.volume = Volume::Linear(new_volume * settings.sound.general);
}

fn update_sfx_volume_label(
    mut label: Single<&mut Text, With<SfxVolumeLabel>>,
    settings: Res<Settings>,
) {
    let percent = (settings.sound.sfx * 100.0).round();
    let mut text = format!("{percent}%");
    if text.len() < 3 {
        text.push(' ');
    }
    label.0 = text;
}

// ============================ UI STUFF ============================

pub fn ui() -> impl Bundle {
    (
        ui_root("Settings Screen"),
        children![
            BackgroundColor(TRANSLUCENT),
            header("Settings"),
            core_grid(),
            // keybindings(),
            navigation()
        ],
    )
}

fn navigation() -> impl Bundle {
    (
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::SpaceEvenly,
            width: Percent(50.0),
            ..default()
        },
        children![btn("Save", save_settings), btn("Back", toggle_settings),],
    )
}

fn core_grid() -> impl Bundle {
    (
        Name::new("Settings Grid"),
        Node {
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children![
            label("General"),
            general_volume(),
            label("Music"),
            music_volume(),
            label("Sfx"),
            sfx_volume(),
        ],
    )
}
fn general_volume() -> impl Bundle {
    (
        Node {
            justify_self: JustifySelf::Start,
            ..Default::default()
        },
        children![
            btn_small("-", lower_general),
            knob_label(GeneralVolumeLabel),
            btn_small("+", raise_general),
        ],
    )
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GeneralVolumeLabel;

fn music_volume() -> impl Bundle {
    (
        knobs_container(),
        children![
            btn_small("-", lower_music),
            knob_label(MusicVolumeLabel),
            btn_small("+", raise_music),
        ],
    )
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct MusicVolumeLabel;

fn sfx_volume() -> impl Bundle {
    (
        knobs_container(),
        children![
            btn_small("-", lower_sfx),
            knob_label(SfxVolumeLabel),
            btn_small("+", raise_sfx),
        ],
    )
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct SfxVolumeLabel;

fn knob_label(knob: impl Component) -> impl Bundle {
    (
        Node {
            padding: UiRect::horizontal(Px(10.0)),
            justify_content: JustifyContent::Center,
            ..Default::default()
        },
        children![(label(""), knob)],
    )
}

fn knobs_container() -> impl Bundle {
    Node {
        justify_self: JustifySelf::Start,
        align_content: AlignContent::SpaceEvenly,
        min_width: Px(100.0),
        ..Default::default()
    }
}
