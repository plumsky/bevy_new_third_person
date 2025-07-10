use asset_loading::*;
use avian3d::prelude::*;
use bevy::{gltf::GltfMesh, prelude::*};
use bevy_skein::SkeinPlugin;
use models::*;

mod skybox;

pub use skybox::*;

/// This plugin handles loading and saving scenes
/// Scene logic is only active during the State `Screen::Playing`
pub fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default(),
        SkeinPlugin::default(),
        bevy_fix_gltf_coordinate_system::FixGltfCoordinateSystemPlugin,
        skybox::plugin,
    ))
    .add_systems(OnEnter(Screen::Title), setup);
}

pub fn setup(
    cfg: Res<Config>,
    meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<StandardMaterial>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    models: Res<Models>,
    gltf_assets: Res<Assets<Gltf>>,
    mut commands: Commands,
) {
    // let Some(scene) = gltf_assets.get(&models.scene) else {
    //     return;
    // };
    // commands.spawn(SceneRoot(scene.scenes[0].clone()));

    let Some(rock) = gltf_assets.get(&models.rock) else {
        return;
    };
    let Some(comb_sphere) = gltf_assets.get(&models.comb_sphere) else {
        return;
    };
    let Some(caged_matter) = gltf_assets.get(&models.caged_matter) else {
        return;
    };

    // Example how to extract and spawn meshes
    let mesh = rock.meshes[0].clone();
    let material = rock.materials[0].clone();
    if let Some(mesh) = gltf_meshes.get(&mesh) {
        for primitive in &mesh.primitives {
            let mut transform = Transform::from_translation(Vec3::new(-50.0, 9.0, 5.0));
            transform.scale = Vec3::splat(3.0);
            let mesh = primitive.mesh.clone();
            let mut e = commands.spawn((
                StateScoped(Screen::Gameplay),
                Rock,
                transform,
                Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                RigidBody::Static,
            ));

            if let Some(mesh) = meshes.get(&mesh) {
                e.insert(
                    Collider::trimesh_from_mesh(mesh)
                        .expect("failed to create collider from rock mesh"),
                );
            }
        }
    }

    let geom = cfg.geom.clone();
    let plane = geom.main_plane;
    let size = plane / 2.0;
    for i in 0..geom.quantity {
        let i = i as f32;
        let (low, upper) = (plane / 100.0, plane / 40.0);
        let step = (upper - low) / geom.quantity as f32;

        let y_size = low + step * i;
        let x_size = low + step * i;
        let (x, y, z) = (
            -size / 4.0 + i * x_size, // + step * 20.0,
            y_size / 2.0 + i * step,
            -size / 4.0,
        );
        // let mut pos = Transform::from_xyz(x, y, z);
        // let (mesh, color) = if i % 2.0 == 0.0 {
        // (Mesh::from(Cuboid::new(x_size, y_size, x_size)), GREEN)
        // }else{
        // pos.translation.z += size / 2.0;
        // (Mesh::from(Sphere::new(y_size)), LIGHT_BLUE)
        // let material = materials.add(StandardMaterial {
        //     base_color: color,
        //     #[cfg(feature = "enhanced")]
        //     specular_tint: Color::WHITE,
        //     ..default()
        // });

        // let mesh3d = Mesh3d(meshes.add(mesh.clone()));
        // let mat = MeshMaterial3d(material.clone());
        // commands.spawn((
        //     StateScoped(Screen::Gameplay),
        //     mat,
        //     pos,
        //     mesh,
        //     RigidBody::Static,
        //     Collider::trimesh_from_mesh(&mesh).expect("failed to create collider for mesh"),
        // ));
        let pos = Transform::from_xyz(x, y, z);
        let mut bundle = (StateScoped(Screen::Gameplay), pos);
        if i % 2.0 == 0.0 {
            commands.spawn_colliding_mesh(caged_matter, &meshes, &gltf_meshes, bundle);
        } else {
            bundle.1.translation.z += size / 2.0;
            commands.spawn_colliding_mesh(comb_sphere, &meshes, &gltf_meshes, bundle);
        };
    }

    // TODO: add spatial boombox object
    // soundtrack boombox
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
