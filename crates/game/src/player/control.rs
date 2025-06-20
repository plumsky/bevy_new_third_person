use super::*;
use bevy_tnua::{
    builtins::{TnuaBuiltinCrouch, TnuaBuiltinDash},
    control_helpers::TnuaSimpleAirActionsCounter,
};

// const FOLLOW_EPSILON: f32 = 0.01;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, movement.in_set(TnuaUserControlsSystemSet))
        .add_observer(handle_sprint_in)
        .add_observer(handle_sprint_out)
        .add_observer(handle_jump)
        .add_observer(handle_dash)
        // .add_observer(handle_attack)
        .add_observer(crouch_in)
        .add_observer(crouch)
        .add_observer(crouch_out);
}

/// Tnua configuration is tricky to grasp from the get go, this is the best demo:
/// <https://github.com/idanarye/bevy-tnua/blob/main/demos/src/character_control_systems/platformer_control_systems.rs>
fn movement(
    // cfg: Res<Config>,
    camera: Query<&Transform, With<SceneCamera>>,
    mut player_query: Query<(&mut Player, &mut TnuaController)>,
    actions: Single<&Actions<GameplayCtx>>,
) -> Result {
    let actions = actions.into_inner();
    let input_value = actions.value::<Navigate>()?.as_axis2d();

    for (player_data, mut controller) in player_query.iter_mut() {
        let camera_transform = camera.single()?;

        let forward = camera_transform.forward().normalize();
        let forward_flat = Vec3::new(forward.x, 0.0, forward.z);
        let right = forward_flat.cross(Vec3::Y).normalize();

        let mut direction = (right * input_value.x) + (forward_flat * input_value.y);
        direction = direction.normalize_or_zero();

        controller.basis(TnuaBuiltinWalk {
            desired_velocity: direction * player_data.speed,
            desired_forward: Dir3::new(direction).ok(),
            float_height: 0.01,
            ..Default::default()
        });
    }

    Ok(())
}

fn handle_sprint_in(
    on: Trigger<Started<Sprint>>,
    cfg: Res<Config>,
    mut player_query: Query<(&mut Player, &mut StepTimer), With<GameplayCtx>>,
) -> Result {
    let (mut player, mut step_timer) = player_query.get_mut(on.target())?;

    info!("IN: new sprint timer: {:?}", step_timer.duration());
    let new_duration = step_timer.duration() / cfg.player.movement.sprint_factor as u32;
    info!("IN: new sprint timer: {new_duration:?}");
    step_timer.set_duration(new_duration);
    player.speed *= cfg.player.movement.sprint_factor;
    // info!("Sprint started for entity: {:?}", ctx);

    Ok(())
}

fn handle_sprint_out(
    on: Trigger<Completed<Sprint>>,
    cfg: Res<Config>,
    mut player_query: Query<(&mut Player, &mut StepTimer), With<GameplayCtx>>,
) -> Result {
    let entity = on.target();
    for (mut player, mut step_timer) in player_query.iter_mut() {
        info!("OUT: sprint timer: {:?}", step_timer.duration());
        let new_duration = step_timer.duration() * cfg.player.movement.sprint_factor as u32;
        info!("OUT: new sprint timer: {new_duration:?}");
        step_timer.set_duration(new_duration);
        player.speed = cfg.player.movement.speed;
    }

    info!("Sprint completed for entity: {:?}", entity);
    Ok(())
}

fn handle_jump(
    on: Trigger<Started<Jump>>,
    // cfg: Res<Config>,
    mut player_query: Query<
        (&mut TnuaController, &mut TnuaSimpleAirActionsCounter),
        With<GameplayCtx>,
    >,
) -> Result {
    let (mut controller, mut air_counter) = player_query.get_mut(on.target())?;

    air_counter.update(controller.as_mut()); // Update air counter
    controller.action(TnuaBuiltinJump {
        height: 2.0,
        allow_in_air: true,
        ..Default::default()
    });

    Ok(())
}

fn handle_dash(
    on: Trigger<Started<Dash>>,
    cfg: Res<Config>,
    actions: Single<&Actions<GameplayCtx>>,
    mut player_query: Query<(&mut TnuaController, &TnuaSimpleAirActionsCounter)>,
) -> Result {
    let (mut controller, air_counter) = player_query.get_mut(on.target())?;
    let navigate = actions.value::<Navigate>()?.as_axis2d();

    // To get the dash direction, we need the *current* navigate value if available.
    // This requires getting the Actions component directly or using a different trigger for Dash
    // that includes the input value, or having a system that pre-calculates the current movement direction.
    // For simplicity, let's assume we use the camera's forward direction as the dash direction for now.
    // A more robust solution might involve querying the `Actions<GameplayCtx>` component directly
    // within this system to get the current `Maps` value.

    // For now, let's use a simple forward dash relative to camera or player's facing direction.
    // Ideally, you'd want the last `Maps` direction here.
    let dash_direction = Vec3::new(navigate.x, 0.0, navigate.y);

    controller.action(TnuaBuiltinDash {
        speed: 50.,
        displacement: dash_direction.normalize() * cfg.player.movement.dash_distance,
        desired_forward: Dir3::new(dash_direction).ok(),
        allow_in_air: air_counter.air_count_for(TnuaBuiltinDash::NAME)
            <= cfg.player.movement.actions_in_air.into(),
        ..Default::default()
    });

    Ok(())
}

// fn handle_attack(on: Trigger<Started<Attack>>, mut commands: Commands) {
//     let entity = on.target();
//     // TODO: Hit
// }

pub fn crouch_in(
    on: Trigger<Fired<Crouch>>,
    cfg: Res<Config>,
    mut player: Query<&mut Player>,
    mut tnua: Query<
        (&mut TnuaAvian3dSensorShape, &mut Collider),
        (With<Player>, Without<SceneCamera>),
    >,
) -> Result {
    let (mut avian_sensor, mut collider) = tnua.single_mut()?;
    let mut player = player.get_mut(on.target())?;

    collider.set_scale(Vec3::new(1.0, 0.5, 1.0), 4);
    avian_sensor.0.set_scale(Vec3::new(1.0, 0.5, 1.0), 4);
    player.speed *= cfg.player.movement.crouch_factor;

    Ok(())
}

pub fn crouch(on: Trigger<Ongoing<Crouch>>, mut player: Query<&mut TnuaController, With<Player>>) {
    if let Ok(mut controller) = player.get_mut(on.target()) {
        controller.action(TnuaBuiltinCrouch {
            float_offset: -0.1,
            ..Default::default()
        });
    }
}

pub fn crouch_out(
    on: Trigger<Completed<Crouch>>,
    cfg: Res<Config>,
    mut player: Query<&mut Player>,
    mut tnua: Query<
        (&mut TnuaAvian3dSensorShape, &mut Collider),
        (With<Player>, Without<SceneCamera>),
    >,
) -> Result {
    let (mut avian_sensor, mut collider) = tnua.get_mut(on.target())?;
    let mut player = player.get_mut(on.target())?;

    collider.set_scale(Vec3::ONE, 4);
    avian_sensor.0.set_scale(Vec3::ONE, 4);

    player.speed = cfg.player.movement.speed;

    Ok(())
}
