use super::*;

pub fn plugin(app: &mut App) {
    app.add_event::<OnBack>()
        .add_event::<OnGoTo>()
        .add_event::<OnInputCtxSwitch>()
        .add_event::<OnSwitchTab>()
        .add_event::<OnNewModal>()
        .add_event::<OnPopModal>()
        .add_event::<OnClearModals>()
        .add_event::<OnPauseToggle>()
        .add_event::<OnMuteToggle>()
        .add_event::<OnFovIncrement>()
        .add_event::<OnCamCursorToggle>()
        .add_event::<OnDebugUiToggle>()
        .add_event::<OnDiagnosticsToggle>()
        .add_observer(pause)
        .add_observer(mute)
        .add_observer(back);
}

#[derive(Event)]
pub struct OnGoTo(pub Screen);
#[derive(Event)]
pub struct OnBack(pub Screen);
#[derive(Event)]
pub struct OnInputCtxSwitch(pub Context);
#[derive(Event, Deref)]
pub struct OnSwitchTab(pub UiTab);
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

fn back(
    _: Trigger<Started<Back>>,
    screen: Res<State<Screen>>,
    settings: Res<Settings>,
    mut commands: Commands,
) {
    match screen.get() {
        Screen::Splash | Screen::Title | Screen::Loading => {}
        _ => {
            let last = settings.last_screen.clone();
            commands.trigger(OnBack(last));
        }
    }
}

fn pause(_: Trigger<Started<Pause>>, mut commands: Commands) {
    info!("on PAUSE");
    commands.trigger(OnPauseToggle);
}
fn mute(_: Trigger<Started<Mute>>, mut commands: Commands) {
    info!("on MUTE");
    commands.trigger(OnMuteToggle);
}
