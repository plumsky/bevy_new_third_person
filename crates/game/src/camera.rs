use super::*;
use bevy_third_person_camera::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera)
        .add_systems(
            OnEnter(Screen::Gameplay),
            (
                add_tpv_cam,
                add_skybox_to_camera.after(camera::spawn_camera),
            ),
        )
        .add_systems(
            OnExit(Screen::Gameplay),
            (rm_tpv_cam, rm_skybox_from_camera),
        )
        .add_observer(toggle_cam_cursor);
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        SceneCamera,
        Camera3d::default(),
        Msaa::Sample4,
        IsDefaultUiCamera,
        Transform::from_xyz(100., 50., 100.).looking_at(Vec3::ZERO, Vec3::Y),
        Camera {
            hdr: true,
            ..Default::default()
        },
    ));
}

fn add_tpv_cam(
    cfg: Res<Config>,
    mut commands: Commands,
    mut camera: Query<Entity, With<SceneCamera>>,
    mut scene_cam: Query<Entity, With<ThirdPersonCamera>>,
) -> Result {
    let camera = camera.single_mut()?;
    if scene_cam.single_mut().is_ok() {
        debug!("ThirdPersonCamera already exist");
        return Ok(());
    }

    commands.entity(camera).insert((
        ThirdPersonCamera {
            aim_speed: 3.0,     // default
            aim_zoom: 0.7,      // default
            zoom_enabled: true, // default
            zoom: Zoom::new(cfg.player.zoom.0, cfg.player.zoom.1),
            aim_enabled: true,
            offset_enabled: true,
            offset_toggle_enabled: true,
            cursor_lock_key: KeyCode::KeyL,
            gamepad_settings: CustomGamepadSettings::default(),
            // bounds: vec![Bound::NO_FLIP, Bound::ABOVE_FLOOR],
            ..default()
        },
        Projection::from(PerspectiveProjection {
            fov: cfg.player.fov.to_radians(),
            ..Default::default()
        }),
    ));

    Ok(())
}

fn rm_tpv_cam(mut commands: Commands, mut camera: Query<Entity, With<SceneCamera>>) {
    if let Ok(camera) = camera.single_mut() {
        commands.entity(camera).remove::<ThirdPersonCamera>();
    }
}

fn toggle_cam_cursor(_: Trigger<OnCamCursorToggle>, mut cam: Query<&mut ThirdPersonCamera>) {
    let Ok(mut cam) = cam.single_mut() else {
        return;
    };
    cam.cursor_lock_active = !cam.cursor_lock_active;
}

/// Helper trait to get direction of movement
pub trait MovementDirection {
    fn movement_direction(&self, input: Vec2) -> Vec3;
}

impl MovementDirection for Transform {
    fn movement_direction(&self, input: Vec2) -> Vec3 {
        let forward = self.forward();
        let forward_flat = Vec3::new(forward.x, 0.0, forward.z);
        let right = forward_flat.cross(Vec3::Y).normalize();
        let direction = (right * input.x) + (forward_flat * input.y);
        direction.normalize_or_zero()
    }
}
