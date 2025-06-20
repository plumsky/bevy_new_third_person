use super::*;

/// Macro to hide the derive trait boilerplate
macro_rules! declare_markers {
  ( $( $name:ident ),* ) => {
        $(
            #[derive(Component, Reflect)]
            #[reflect(Component)]
            pub struct $name;
        )*
    };
}

declare_markers!(
    SceneCamera,
    // scene
    Sun,
    Moon,
    Rock,
    // TODO: The idea is to create a boombox with spatial audio
    // <https://github.com/bevyengine/bevy/blob/main/examples/audio/spatial_audio_3d.rs>
    Boombox,
    SunCycleLabel,
    // user input
    InputCtx,
    // UI
    PerfUi,
    GameplayUi,
    PauseIcon,
    MuteIcon,
    MenuModal,
    // settings
    TabBar,
    TabContent,
    SettingsModal,
    GeneralVolumeLabel,
    MusicVolumeLabel,
    SfxVolumeLabel,
    FovLabel
);

macro_rules! timers {
  ( $( $name:ident ),* ) => {
        $(
            #[derive(Component, Reflect, Deref, DerefMut)]
            #[reflect(Component)]
            pub struct $name(pub Timer);
        )*
    };
}

timers!(JumpTimer, StepTimer);
