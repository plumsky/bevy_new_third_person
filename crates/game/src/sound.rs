use super::*;
use rand::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnExit(Screen::Gameplay), stop_soundtrack)
        .add_systems(OnEnter(Screen::Gameplay), start_soundtrack)
        .add_observer(movement_sound)
        .add_observer(dash_sound)
        .add_observer(jump_sound);
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

fn movement_sound(
    on: Trigger<Fired<Navigate>>,
    time: Res<Time>,
    settings: Res<Settings>,
    sources: ResMut<AudioSources>,
    tnua: Query<&TnuaController, With<Player>>,
    actions: Single<&Actions<GameplayCtx>>,
    mut cmds: Commands,
    mut step_timer: Query<&mut StepTimer, With<Player>>,
) -> Result {
    if settings.muted || settings.paused {
        return Ok(());
    }

    let controller = tnua.get(on.target())?;
    let mut step_timer = step_timer.get_mut(on.target())?;

    let Some((_, basis)) = controller.concrete_basis::<TnuaBuiltinWalk>() else {
        return Ok(());
    };

    // WALK SOUND
    if step_timer.tick(time.delta()).just_finished() && basis.standing_on_entity().is_some() {
        let mut rng = thread_rng();
        let i = rng.gen_range(0..sources.steps.len());
        let handle = if actions.value::<Crouch>()?.as_bool() {
            // TODO: select crouch steps
            sources.steps[i].clone()
        } else {
            sources.steps[i].clone()
        };
        cmds.spawn(SamplePlayer::new(handle).with_volume(settings.sfx()));
    }

    Ok(())
}

fn jump_sound(
    _: Trigger<Started<Jump>>,
    settings: Res<Settings>,
    sources: ResMut<AudioSources>,
    // jump_timer: Query<&JumpTimer, With<Player>>,
    mut cmds: Commands,
) -> Result {
    if settings.muted || settings.paused {
        return Ok(());
    }

    // let jump_timer = jump_timer.get(on.target())?;
    // if jump_timer.just_finished() {
    let mut rng = thread_rng();
    let i = rng.gen_range(0..sources.steps.len());
    let handle = sources.steps[i].clone();
    cmds.spawn(SamplePlayer::new(handle).with_volume(settings.sfx()));
    // }

    Ok(())
}

fn dash_sound(
    _: Trigger<Started<Dash>>,
    settings: Res<Settings>,
    sources: ResMut<AudioSources>,
    // jump_timer: Query<&JumpTimer, With<Player>>,
    mut cmds: Commands,
) -> Result {
    if settings.muted || settings.paused {
        return Ok(());
    }

    // let jump_timer = jump_timer.get(on.target())?;
    // if jump_timer.just_finished() {
    let mut rng = thread_rng();
    let i = rng.gen_range(0..sources.steps.len());
    let handle = sources.steps[i].clone();
    cmds.spawn(SamplePlayer::new(handle).with_volume(settings.sfx()));
    // }

    Ok(())
}
