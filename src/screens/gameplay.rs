//! The screen state for the main gameplay.

use super::*;
use crate::{
    game::{input_dispatch::*, scene},
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
        .add_observer(add_new_modal)
        .add_observer(pop_modal)
        .add_observer(clear_modals);
}

#[derive(Component)]
pub struct DevUi;
#[derive(Component)]
pub struct PauseLabel;
#[derive(Component)]
pub struct MuteLabel;

fn spawn_gameplay_ui(mut cmds: Commands, settings: Res<Settings>) {
    cmds.spawn((
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
    mut cmds: Commands,
    settings: Res<Settings>,
    sources: ResMut<AudioSources>,
    action: Query<&ActionState<Action>>,
) -> Result {
    let state = action.single()?;
    if state.just_pressed(&Action::Toot) {
        cmds.spawn(sfx(sources.btn_press.clone(), settings.sfx()));
    }

    Ok(())
}

#[derive(Component)]
pub struct MenuModal;
#[derive(Component)]
pub struct SettingsModal;

fn click_to_menu(_: Trigger<Pointer<Click>>, mut cmds: Commands) {
    cmds.trigger(OnGoTo(Screen::Title));
    cmds.trigger(OnPauseToggle);
}
fn click_pop_modal(_: Trigger<Pointer<Click>>, mut cmds: Commands) {
    cmds.trigger(OnPopModal);
}
fn click_spawn_settings(_: Trigger<Pointer<Click>>, mut cmds: Commands) {
    cmds.trigger(OnNewModal(Modal::Settings));
}

fn trigger_menu_toggle_on_esc(
    _: Trigger<OnBack>,
    mut cmds: Commands,
    screen: Res<State<Screen>>,
    settings: ResMut<Settings>,
) {
    if *screen.get() != Screen::Gameplay {
        return;
    }
    if settings.modals.is_empty() {
        cmds.trigger(OnNewModal(Modal::Main));
    } else {
        cmds.trigger(OnPopModal);
    }
}

fn add_new_modal(
    trig: Trigger<OnNewModal>,
    mut cmds: Commands,
    screen: Res<State<Screen>>,
    mut settings: ResMut<Settings>,
) {
    if *screen.get() != Screen::Gameplay {
        return;
    }

    if settings.modals.is_empty() {
        cmds.trigger(OnPauseToggle);
        cmds.trigger(OnCamCursorToggle);
    }

    // despawn all previous modals
    cmds.trigger(OnClearModals);
    let OnNewModal(modal) = trig.event();
    match modal {
        Modal::Main => cmds.spawn(menu_modal()),
        Modal::Settings => cmds.spawn(settings_modal()),
    };

    settings.modals.push(modal.clone());
}

fn pop_modal(
    _: Trigger<OnPopModal>,
    mut cmds: Commands,
    screen: Res<State<Screen>>,
    mut settings: ResMut<Settings>,
    menu_marker: Query<Entity, With<MenuModal>>,
    settings_marker: Query<Entity, With<SettingsModal>>,
) {
    if Screen::Gameplay != *screen.get() {
        return;
    }

    // just a precaution
    assert!(!settings.modals.is_empty());

    let popped = settings.modals.pop().expect("popped modal with empty list");
    match popped {
        Modal::Main => {
            if let Ok(menu) = menu_marker.single() {
                cmds.entity(menu).despawn();
            }
        }
        Modal::Settings => {
            if let Ok(menu) = settings_marker.single() {
                cmds.entity(menu).despawn();
            }
        }
    }

    // respawn next in the modal stack
    if let Some(modal) = settings.modals.last() {
        match modal {
            Modal::Main => cmds.spawn(menu_modal()),
            Modal::Settings => cmds.spawn(settings_modal()),
        };
    }

    if settings.modals.is_empty() {
        cmds.trigger(OnPauseToggle);
        cmds.trigger(OnCamCursorToggle);
    }
}

fn clear_modals(
    _: Trigger<OnClearModals>,
    mut cmds: Commands,
    settings: ResMut<Settings>,
    menu_marker: Query<Entity, With<MenuModal>>,
    settings_marker: Query<Entity, With<SettingsModal>>,
) {
    for m in &settings.modals {
        match m {
            Modal::Main => {
                if let Ok(modal) = menu_marker.single() {
                    cmds.entity(modal).despawn();
                }
            }
            Modal::Settings => {
                if let Ok(modal) = settings_marker.single() {
                    cmds.entity(modal).despawn();
                }
            }
        }
    }
}

fn settings_modal() -> impl Bundle {
    (
        StateScoped(Screen::Gameplay),
        SettingsModal,
        BackgroundColor(TRANSLUCENT),
        settings::ui(),
    )
}

fn menu_modal() -> impl Bundle {
    let opts = Opts::new("Settings")
        .width(Vw(15.0))
        .padding(UiRect::axes(Vw(2.0), Vw(0.5)));
    (
        StateScoped(Screen::Gameplay),
        MenuModal,
        ui_root("In game menu"),
        children![(
            BorderColor(WHITEISH),
            BackgroundColor(TRANSLUCENT),
            Node {
                border: UiRect::all(Px(2.0)),
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
                    children![btn_small(Opts::new("x").width(Vw(5.0)), click_pop_modal)]
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
                        btn(opts.clone(), click_spawn_settings),
                        btn(opts.text("Main Menu"), click_to_menu)
                    ]
                )
            ]
        )],
    )
}
