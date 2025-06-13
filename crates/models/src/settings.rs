use super::*;
use leafwing_input_manager::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<Settings>();
    app.add_plugins(InputManagerPlugin::<Action>::default())
        .add_systems(Startup, spawn_player_input_map)
        .add_systems(
            OnEnter(Screen::Gameplay),
            inject_settings_from_cfg.run_if(resource_exists::<Config>),
        );
}

#[derive(Resource)]
pub struct Settings {
    pub fov: f32,
    pub sound: Sound,

    // game state things
    /// Modal stack. kudo for the idea to @skyemakesgames
    /// Only relevant in [`Screen::Gameplay`]
    pub modals: Vec<Modal>,
    pub diagnostics: bool,
    pub muted: bool,
    pub paused: bool,
    pub sun_cycle: SunCycle,
    pub last_screen: Screen,
}

impl Settings {
    pub fn music(&self) -> f32 {
        self.sound.general * self.sound.music
    }
    pub fn sfx(&self) -> f32 {
        self.sound.general * self.sound.sfx
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            last_screen: Screen::Title,
            sun_cycle: SunCycle::DayNight,
            sound: Sound::default(),
            modals: vec![],
            diagnostics: true,
            paused: false,
            muted: false,
            fov: 45.0, // bevy default
        }
    }
}

fn inject_settings_from_cfg(mut commands: Commands, cfg: Res<Config>) {
    commands.insert_resource(Settings {
        sound: cfg.sound.clone(),
        fov: cfg.player.fov,
        diagnostics: true,
        ..Default::default()
    });
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    // Movement
    Forward,
    Backward,
    Left,
    Right,
    #[actionlike(DualAxis)]
    Move,
    #[actionlike(DualAxis)]
    LookAround,

    Jump,
    Sprint,
    Crouch,
    Dash,

    // Miscellaneous
    ToggleUiDebug,
    Back,
    ToggleMute,
    TogglePause,
    ToggleSunCycle,
    ToggleDiagnostics,
    Toot,
    FovIncr,
}

fn spawn_player_input_map(mut commands: Commands) {
    let mut input_map = InputMap::default()
        .with_dual_axis(Action::Move, GamepadStick::LEFT)
        .with_dual_axis(Action::LookAround, GamepadStick::RIGHT);

    input_map.insert(Action::Left, KeyCode::KeyA);
    input_map.insert(Action::Right, KeyCode::KeyD);
    input_map.insert(Action::Forward, KeyCode::KeyW);
    input_map.insert(Action::Backward, KeyCode::KeyS);

    input_map.insert(Action::Left, KeyCode::ArrowLeft);
    input_map.insert(Action::Right, KeyCode::ArrowRight);
    input_map.insert(Action::Forward, KeyCode::ArrowUp);
    input_map.insert(Action::Backward, KeyCode::ArrowDown);

    input_map.insert(Action::Jump, KeyCode::Space);
    input_map.insert(Action::Jump, GamepadButton::South);
    input_map.insert(Action::Sprint, KeyCode::ShiftLeft);
    input_map.insert(Action::Sprint, GamepadButton::LeftThumb);
    input_map.insert(Action::Crouch, KeyCode::ControlLeft);
    input_map.insert(Action::Crouch, GamepadButton::East);
    input_map.insert(Action::Dash, KeyCode::AltLeft);
    input_map.insert(Action::Dash, GamepadButton::LeftTrigger);

    input_map.insert(Action::Back, KeyCode::Escape);
    input_map.insert(Action::TogglePause, KeyCode::KeyP);
    input_map.insert(Action::ToggleMute, KeyCode::KeyM);
    input_map.insert(Action::ToggleSunCycle, KeyCode::KeyO);
    input_map.insert(Action::ToggleDiagnostics, KeyCode::KeyF);
    input_map.insert(Action::ToggleUiDebug, KeyCode::Backquote);

    input_map.insert(Action::Toot, KeyCode::KeyC);
    input_map.insert(Action::FovIncr, KeyCode::KeyV);

    commands.spawn(input_map);
}
