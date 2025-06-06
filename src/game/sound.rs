use crate::prelude::*;
use bevy::prelude::*;
use bevy_seedling::{pool::Sampler, prelude::*};
// use bevy_seedling::{pool::Sampler, prelude::*};
use leafwing_input_manager::prelude::*;
use rand::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnExit(Screen::Gameplay), stop_soundtrack)
        .add_systems(OnEnter(Screen::Gameplay), start_or_resume_soundtrack)
        .add_systems(
            Update,
            movement_sound
                .run_if(in_state(Screen::Gameplay))
                .after(player::spawn_player),
        );
}

// TODO: implement different music states
// good structure in this example: <https://github.com/bevyengine/bevy/blob/main/examples/audio/soundtrack.rs#L29>
fn start_or_resume_soundtrack(
    mut cmds: Commands,
    settings: Res<Settings>,
    sources: ResMut<AudioSources>,
    // boombox: Query<Entity, With<Boombox>>,
    mut music_query: Query<(&Sampler, &mut PlaybackSettings), With<Music>>,
) -> Result {
    if let Ok((player, mut instance)) = music_query.single_mut() {
        if !player.is_playing() {
            info!("player is not playing");
            instance.play();
        }
    } else {
        let handle = *[&sources.bg_music].choose(&mut thread_rng()).unwrap();
        // // Play music from boombox entity
        // cmds
        //     .entity(boombox.single()?)
        //     .insert(music(handle.clone(), vol));
        // Or just play music
        cmds.spawn(music(handle.clone(), settings.sfx()));
    }

    Ok(())
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
    mut cmds: Commands,
    time: Res<Time>,
    settings: Res<Settings>,
    mut step_timer: Query<&mut player::StepTimer>,
    sources: ResMut<AudioSources>,
    state: Query<&ActionState<Action>>,
    player_pos: Query<&Transform, With<player::Player>>,
) -> Result {
    let Ok(player_pos) = player_pos.single() else {
        return Ok(());
    };
    let Ok(mut step_timer) = step_timer.single_mut() else {
        return Ok(());
    };
    let state = state.single()?;

    // WALK SOUND
    if step_timer.0.tick(time.delta()).just_finished() {
        // TODO: only run animation after tick
        if (state.pressed(&Action::Forward)
            | state.pressed(&Action::Backward)
            | state.pressed(&Action::Left)
            | state.pressed(&Action::Right))
            && player_pos.translation.y <= 2.0
        {
            let mut rng = thread_rng();
            let i = rng.gen_range(0..sources.steps.len());
            let handle = sources.steps[i].clone();
            cmds.spawn(sfx(handle, settings.sfx()));
        }
    }

    // TODO: JUMP SOUND

    // TODO: DASH SOUND

    // TODO: CROUCH WALK SOUND

    Ok(())
}
