use super::*;
use bevy::ui::Display as NodeDisplay;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            update_general_volume_label,
            update_music_volume_label,
            update_sfx_volume_label,
            update_fov_label,
            update_tab_content.run_if(resource_changed::<ActiveTab>),
        ),
    );
}

// ============================ CONTROL KNOBS OBSERVERS ============================

pub fn save_settings(_: Trigger<OnPress>, settings: Res<Settings>) {
    match settings.save() {
        Ok(()) => info!("writing settings to '{SETTINGS_PATH}'"),
        Err(e) => error!("unable to write settings to '{SETTINGS_PATH}': {e}"),
    }
}

// TAB CHANGING
fn update_tab_content(
    settings: Res<Settings>,
    active_tab: Res<ActiveTab>,
    tab_bar: Query<&Children, With<TabBar>>,
    mut tab_content: Query<(Entity, &Children), With<TabContent>>,
    mut buttons: Query<(&UiTab, &mut Node)>,
    mut commands: Commands,
) -> Result {
    for children in &tab_bar {
        for &child in children {
            if let Ok((tab, mut node)) = buttons.get_mut(child) {
                if *tab == active_tab.0 {
                    node.border.bottom = Px(0.0);

                    let (e, content) = tab_content.single_mut()?;
                    for child in content.iter() {
                        commands.entity(child).despawn();
                    }
                    match tab {
                        UiTab::Audio => {
                            commands.spawn(audio_grid()).insert(ChildOf(e));
                        }
                        UiTab::Video => {
                            commands
                                .spawn(video_grid(&settings.sun_cycle))
                                .insert(ChildOf(e));
                        }
                        UiTab::Keybindings => {
                            commands
                                .spawn(keybind_editor(&settings.keybind))
                                .insert(ChildOf(e));
                        }
                    }
                } else {
                    node.border.bottom = Px(10.0);
                }
            }
        }
    }

    Ok(())
}

// ============================ +/- BUTTON HOOKS ============================

fn lower_fov(
    _: Trigger<Pointer<Click>>,
    cfg: Res<Config>,
    mut settings: ResMut<Settings>,
    mut world_model_projection: Single<&mut Projection>,
) {
    let Projection::Perspective(perspective) = world_model_projection.as_mut() else {
        return;
    };
    let new_fov = (settings.fov - cfg.settings.step.to_degrees()).max(cfg.settings.min_fov);
    perspective.fov = new_fov.to_radians();
    settings.fov = perspective.fov.to_degrees();
}

fn raise_fov(
    _: Trigger<Pointer<Click>>,
    cfg: Res<Config>,
    mut settings: ResMut<Settings>,
    mut world_model_projection: Single<&mut Projection>,
) {
    let Projection::Perspective(perspective) = world_model_projection.as_mut() else {
        return;
    };
    let new_fov = (settings.fov + cfg.settings.step.to_degrees()).min(cfg.settings.max_fov);
    perspective.fov = new_fov.to_radians();
    settings.fov = perspective.fov.to_degrees();
}

fn update_fov_label(settings: Res<Settings>, mut label: Single<&mut Text, With<FovLabel>>) {
    let fov = settings.fov.round();
    let mut text = format!("{fov}");
    if text.len() < 4 {
        text.push(' ');
    }
    label.0 = text;
}

// GENERAL
fn lower_general(
    _: Trigger<Pointer<Click>>,
    cfg: ResMut<Config>,
    mut settings: ResMut<Settings>,
    mut general: Single<&mut VolumeNode, With<MainBus>>,
) {
    let new_volume = (settings.sound.general - cfg.settings.step).max(cfg.settings.min_volume);
    settings.sound.general = new_volume;
    general.volume = Volume::Linear(new_volume);
}

fn raise_general(
    _: Trigger<Pointer<Click>>,
    cfg: ResMut<Config>,
    mut settings: ResMut<Settings>,
    mut general: Single<&mut VolumeNode, With<MainBus>>,
) {
    let new_volume = (settings.sound.general + cfg.settings.step).min(cfg.settings.max_volume);
    settings.sound.general = new_volume;
    general.volume = Volume::Linear(new_volume);
}

fn update_general_volume_label(
    settings: Res<Settings>,
    mut label: Single<&mut Text, With<GeneralVolumeLabel>>,
) {
    let percent = (settings.sound.general * 100.0).round();
    let mut text = format!("{percent}%");
    if text.len() < 4 {
        text.push(' ');
    }
    label.0 = text;
}

// MUSIC
fn lower_music(
    _: Trigger<Pointer<Click>>,
    cfg: ResMut<Config>,
    mut settings: ResMut<Settings>,
    mut music: Single<&mut VolumeNode, (With<SamplerPool<Music>>, Without<SamplerPool<Sfx>>)>,
) {
    let new_volume = (settings.sound.music - cfg.settings.step).max(cfg.settings.min_volume);
    settings.sound.music = new_volume;
    music.volume = settings.music();
}

fn raise_music(
    _: Trigger<Pointer<Click>>,
    cfg: ResMut<Config>,
    mut settings: ResMut<Settings>,
    mut music: Single<&mut VolumeNode, (With<SamplerPool<Music>>, Without<SamplerPool<Sfx>>)>,
) {
    let new_volume = (settings.sound.music + cfg.settings.step).min(cfg.settings.max_volume);
    settings.sound.music = new_volume;
    music.volume = settings.music();
}

fn update_music_volume_label(
    settings: Res<Settings>,
    mut label: Single<&mut Text, With<MusicVolumeLabel>>,
) {
    let percent = (settings.sound.music * 100.0).round();
    let mut text = format!("{percent}%");
    if text.len() < 4 {
        text.push(' ');
    }
    label.0 = text;
}

