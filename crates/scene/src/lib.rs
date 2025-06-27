use asset_loading::*;
use avian3d::prelude::*;
use bevy::gltf::GltfMesh;
use bevy::prelude::*;
use models::*;

mod skybox;

pub use skybox::*;

/// This plugin handles loading and saving scenes
/// Scene logic is only active during the State `Screen::Playing`
pub fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsPlugins::default(),
        bevy_fix_gltf_coordinate_system::FixGltfCoordinateSystemPlugin,
        skybox::plugin,
    ))
    .add_systems(OnEnter(Screen::Title), setup);
}

pub fn setup(
    cfg: Res<Config>,
    models: Res<Models>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    gltf_assets: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
) {
    let Some(gltf) = gltf_assets.get(&models.rock) else {
        return;
    };
    let plane = cfg.geom.main_plane;

    // Plane
    let shape = Cuboid::new(plane, 1.0, plane);
    let mesh = meshes.add(shape);
    let mat = MeshMaterial3d(materials.add(SAND_YELLOW));
    commands.spawn((
        StateScoped(Screen::Gameplay),
        mat,
        Mesh3d(mesh),
        Transform::from_xyz(0.0, -1.0, 0.0),
        RigidBody::Static,
        Collider::trimesh_from_mesh(&Mesh::from(shape)).unwrap_or(Collider::half_space(Vec3::Y)),
    ));

    // Rock
    let mesh = gltf.meshes[0].clone();
    let material = gltf.materials[0].clone();
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

    let size = plane / 2.0;
    let geom = cfg.geom.clone();
    for i in 0..geom.quantity {
        let i = i as f32;
        let (low, upper) = (plane / 100.0, plane / 40.0);
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
