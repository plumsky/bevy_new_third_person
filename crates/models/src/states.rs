use super::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<GameState>();
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct GameState {
    /// Modal stack. kudo for the idea to @skyemakesgames
    /// Only relevant in [`Screen::Gameplay`]
    pub modals: Vec<Modal>,
    pub last_screen: Screen,

    pub diagnostics: bool,
    pub debug_ui: bool,
    pub paused: bool,
    pub muted: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            modals: vec![],
            last_screen: Screen::Title,
            diagnostics: true,
            debug_ui: false,
            paused: false,
            muted: false,
        }
    }
}

impl GameState {
    pub fn reset(&mut self) {
        self.modals.clear();
        self.paused = false;
        self.muted = false;
    }
}
