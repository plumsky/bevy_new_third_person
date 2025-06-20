use super::*;
use serde::Deserialize;
use std::{error::Error, fs};

pub fn plugin(app: &mut App) {
    app.init_resource::<Settings>().init_resource::<ActiveTab>();
    app.add_systems(
        OnEnter(Screen::Gameplay),
        inject_settings_from_cfg.run_if(resource_exists::<Config>),
    );
}

pub const SETTINGS_PATH: &str = "assets/settings.ron";

#[derive(Resource, Reflect, Deserialize, Serialize)]
#[reflect(Resource)]
pub struct Settings {
    pub fov: f32,
    pub sound: Sound,

    // game state things
    /// Modal stack. kudo for the idea to @skyemakesgames
    /// Only relevant in [`Screen::Gameplay`]
    pub modals: Vec<Modal>,
    pub sun_cycle: SunCycle,
    pub last_screen: Screen,
    pub keybind: Keybind,

    pub diagnostics: bool,
    pub debug_ui: bool,
    pub paused: bool,
    pub muted: bool,
}

impl Settings {
    pub fn music(&self) -> Volume {
        Volume::Linear(self.sound.general * self.sound.music)
    }
    pub fn sfx(&self) -> Volume {
        Volume::Linear(self.sound.general * self.sound.sfx)
    }
    pub fn read() -> Result<Self, Box<dyn Error>> {
        let content = fs::read_to_string(SETTINGS_PATH)?;
        let settings = ron::from_str(&content).unwrap_or_default();
        Ok(settings)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let content = ron::ser::to_string_pretty(self, Default::default())?;
        fs::write(SETTINGS_PATH, content)?;
        Ok(())
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
            debug_ui: false,
            paused: false,
            muted: false,
            fov: 45.0, // bevy default
            keybind: Keybind::default(),
        }
    }
}

fn inject_settings_from_cfg(mut commands: Commands, cfg: Res<Config>) {
    let settings = match Settings::read() {
        Ok(settings) => {
            info!("loaded settings from '{SETTINGS_PATH}'");
            settings
        }
        Err(e) => {
            info!("unable to load settings from '{SETTINGS_PATH}', switching to defaults: {e}");
            Default::default()
        }
    };

    commands.insert_resource(Settings {
        sound: cfg.sound.clone(),
        ..settings
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Reflect, Component)]
#[reflect(Component)]
pub enum UiTab {
    #[default]
    Audio,
    Video,
    Keybindings,
}

#[derive(Resource, Default)]
pub struct ActiveTab(pub UiTab);
