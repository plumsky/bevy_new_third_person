use super::*;
use bevy::scene::SceneInstanceReady;
use bevy_third_person_camera::*;
use bevy_tnua::{TnuaAnimatingState, control_helpers::TnuaSimpleAirActionsCounter};
use bevy_tnua_avian3d::*;
use std::f32::consts::PI;

mod animation;
mod control;

pub use animation::*;
pub use control::*;

/// This plugin handles player related stuff like movement, shooting
/// Player logic is only active during the State `Screen::Playing`
pub fn plugin(app: &mut App) {
    app.add_plugins((
        ThirdPersonCameraPlugin,
        TnuaControllerPlugin::new(FixedUpdate),
        TnuaAvian3dPlugin::new(FixedUpdate),
        control::plugin,
    ));

    app.configure_sets(PostUpdate, CameraSyncSet.after(PhysicsSet::Sync))
        .add_systems(OnEnter(Screen::Gameplay), spawn_player)
        .add_systems(
            Update,
            animating
                .in_set(TnuaUserControlsSystemSet)
                .run_if(in_state(Screen::Gameplay)),
        )
        .add_observer(player_post_spawn);
}

pub fn spawn_player(
    cfg: Res<Config>,
    models: Res<Models>,
    mut commands: Commands,
    gltf_assets: Res<Assets<Gltf>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    camera: Query<&Transform, With<SceneCamera>>,
) -> Result {
    let Some(gltf) = gltf_assets.get(&models.player) else {
        return Ok(());
    };

    let camera_transform = camera.single()?;
    let mut forward = camera_transform.forward().normalize();
    forward.y = 0.0;

    let player_rot = Quat::from_rotation_y(PI);
    let mesh = SceneRoot(gltf.scenes[0].clone());
    let pos = Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)).with_rotation(player_rot);
    let player = Player {
        id: Entity::PLACEHOLDER,
        speed: cfg.player.movement.speed,
        animation_state: AnimationState::StandIdle,
    };

    let collider = Collider::capsule(cfg.player.hitbox.radius, cfg.player.hitbox.height);

    commands
        .spawn((
            StateScoped(Screen::Gameplay),
            pos,
            player,
            // input context
            (
                GameplayCtx,
                CurrentCtx(Context::Gameplay),
                Actions::<GameplayCtx>::default(),
            ),
            ThirdPersonCameraTarget,
            // tnua stuff
            (
                TnuaController::default(),
                // Tnua can fix the rotation, but the character will still get rotated before it can do so.
                // By locking the rotation we can prevent this.
                LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
                TnuaAnimatingState::<AnimationState>::default(),
                TnuaSimpleAirActionsCounter::default(),
                // A sensor shape is not strictly necessary, but without it we'll get weird results.
                TnuaAvian3dSensorShape(collider.clone()),
            ),
            // physics
            (
                collider,
                RigidBody::Dynamic,
                Friction::ZERO.with_combine_rule(CoefficientCombine::Multiply),
            ),
            JumpTimer(Timer::from_seconds(cfg.timers.jump, TimerMode::Repeating)),
            StepTimer(Timer::from_seconds(cfg.timers.step, TimerMode::Repeating)),
            InheritedVisibility::default(), // silence the warning because of adding SceneRoot as a child
        ))
        // spawn character mesh as child to adjust mesh position relative to the player origin
        .with_children(|parent| {
            let mut e = parent.spawn((Transform::from_xyz(0.0, -1.0, 0.0), mesh));
            e.observe(prepare_animations);
        })
        .observe(player_post_spawn);

    // DEBUG
    // let collider_mesh = Mesh::from(Capsule3d::new(
    //     cfg.player.hitbox.radius,
    //     cfg.player.hitbox.height,
    // ));
    // let debug_collider_mesh = Mesh3d(meshes.add(collider_mesh.clone()));
    // let debug_collider_color: MeshMaterial3d<StandardMaterial> =
    //     MeshMaterial3d(materials.add(Color::srgba(0.9, 0.9, 0.9, 0.1)));
    // e.with_children(|parent| {
    //     e.with_children(|parent| {
    //         parent.spawn((
    //             debug_collider_mesh,
    //             debug_collider_color,
    //             Transform::from_xyz(0.0, 0.9, 0.0),
    //         ));
    //     });
    // });

    Ok(())
}

fn player_post_spawn(
    on: Trigger<OnAdd, Player>,
    mut players: Query<&mut Player>,
    mut commands: Commands,
) {
    let player = on.target();
    if let Ok(mut p) = players.get_mut(player) {
        p.id = player; // update player id with spawned entity
    }
    info!("Switch CTX for: {player}");
    commands.trigger(SwitchInputCtx::new(player, Context::Gameplay));
    commands.trigger(SwitchInputCtx::from_context(Context::Gameplay));
}
