use crate::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_event::<OnBack>()
        .add_event::<OnGoTo>()
        .add_event::<OnNewModal>()
        .add_event::<OnPopModal>()
        .add_event::<OnClearModals>()
        .add_event::<OnPauseToggle>()
        .add_event::<OnMuteToggle>()
        .add_event::<OnFovIncrement>()
        .add_event::<OnCamCursorToggle>()
        .add_event::<OnDebugUiToggle>()
        .add_event::<OnDiagnosticsToggle>()
        .add_systems(Update, trigger_input_dispatch);
}

#[derive(Event)]
pub struct OnGoTo(pub Screen);
#[derive(Event)]
pub struct OnBack(pub Screen);
#[derive(Event, Deref)]
pub struct OnNewModal(pub Modal);
#[derive(Event)]
pub struct OnPopModal;
#[derive(Event)]
pub struct OnClearModals;
#[derive(Event)]
pub struct OnCamCursorToggle;
#[derive(Event)]
pub struct OnFovIncrement;
#[derive(Event)]
pub struct OnPauseToggle;
#[derive(Event)]
pub struct OnMuteToggle;
#[derive(Event)]
pub struct OnDiagnosticsToggle;
#[derive(Event)]
pub struct OnDebugUiToggle;

fn trigger_input_dispatch(
    mut commands: Commands,
    screen: Res<State<Screen>>,
    settings: Res<Settings>,
    action: Query<&ActionState<Action>>,
) -> Result {
    let state = action.single()?;

    // TODO: replace with match, becaue it will grow
    if state.just_pressed(&Action::ToggleDiagnostics) {
        commands.trigger(OnDiagnosticsToggle);
    }
    if state.just_pressed(&Action::TogglePause) {
        commands.trigger(OnPauseToggle);
    }
    if state.just_pressed(&Action::ToggleMute) {
        commands.trigger(OnMuteToggle);
    }
    if state.just_pressed(&Action::ToggleUiDebug) {
        commands.trigger(OnDebugUiToggle);
    }
    if state.just_pressed(&Action::Back) {
        match screen.get() {
            Screen::Splash | Screen::Title | Screen::Loading => {}
            _ => {
                let last = settings.last_screen.clone();
                commands.trigger(OnBack(last));
            }
        }
    }
    if state.just_pressed(&Action::FovIncr) {
        commands.trigger(OnFovIncrement);
    }

    Ok(())
}
