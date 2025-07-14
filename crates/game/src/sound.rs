use super::*;
use rand::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnExit(Screen::Gameplay), stop_soundtrack)
        .add_systems(OnEnter(Screen::Gameplay), start_soundtrack);
}

// TODO: implement different music states
// TODO: basic track/mood change per zone
// good structure in this example: <https://github.com/bevyengine/bevy/blob/main/examples/audio/soundtrack.rs#L29>
fn start_soundtrack(
    mut cmds: Commands,
    settings: Res<Settings>,
    sources: ResMut<AudioSources>,
    // boombox: Query<Entity, With<Boombox>>,
) {
    let mut rng = thread_rng();
    let handle = *[&sources.bg_music].choose(&mut rng).unwrap();

    // // Play music from boombox entity
    // cmds
    //     .entity(boombox.single()?)
    //     .insert(music(handle.clone(), settings.music());
    // Or just play music
    cmds.spawn((
        Music,
        SamplePlayer::new(handle.clone())
            .with_volume(settings.music())
            .looping(),
    ));
}

fn stop_soundtrack(
    // boombox: Query<Entity, With<Boombox>>,
    mut bg_music: Query<&mut PlaybackSettings, With<Music>>,
) {
    for mut s in bg_music.iter_mut() {
        s.pause();
    }
}