// SFX
fn lower_sfx(
    _: Trigger<Pointer<Click>>,
    cfg: ResMut<Config>,
    mut settings: ResMut<Settings>,
    mut sfx: Single<&mut VolumeNode, (With<SamplerPool<Sfx>>, Without<SamplerPool<Music>>)>,
) {
    let new_volume = (settings.sound.sfx - cfg.settings.step).max(cfg.settings.min_volume);
    settings.sound.sfx = new_volume;
    sfx.volume = settings.sfx();
}

fn raise_sfx(
    _: Trigger<Pointer<Click>>,
    cfg: ResMut<Config>,
    mut settings: ResMut<Settings>,
    mut sfx: Single<&mut VolumeNode, (With<SamplerPool<Sfx>>, Without<SamplerPool<Music>>)>,
) {
    let new_volume = (settings.sound.sfx + cfg.settings.step).min(cfg.settings.max_volume);
    settings.sound.sfx = new_volume;
    sfx.volume = settings.sfx();
}

fn update_sfx_volume_label(
    mut label: Single<&mut Text, With<SfxVolumeLabel>>,
    settings: Res<Settings>,
) {
    let percent = (settings.sound.sfx * 100.0).round();
    let mut text = format!("{percent}%");
    if text.len() < 4 {
        text.push(' ');
    }
    label.0 = text;
}

// ============================ OTHER BUTTON HOOKS ============================

fn switch_to_tab(tab: UiTab) -> impl Fn(Trigger<Pointer<Click>>, ResMut<ActiveTab>) + Clone {
    move |_: Trigger<Pointer<Click>>, mut active_tab: ResMut<ActiveTab>| {
        active_tab.0 = tab;
    }
}

fn click_toggle_diagnostics(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut settings: ResMut<Settings>,
    mut perf_ui: Query<&mut Node, With<PerfUi>>,
) {
    if let Ok(mut perf_ui) = perf_ui.single_mut() {
        if perf_ui.display == NodeDisplay::None {
            perf_ui.display = NodeDisplay::Flex;
        } else {
            perf_ui.display = NodeDisplay::None;
        }
        settings.diagnostics = !settings.diagnostics;
        commands.trigger(OnDiagnosticsToggle);
    }
}

#[cfg(feature = "dev_native")]
fn clock_toggle_debug_ui(
    _: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut settings: ResMut<Settings>,
) {
    settings.debug_ui = !settings.debug_ui;
    commands.trigger(OnDebugUiToggle);
}

fn click_toggle_sun_cycle(
    _: Trigger<Pointer<Click>>,
    mut settings: ResMut<Settings>,
    mut label: Query<&mut Text, With<SunCycleLabel>>,
) {
    match settings.sun_cycle {
        SunCycle::Nimbus => {
            settings.sun_cycle = SunCycle::DayNight;
        }
        SunCycle::DayNight => {
            settings.sun_cycle = SunCycle::Nimbus;
        }
    }

    if let Ok(mut text) = label.single_mut() {
        *text = Text(settings.sun_cycle.as_str().into());
    }
}

fn click_toggle_settings(
    _: Trigger<OnPress>,
    mut cmds: Commands,
    screen: Res<State<Screen>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if *screen.get() == Screen::Settings {
        next_screen.set(Screen::Title);
    } else {
        info!("Pop modal from prefab");
        cmds.trigger(OnPopModal);
    }
}

// ============================ UI ============================

pub fn settings_ui() -> impl Bundle {
    (
        ui_root("Settings Screen"),
        children![
            BackgroundColor(TRANSLUCENT),
            header("Settings"),
            tab_bar(),
            (TabContent, Node::default(), children![audio_grid()]),
            // keybindings(),
            navigation()
        ],
    )
}

fn tab_bar() -> impl Bundle {
    let opts = Opts::default().border_radius(Px(0.0));
    (
        Node {
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            width: Percent(100.0),
            ..default()
        },
        TabBar,
        children![
            (
                btn(opts.clone().text("Audio"), switch_to_tab(UiTab::Audio)),
                UiTab::Audio
            ),
            (
                btn(opts.clone().text("Video"), switch_to_tab(UiTab::Video)),
                UiTab::Video
            ),
            (
                btn(opts.text("Keybindings"), switch_to_tab(UiTab::Keybindings)),
                UiTab::Keybindings
            ),
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
        children![
            btn("Save", save_settings),
            btn("Back", click_toggle_settings),
        ],
    )
}

fn video_grid(cycle: &SunCycle) -> impl Bundle {
    (
        Name::new("Settings Video Grid"),
        Node {
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            display: Display::Grid,
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children![
            label("Sun cycle"),
            (btn(cycle.as_str(), click_toggle_sun_cycle), SunCycleLabel),
            label("FOV"),
            fov(),
            // TODO: do checkboxes
            label("diagnostics"),
            btn("toggle", click_toggle_diagnostics),
            #[cfg(feature = "dev_native")]
            label("debug ui"),
            #[cfg(feature = "dev_native")]
            btn("toggle", clock_toggle_debug_ui)
        ],
    )
}
fn audio_grid() -> impl Bundle {
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
            label("general"),
            general_volume(),
            label("music"),
            music_volume(),
            label("sfx"),
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

// TODO: fov slider
fn fov() -> impl Bundle {
    (
        knobs_container(),
        children![
            btn_small("-", lower_fov),
            knob_label(FovLabel),
            btn_small("+", raise_fov),
        ],
    )
}

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
