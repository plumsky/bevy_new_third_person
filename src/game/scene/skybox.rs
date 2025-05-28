use crate::prelude::*;
use bevy::{
    core_pipeline::{bloom::Bloom, tonemapping::Tonemapping},
    pbr::{Atmosphere, AtmosphereSettings, CascadeShadowConfigBuilder, light_consts::lux},
    prelude::*,
    render::camera::Exposure,
};
use leafwing_input_manager::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::Gameplay),
        add_skybox_to_camera.after(camera::spawn_camera),
    )
    .add_systems(OnExit(Screen::Gameplay), rm_skybox_from_camera)
    .add_systems(
        Update,
        (toggle_sun_cycle, sun_cycle).run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Component)]
pub struct Sun;
#[derive(Component)]
pub struct Moon;

#[derive(Debug)]
pub enum SunCycle {
    DayNight,
    Nimbus,
}

/// Mainly this example:
/// <https://bevyengine.org/examples/3d-rendering/atmosphere/>
pub fn add_skybox_to_camera(
    cfg: Res<Config>,
    mut commands: Commands,
    mut camera: Query<Entity, With<Camera3d>>,
) -> Result {
    let camera = camera.single_mut()?;

    let cascade_shadow_config = CascadeShadowConfigBuilder {
        first_cascade_far_bound: 0.3,
        maximum_distance: cfg.physics.shadow_distance,
        ..default()
    }
    .build();

    // Sun
    commands.spawn((
        StateScoped(Screen::Gameplay),
        DirectionalLight {
            color: SUN,
            shadows_enabled: true,
            illuminance: lux::FULL_DAYLIGHT,
            ..Default::default()
        },
        Sun,
        Transform::from_translation(Vec3::new(0.0, 0.0, cfg.geom.main_plane)),
        cascade_shadow_config.clone(),
    ));

    // Moon
    commands.spawn((
        StateScoped(Screen::Gameplay),
        DirectionalLight {
            color: MOON,
            shadows_enabled: true,
            illuminance: lux::FULL_MOON_NIGHT,
            ..Default::default()
        },
        Moon,
        Transform::from_translation(Vec3::new(0.0, 0.0, -cfg.geom.main_plane)),
        cascade_shadow_config,
    ));

    commands.entity(camera).insert((
        // This is the component that enables atmospheric scattering for a camera
        Atmosphere::EARTH,
        // The scene is in units of 10km, so we need to scale up the
        // aerial view lut distance and set the scene scale accordingly.
        // Most usages of this feature will not need to adjust this.
        AtmosphereSettings {
            aerial_view_lut_max_distance: 3.2e4,
            scene_units_to_m: 1e+4,
            ..Default::default()
        },
        Tonemapping::BlenderFilmic,
        Exposure::OVERCAST,
        Bloom::NATURAL,
    ));

    if cfg.physics.fog {
        commands.entity(camera).insert(fog(cfg));
    }

    Ok(())
}

pub fn rm_skybox_from_camera(
    mut commands: Commands,
    mut camera: Query<Entity, With<Camera3d>>,
) -> Result {
    let camera = camera.single_mut()?;
    commands
        .entity(camera)
        .remove::<Atmosphere>()
        .remove::<AtmosphereSettings>()
        .remove::<Exposure>()
        .remove::<Bloom>()
        .remove::<DistanceFog>()
        .remove::<Tonemapping>();

    Ok(())
}

pub fn fog(cfg: Res<Config>) -> impl Bundle {
    DistanceFog {
        color: Color::srgba(0.35, 0.48, 0.66, 1.0),
        directional_light_color: Color::srgba(1.0, 0.95, 0.85, 0.5),
        directional_light_exponent: cfg.physics.fog_directional_light_exponent,
        falloff: FogFalloff::from_visibility_colors(
            cfg.physics.fog_visibility, // distance in world units up to which objects retain visibility (>= 5% contrast)
            Color::srgb(0.35, 0.5, 0.66), // atmospheric extinction color (after light is lost due to absorption by atmospheric particles)
            Color::srgb(0.8, 0.844, 1.0), // atmospheric inscattering color (light gained due to scattering from the sun)
        ),
    }
}

fn sun_cycle(
    settings: Res<Settings>,
    mut sky_lights: Query<&mut Transform, With<DirectionalLight>>,
    time: Res<Time>,
) {
    match settings.sun_cycle {
        SunCycle::DayNight => sky_lights
            .iter_mut()
            .for_each(|mut tf| tf.rotate_x(-time.delta_secs() * std::f32::consts::PI / 50.0)),
        SunCycle::Nimbus => sky_lights
            .iter_mut()
            .for_each(|mut tf| tf.rotate_y(-time.delta_secs() * std::f32::consts::PI / 50.0)),
    }
}

#[derive(Component)]
pub struct SunCycleLabel;

fn toggle_sun_cycle(
    mut settings: ResMut<Settings>,
    action: Query<&ActionState<Action>>,
    mut label: Query<&mut Text, With<SunCycleLabel>>,
) -> Result {
    let state = action.single()?;
    if state.just_pressed(&Action::ToggleSunCycle) {
        let mut text = label.single_mut()?;
        match settings.sun_cycle {
            SunCycle::Nimbus => {
                settings.sun_cycle = SunCycle::DayNight;
            }
            SunCycle::DayNight => {
                settings.sun_cycle = SunCycle::Nimbus;
            }
        }
        *text = format!("O - toggle sun cycle: {:?}", settings.sun_cycle).into();
    }

    Ok(())
}
