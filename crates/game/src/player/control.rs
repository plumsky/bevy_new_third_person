use super::*;
use bevy_tnua::{
    builtins::{TnuaBuiltinCrouch, TnuaBuiltinDash},
    control_helpers::TnuaSimpleAirActionsCounter,
};

// const FOLLOW_EPSILON: f32 = 0.01;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, movement.run_if(in_state(Screen::Gameplay)))
        // app.add_observer(movement)
        .add_observer(handle_sprint_in)
        .add_observer(handle_sprint_out)
        .add_observer(handle_jump)
        .add_observer(handle_dash)
        .add_observer(handle_attack)
        .add_observer(crouch_in)
        .add_observer(crouch)
        .add_observer(crouch_out);
}

/// Tnua configuration is tricky to grasp from the get go, this is the best demo:
/// <https://github.com/idanarye/bevy-tnua/blob/main/demos/src/character_control_systems/platformer_control_systems.rs>
// The height is the only mandatory field of the jump button.
// pub fn movement(
//     trigger: Trigger<Fired<Navigate>>,
//     cfg: Res<Config>,
//     mut player: Query<&mut Player>,
//     mut step_timer: Query<&mut StepTimer>,
//     camera: Query<&Transform, With<camera::SceneCamera>>,
//     mut air_counter: Query<&mut TnuaSimpleAirActionsCounter>,
//     mut tnua: Query<
//     (
//         &mut TnuaController,
//         &mut TnuaAvian3dSensorShape,
//         &mut Collider,
//     ),
//     (With<Player>, Without<camera::SceneCamera>),
//     >,
// ) -> Result {
//     let (mut controller, mut avian_sensor, mut collider) = tnua.single_mut()?;
//     let (mut player, mut step_timer) = (player.single_mut()?, step_timer.single_mut()?);
//     let mut direction = Vec3::ZERO;
//
//     // let (state, camera_transform) = (action.single()?, camera.single()?);
//     let camera_transform = camera.single()?;
//     let forward = camera_transform.forward().normalize();
//     let forward_flat = Vec3::new(forward.x, 0.0, forward.z);
//
//     // let left_stick = state.clamped_axis_pair(&Action::Move);
//     // if left_stick.length() >= 0.1 {
//     //     let right = forward_flat.cross(Vec3::Y).normalize();
//     //     direction = (right * left_stick.x) + (forward_flat * left_stick.y);
//     // }
//
//     if state.just_pressed(&Action::Sprint) {
//         let new = step_timer.0.duration() / SPRINT_FACTOR as u32;
//         step_timer.0.set_duration(new);
//         player.speed *= SPRINT_FACTOR;
//     }
//     if state.just_released(&Action::Sprint) {
//         let new = step_timer.0.duration() * SPRINT_FACTOR as u32;
//         step_timer.0.set_duration(new);
//         player.speed /= SPRINT_FACTOR;
//     }
//
//     // if state.just_pressed(&Action::Crouch) {
//     //     collider.set_scale(Vec3::new(1.0, 0.5, 1.0), 4);
//     //     avian_sensor.0.set_scale(Vec3::new(1.0, 0.5, 1.0), 4);
//     // }
//     // if state.pressed(&Action::Crouch) {
//     //     controller.action(TnuaBuiltinCrouch {
//     //         float_offset: -0.1,
//     //         ..Default::default()
//     //     });
//     //     player.speed /= 2.0;
//     // }
//     // if state.just_released(&Action::Crouch) {
//     //     collider.set_scale(Vec3::ONE, 4);
//     //     avian_sensor.0.set_scale(Vec3::ONE, 4);
//     // }
//
//     // if state.pressed(&Action::Right) {
//     //     let right = camera_transform.right().normalize();
//     //     let right_flat = Vec3::new(right.x, 0.0, right.z);
//     //     direction += right_flat;
//     // }
//     //
//     // if state.pressed(&Action::Left) {
//     //     let left = camera_transform.left().normalize();
//     //     let left_flat = Vec3::new(left.x, 0.0, left.z);
//     //     direction += left_flat;
//     // }
//     //
//     // if state.pressed(&Action::Forward) {
//     //     direction += forward_flat;
//     // }
//     //
//     // if state.pressed(&Action::Backward) {
//     //     let back = camera_transform.back().normalize();
//     //     let back_flat = Vec3::new(back.x, 0.0, back.z);
//     //     direction += back_flat;
//     // }
//
//     // NOTE: subject to change. UAL model is imported rotated 180 so we rotate it back
//     // let player_rot = Quat::from_rotation_y(PI) * direction;
//     //
//
//     let right = camera_transform.right().normalize();
//     let right_flat = Vec3::new(right.x, 0.0, right.z);
//     direction += right_flat;
//     // Feed the basis every frame. Even if the player doesn't move - just use `desired_velocity:
//     // Vec3::ZERO`. `TnuaController` starts without a basis, which will make the character collider
//     // just fall.
//     controller.basis(TnuaBuiltinWalk {
//         // The `desired_velocity` determines how the character will move.
//         desired_velocity: direction.normalize_or_zero() * player.speed,
//         desired_forward: Dir3::new(direction).ok(),
//         // The `float_height` must be greater (even if by little) from the distance between the
//         // character's center and the lowest point of its collider.
//         float_height: 0.01,
//         ..Default::default()
//     });
//
//     let mut air_counter = air_counter.single_mut()?;
//     if state.pressed(&Action::Jump) {
//         // TODO: figure out jump timer with tnua
//         // let mut timer = jump_timer.single_mut()?;
//         // if state.pressed(&Action::Jump) && timer.0.tick(time.delta()).just_finished() {
//
//         // Feed the jump action every frame as long as the player holds the jump button. If the player
//         // stops holding the jump button, simply stop feeding the action.
//         air_counter.update(controller.as_mut());
//
//         controller.action(TnuaBuiltinJump {
//             height: 1.0,
//             allow_in_air: true,
//             ..Default::default()
//         });
//     }
//
//     if state.just_pressed(&Action::Dash) {
//         controller.action(TnuaBuiltinDash {
//             speed: 50.,
//             // Dashing is also an action, but because it has directions we need to provide said
//             // directions. `displacement` is a vector that determines where the jump will bring
//             // us. Note that even after reaching the displacement, the character may still have
//             // some leftover velocity (configurable with the other parameters of the action)
//             //
//             // The displacement is "frozen" when the action starts - user code does not have to
//             // worry about storing the original direction
//             displacement: direction.normalize() * cfg.player.movement.dash_distance,
//             // When set, the `desired_forward` of the dash action "overrides" the
//             // `desired_forward` of the walk basis. Like the displacement, it gets "frozen" -
//             // allowing to easily maintain a forward direction during the dash.
//             desired_forward: Dir3::new(direction).ok(),
//             allow_in_air: air_counter.air_count_for(TnuaBuiltinDash::NAME)
//                 <= cfg.player.movement.actions_in_air.into(),
//             ..Default::default()
//         });
//     }
//
//     //if let Some(touch_position) = touch_input.first_pressed_position() {
//     //    if let Ok(touch_position) = camera.viewport_to_world_2d(camera_transform, touch_position) {
//     //        let diff = touch_position - player.translation.xy();
//     //        if diff.length() > FOLLOW_EPSILON {
//     //            player_movement = diff.normalize();
//     //        }
//     //    }
//     //}
//     Ok(())
// }

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
    _: Trigger<Started<Sprint>>,
    cfg: Res<Config>,
    mut player_query: Query<(&mut Player, &mut StepTimer, &GameplayCtx), With<GameplayCtx>>,
) -> Result {
    for (mut player, mut step_timer, _ctx) in player_query.iter_mut() {
        let new_duration = step_timer.0.duration() / cfg.player.movement.sprint_factor as u32;
        step_timer.0.set_duration(new_duration);
        player.speed *= cfg.player.movement.sprint_factor;
        // info!("Sprint started for entity: {:?}", ctx);
    }

    Ok(())
}

