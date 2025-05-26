//! The screen state for the main gameplay.

use super::*;
use crate::{
    game::{scene, triggers::*},
    screens::settings,
};
use bevy::ui::Val::*;
use leafwing_input_manager::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(crate::game::plugin)
        .add_systems(
            OnEnter(Screen::Gameplay),
            spawn_gameplay_ui.after(scene::setup),
        )
        .add_systems(
            Update,
            toot.run_if(resource_exists::<AudioSources>)
                .run_if(in_state(Screen::Gameplay)),
        )
        .add_observer(trigger_menu_toggle_on_esc)
        .add_observer(toggle_settings)
        .add_observer(toggle_menu);
}

#[derive(Component)]
pub struct DevUi;
#[derive(Component)]
pub struct PauseLabel;
#[derive(Component)]
pub struct MuteLabel;

fn spawn_gameplay_ui(mut commands: Commands, settings: Res<Settings>) {
    commands.spawn((
        StateScoped(Screen::Gameplay),
        DevUi,
        Node {
            flex_direction: FlexDirection::Row,
            ..Default::default()
        },
        children![
            // Demo keys
            (
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Start,
                    ..Default::default()
                },
                children![
                    (label("P - pause"), PauseLabel),
                    (label("M - mute"), MuteLabel),
                    label("F - diagnostics"),
                    label("V - incr fov"),
                    label("~ - toggle UI debug mode"),
                    (
                        label(format!("O - toggle sun cycle: {:?}", settings.sun_cycle)),
                        SunCycleLabel
                    ),
                    TextLayout::new_with_justify(JustifyText::Left)
                ]
            ),
        ],
    ));
}

fn toot(
    mut commands: Commands,
    settings: Res<Settings>,
    sources: ResMut<AudioSources>,
    action: Query<&ActionState<Action>>,
) -> Result {
    let state = action.single()?;
    if state.just_pressed(&Action::Toot) {
        let vol = settings.sound.general * settings.sound.sfx;
        commands.spawn(sfx(sources.btn_press.clone(), vol));
    }

    Ok(())
}

#[derive(Component)]
pub struct MenuModal;
#[derive(Component)]
pub struct SettingsModal;

// TODO: maybe we don't need 3 separate functions to manage menu, but for now this is the way

fn click_toggle_menu(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(OnMenuToggle);
}
fn click_toggle_settings(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.trigger(OnSettingsToggle);
    commands.trigger(OnMenuToggle);
}

fn trigger_menu_toggle_on_esc(
    _: Trigger<OnBack>,
    mut commands: Commands,
    screen: Res<State<Screen>>,
    settings: ResMut<Settings>,
) {
    if *screen.get() != Screen::Gameplay {
        return;
    }
    if settings.settings_modal {
        commands.trigger(OnSettingsToggle);
    }
    commands.trigger(OnMenuToggle);
}

fn toggle_menu(
    _: Trigger<OnMenuToggle>,
    mut commands: Commands,
    screen: Res<State<Screen>>,
    mut settings: ResMut<Settings>,
    menu: Query<Entity, With<MenuModal>>,
) {
    if *screen.get() != Screen::Gameplay {
        return;
    }

    if !settings.settings_modal {
        commands.trigger(OnPauseToggle);
        commands.trigger(OnCamCursorToggle);
    }
    settings.menu_modal = !settings.menu_modal;

    if settings.menu_modal {
        commands.spawn(menu_modal());
    } else if let Ok(menu) = menu.single() {
        commands.entity(menu).despawn();
    }
}

fn toggle_settings(
    _: Trigger<OnSettingsToggle>,
    mut commands: Commands,
    screen: Res<State<Screen>>,
    mut settings: ResMut<Settings>,
    settings_marker: Query<Entity, With<SettingsModal>>,
) {
    if Screen::Gameplay != *screen.get() {
        return;
    }

    if !settings.menu_modal {
        commands.trigger(OnPauseToggle);
        commands.trigger(OnCamCursorToggle);
    }
    settings.settings_modal = !settings.settings_modal;

    if settings.settings_modal {
        commands.spawn(settings_modal());
    } else if let Ok(settings) = settings_marker.single() {
        commands.entity(settings).despawn();
    }
}

fn settings_modal() -> impl Bundle {
    (StateScoped(Screen::Gameplay), SettingsModal, settings::ui())
}

fn menu_modal() -> impl Bundle {
    let opts = Opts::new("Settings")
        .width(Vw(15.0))
        .padding(UiRect::axes(Vw(2.0), Vw(1.0)));
    (
        StateScoped(Screen::Gameplay),
        MenuModal,
        ui_root("In game menu"),
        children![(
            BackgroundColor(TRANSLUCENT),
            Node {
                padding: UiRect::all(Vw(10.0)),
                ..default()
            },
            children![
                (
                    Node {
                        position_type: PositionType::Absolute,
                        top: Px(0.0),
                        right: Px(0.0),
                        ..Default::default()
                    },
                    children![btn_small(Opts::new("x").width(Vw(5.0)), click_toggle_menu)]
                ),
                (
                    Node {
                        row_gap: Percent(20.0),
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_content: AlignContent::Center,
                        ..default()
                    },
                    children![
                        btn(opts.clone(), click_toggle_settings),
                        btn(opts.text("Main Menu"), to::title)
                    ]
                )
            ]
        )],
    )
}
