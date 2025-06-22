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
    actions: Single<&Actions<GameplayCtx>>,
    camera: Query<&Transform, With<SceneCamera>>,
    mut player_query: Query<(&mut Player, &mut TnuaController)>,
) -> Result {
    let actions = actions.into_inner();
    for (player_data, mut controller) in player_query.iter_mut() {
        let cam_transform = camera.single()?;
        let input_value = actions.value::<Navigate>()?.as_axis2d();
        let direction = cam_transform.movement_direction(input_value);

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
    for (mut player, mut step_timer) in player_query.iter_mut() {
        info!("IN: new sprint timer: {:?}", step_timer.duration());
        let new_duration = step_timer.duration() / cfg.player.movement.sprint_factor as u32;
        info!("IN: new sprint timer: {new_duration:?}");
        step_timer.set_duration(new_duration);
        player.speed *= cfg.player.movement.sprint_factor;
        let entity = on.target();
        info!("Sprint started for entity: {:?}", ctx);
    }

    Ok(())
}

fn handle_sprint_out(
    on: Trigger<Completed<Navigate>>,
    cfg: Res<Config>,
    mut player_query: Query<(&mut Player, &mut StepTimer), With<GameplayCtx>>,
) -> Result {
    for (mut player, mut step_timer) in player_query.iter_mut() {
        info!("OUT: sprint timer: {:?}", step_timer.duration());
        let new_duration = step_timer.duration() * cfg.player.movement.sprint_factor as u32;
        info!("OUT: new sprint timer: {new_duration:?}");
        step_timer.set_duration(new_duration);
        player.speed = cfg.player.movement.speed;
    }

    let entity = on.target();
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
    camera: Query<&Transform, With<SceneCamera>>,
    mut player_query: Query<(&mut TnuaController, &TnuaSimpleAirActionsCounter)>,
) -> Result {
    let (mut controller, air_counter) = player_query.get_mut(on.target())?;
    let cam_transform = camera.single()?;
    let navigate = actions.value::<Navigate>()?.as_axis2d();
    let direction = cam_transform.movement_direction(navigate);

    controller.action(TnuaBuiltinDash {
        speed: 50.,
        displacement: direction * cfg.player.movement.dash_distance,
        desired_forward: Dir3::new(direction).ok(),
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
    on: Trigger<Started<Crouch>>,
    cfg: Res<Config>,
    mut player: Query<&mut Player>,
    mut tnua: Query<
        (&mut TnuaAvian3dSensorShape, &mut Collider),
        (With<Player>, Without<SceneCamera>),
    >,
) -> Result {
    let (mut avian_sensor, mut collider) = tnua.single_mut()?;
    let mut player = player.get_mut(on.target())?;

    info!("Crouch IN");
    collider.set_scale(Vec3::new(1.0, 0.5, 1.0), 4);
    avian_sensor.0.set_scale(Vec3::new(1.0, 0.5, 1.0), 4);
    player.speed *= cfg.player.movement.crouch_factor;

    Ok(())
}

pub fn crouch(on: Trigger<Ongoing<Crouch>>, mut player: Query<&mut TnuaController, With<Player>>) {
    if let Ok(mut controller) = player.get_mut(on.target()) {
        info!("Crouch");
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

    info!("Crouch OUT");
    collider.set_scale(Vec3::ONE, 4);
    avian_sensor.0.set_scale(Vec3::ONE, 4);

    player.speed = cfg.player.movement.speed;

    Ok(())
}
