//! The settings screen accessible from the title screen.
//! We can add all manner of settings and accessibility options here.
//! For 3D, we'd also place the camera sensitivity and FOV here.

use super::*;
use bevy::{audio::Volume, ui::Val::*};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<GeneralVolumeLabel>();
    app.add_systems(OnEnter(Screen::Settings), spawn_settings_screen)
        .add_systems(
            Update,
            update_volume_label.run_if(in_state(Screen::Settings)),
        );
}

fn spawn_settings_screen(mut commands: Commands) {
    commands.spawn((StateScoped(Screen::Settings), ui()));
}

pub fn ui() -> impl Bundle {
    (
        ui_root("Settings Screen"),
        children![
            BackgroundColor(TRANSLUCENT),
            header("Settings"),
            core_grid(),
            // keybindings(),
            (
                Node {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::SpaceEvenly,
                    width: Percent(50.0),
                    ..default()
                },
                children![btn("Save", save_settings), btn("Back", toggle_settings),]
            )
        ],
    )
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
        cmds.trigger(OnSettingsToggle);
    }
}

pub fn save_settings(_: Trigger<OnPress>) {
    info!("TODO: serialize and save settings");
}

// TODO: implement keybinding
// good example with serializeable keybindings:
// <https://github.com/projectharmonia/bevy_enhanced_input/blob/master/examples/keybinding_menu.rs>
// fn keybindings() -> impl Bundle {}

fn core_grid() -> impl Bundle {
    (
        Name::new("Settings Grid"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children![
            (
                Node {
                    justify_self: JustifySelf::End,
                    ..default()
                },
                label("General Audio"),
            ),
            volume_widget(),
        ],
    )
}

fn volume_widget() -> impl Bundle {
    (
        Node {
            justify_self: JustifySelf::Start,
            ..Default::default()
        },
        children![
            btn_small("-", lower_general),
            (
                Node {
                    padding: UiRect::horizontal(Px(10.0)),
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                children![(label(""), GeneralVolumeLabel)],
            ),
            btn_small("+", raise_general),
        ],
    )
}

const MIN_VOLUME: f32 = 0.0;
const MAX_VOLUME: f32 = 3.0;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GeneralVolumeLabel;

fn lower_general(
    _: Trigger<Pointer<Click>>,
    mut settings: ResMut<Settings>,
    mut global_volume: ResMut<GlobalVolume>,
) {
    let new_volume = (settings.sound.general - 0.1).max(MIN_VOLUME);
    settings.sound.general = new_volume;
    global_volume.volume = Volume::Linear(settings.sound.general);
    // TODO: update all playing music because updating global volume does not affect existing Playback
}

fn raise_general(
    _: Trigger<Pointer<Click>>,
    mut settings: ResMut<Settings>,
    mut global_volume: ResMut<GlobalVolume>,
) {
    let new_volume = (settings.sound.general + 0.1).min(MAX_VOLUME);
    settings.sound.general = new_volume;
    global_volume.volume = Volume::Linear(settings.sound.general);
    // TODO: update all playing music because updating global volume does not affect existing Playback
}

fn update_volume_label(
    mut label: Single<&mut Text, With<GeneralVolumeLabel>>,
    settings: Res<Settings>,
) {
    let percent = (settings.sound.general * 100.0).round();
    let text = format!("{percent}%");
    label.0 = text;
}
