use bevy::prelude::*;

pub mod input_dispatch;
pub mod pre_load;
pub mod settings;

pub use input_dispatch::*;
pub use pre_load::*;
pub use settings::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((settings::plugin, input_dispatch::plugin));
}

/// The game's main screen states.
/// See <https://bevy-cheatbook.github.io/programming/states.html>
/// Or <https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs>
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Screen {
    // Bevy tribute <3
    #[default]
    Splash,
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    Tutorial,
    Credits,
    Settings,
    // Here the menu is drawn and waiting for player interaction
    Title,
    // During this State the actual game logic is executed
    Gameplay,
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AppSystems {
    TickTimers,
    RecordInput,
    Update,
}

#[derive(Debug, Clone)]
pub enum Modal {
    Main,
    Settings,
}

#[derive(Debug)]
pub enum SunCycle {
    DayNight,
    Nimbus,
}
