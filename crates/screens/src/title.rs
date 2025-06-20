use super::*;

/// This plugin is responsible for the game menu
/// The menu is only drawn during the State [`Screen::Title`] and is removed when that state is exited
pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), setup_menu);
}

fn setup_menu(
    //font: Res<Fira>,
    mut commands: Commands,
) {
    commands.spawn((
        StateScoped(Screen::Title),
        ui_root("Title"),
        // Crutch until we can use #cfg in children![] macro
        // https://github.com/bevyengine/bevy/issues/18953
        #[cfg(target_arch = "wasm32")]
        children![
            BackgroundColor(TRANSLUCENT),
            btn_big("Play", to::gameplay_or_loading),
            btn_big("Credits", to::credits),
            btn_big("Settings", to::settings),
        ],
        #[cfg(not(target_arch = "wasm32"))]
        children![
            BackgroundColor(TRANSLUCENT),
            btn_big("Play", to::gameplay_or_loading),
            btn_big("Credits", to::credits),
            btn_big("Settings", to::settings),
            btn_big("Exit", exit_app)
        ],
    ));
}

#[cfg(not(target_arch = "wasm32"))]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
