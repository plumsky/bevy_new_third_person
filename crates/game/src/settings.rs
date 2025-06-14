use super::*;
use leafwing_input_manager::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<Settings>();
    app.add_plugins(InputManagerPlugin::<Action>::default())
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

#[derive(Debug, Clone)]
pub enum Modal {
    Main,
    Settings,
}

fn inject_settings_from_cfg(mut commands: Commands, cfg: Res<Config>) {
    commands.insert_resource(Settings {
        sound: cfg.sound.clone(),
        fov: cfg.player.fov,
        diagnostics: true,
        ..Default::default()
    });
}
