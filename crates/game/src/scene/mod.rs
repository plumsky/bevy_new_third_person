use super::*;
use asset_loading::Models;
use avian3d::prelude::*;
use models::{Config, Screen};

pub mod player;
pub mod skybox;

pub use skybox::*;

/// This plugin handles loading and saving scenes
/// Scene logic is only active during the State `Screen::Playing`
pub fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default(),
        bevy_fix_gltf_coordinate_system::FixGltfCoordinateSystemPlugin,
        player::plugin,
        skybox::plugin,
    ))
    // .add_systems(Update, rotate_rock.run_if(in_state(Screen::Gameplay)))
    .add_systems(OnEnter(Screen::Gameplay), setup);
}

// TODO: The idea is to create a boombox with spatial audio
// <https://github.com/bevyengine/bevy/blob/main/examples/audio/spatial_audio_3d.rs>
// #[derive(Component)]
// pub struct Boombox;
#[derive(Component)]
pub struct Rock;

pub(crate) fn setup(
    cfg: Res<Config>,
    models: Res<Models>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    gltf_assets: Res<Assets<Gltf>>,
) {
    let Some(gltf) = gltf_assets.get(&models.rock) else {
        return;
    };
    let main_plane = cfg.geom.main_plane;

    // Plane
    let mesh = Mesh3d(meshes.add(Cuboid::new(main_plane, 0., main_plane)));
    let mat = MeshMaterial3d(materials.add(SAND_YELLOW));
    commands.spawn((
        StateScoped(Screen::Gameplay),
        mat,
        mesh,
        Transform::default(),
        RigidBody::Static,
        Collider::half_space(Vec3::Y),
    ));

    // Rock
    for mesh_handle in &gltf.meshes {
        if let Some(mesh) = meshes.get(mesh_handle) {
            info!(" vertex count: {:?}", mesh.count_vertices());
            // let mesh = gltf.named_meshes.get("mt_lp").expect("");
            let pos = Transform::from_translation(Vec3::new(5.0, 3.0, 5.0));
            commands.spawn((
                StateScoped(Screen::Gameplay),
                Rock,
                pos,
                Mesh3d(mesh),
                // children![mesh],
                RigidBody::Static,
                Collider::trimesh_from_mesh(mesh)
                    .expect("failed to create collider from rock mesh"),
            ));
            // You could now use `mesh` directly or clone it
            // e.g., commands.spawn(PbrBundle { mesh: mesh_handle.clone(), ..default() });
        }
    }

    let size = main_plane / 2.0;
    let geom = cfg.geom.clone();
    for i in 0..geom.quantity {
        let i = i as f32;
        let (low, upper) = (main_plane / 100.0, main_plane / 40.0);
        let step = (upper - low) / geom.quantity as f32;

        let y_size = low + step * i;
        let x_size = low + step * i;
        let (x, y, mut z) = (
            -size / 4.0 + i * x_size, // + step * 20.0,
            y_size / 2.0 + i * step,
            -size / 4.0,
        );
        let (mesh, color) = if i % 2.0 == 0.0 {
            (Mesh::from(Cuboid::new(x_size, y_size, x_size)), GREEN)
        } else {
            z += size / 2.0;
            (Mesh::from(Sphere::new(y_size)), LIGHT_BLUE)
        };
        let material = materials.add(StandardMaterial {
            base_color: color,
            #[cfg(feature = "enhanced")]
            specular_tint: Color::WHITE,
            ..default()
        });

        let mesh3d = Mesh3d(meshes.add(mesh.clone()));
        let mat = MeshMaterial3d(material.clone());
        let pos = Transform::from_xyz(x, y, z);
        commands.spawn((
            StateScoped(Screen::Gameplay),
            mat,
            pos,
            mesh3d,
            RigidBody::Static,
            Collider::trimesh_from_mesh(&mesh).expect("failed to create collider for mesh"),
        ));
    }

    // TODO: add spatial boombox object
    // // soundtrack boombox
    // commands.spawn((
    //     Boombox,
    //     Mesh3d(meshes.add(Sphere::new(0.2).mesh().uv(32, 18))),
    //     MeshMaterial3d(materials.add(LIGHT_BLUE)),
    //     Transform::from_xyz(0.0, 0.0, 0.0),
    // ));

    // to see something when suns go away
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0,
        ..Default::default()
    });
}

fn rotate_rock(time: Res<Time>, mut rock: Query<&mut Transform, With<Rock>>) -> Result {
    let mut rock_transform = rock.single_mut()?;
    rock_transform.rotate_y(time.delta_secs());

    Ok(())
}