fn handle_sprint_out(
    trigger: Trigger<Completed<Sprint>>,
    cfg: Res<Config>,
    mut player_query: Query<(&mut Player, &mut StepTimer), With<GameplayCtx>>,
) -> Result {
    let entity = trigger.target();
    let (mut player, mut step_timer) = player_query.get_mut(entity)?;

    let new_duration = step_timer.0.duration() * cfg.player.movement.sprint_factor as u32;
    step_timer.0.set_duration(new_duration);
    player.speed = cfg.player.movement.speed;

    info!("Sprint completed for entity: {:?}", entity);
    Ok(())
}

fn handle_jump(
    trigger: Trigger<Started<Jump>>,
    cfg: Res<Config>,
    mut player_query: Query<
        (&mut TnuaController, &mut TnuaSimpleAirActionsCounter),
        With<GameplayCtx>,
    >,
) -> Result {
    let entity = trigger.target();
    let (mut controller, mut air_counter) = player_query.get_mut(entity)?;

    air_counter.update(controller.as_mut()); // Update air counter

    controller.action(TnuaBuiltinJump {
        height: 2.0,
        allow_in_air: true,
        ..Default::default()
    });

    info!("Jump triggered for entity: {:?}", entity);
    Ok(())
}

fn handle_dash(
    trigger: Trigger<Started<Dash>>,
    cfg: Res<Config>,
    actions: Single<&Actions<GameplayCtx>>,
    camera: Query<&Transform, With<SceneCamera>>,
    mut player_query: Query<(&mut TnuaController, &TnuaSimpleAirActionsCounter)>,
) -> Result {
    let entity = trigger.target();
    let (mut controller, air_counter) = player_query.get_mut(entity)?;
    let camera_transform = camera.single()?;

    let forward = camera_transform.forward().normalize();
    let forward_flat = Vec3::new(forward.x, 0.0, forward.z);
    let right = forward_flat.cross(Vec3::Y).normalize();

    // To get the dash direction, we need the *current* navigate value if available.
    // This requires getting the Actions component directly or using a different trigger for Dash
    // that includes the input value, or having a system that pre-calculates the current movement direction.
    // For simplicity, let's assume we use the camera's forward direction as the dash direction for now.
    // A more robust solution might involve querying the `Actions<GameplayCtx>` component directly
    // within this system to get the current `Maps` value.

    // For now, let's use a simple forward dash relative to camera or player's facing direction.
    // Ideally, you'd want the last `Maps` direction here.
    let dash_direction = forward_flat; // Or get from Actions.value(Navigate) if it's continuously updated

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

fn handle_attack(trigger: Trigger<Started<Attack>>, mut commands: Commands) {
    let entity = trigger.target();
    info!("Attack triggered by entity: {:?}", entity);
    // TODO: Hit
}

pub fn crouch_in(
    _: Trigger<Fired<Crouch>>,
    cfg: Res<Config>,
    mut player: Query<&mut Player>,
    mut tnua: Query<
        (&mut TnuaAvian3dSensorShape, &mut Collider),
        (With<Player>, Without<SceneCamera>),
    >,
) -> Result {
    let (mut avian_sensor, mut collider) = tnua.single_mut()?;
    let mut player = player.single_mut()?;

    collider.set_scale(Vec3::new(1.0, 0.5, 1.0), 4);
    avian_sensor.0.set_scale(Vec3::new(1.0, 0.5, 1.0), 4);
    player.speed *= cfg.player.movement.crouch_factor;

    Ok(())
}

pub fn crouch(_: Trigger<Ongoing<Crouch>>, mut tnua: Query<&mut TnuaController, With<Player>>) {
    if let Ok(mut controller) = tnua.single_mut() {
        controller.action(TnuaBuiltinCrouch {
            float_offset: -0.1,
            ..Default::default()
        });
    }
}

pub fn crouch_out(
    _: Trigger<Completed<Crouch>>,
    cfg: Res<Config>,
    mut player: Query<&mut Player>,
    mut tnua: Query<
        (&mut TnuaAvian3dSensorShape, &mut Collider),
        (With<Player>, Without<SceneCamera>),
    >,
) -> Result {
    let (mut avian_sensor, mut collider) = tnua.single_mut()?;
    let mut player = player.single_mut()?;

    collider.set_scale(Vec3::ONE, 4);
    avian_sensor.0.set_scale(Vec3::ONE, 4);

    player.speed = cfg.player.movement.speed;

    Ok(())
}
