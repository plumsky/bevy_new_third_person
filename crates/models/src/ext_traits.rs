use super::*;
use avian3d::prelude::*;
use bevy::gltf::GltfMesh;

pub trait SpawnCollidingMesh {
    fn spawn_colliding_mesh(
        &mut self,
        gltf: &Gltf,
        meshes: &ResMut<Assets<Mesh>>,
        gltf_meshes: &Res<Assets<GltfMesh>>,
        bundle: impl Bundle + Clone,
    );
}

impl SpawnCollidingMesh for Commands<'_, '_> {
    fn spawn_colliding_mesh(
        &mut self,
        gltf: &Gltf,
        meshes: &ResMut<Assets<Mesh>>,
        gltf_meshes: &Res<Assets<GltfMesh>>,
        bundle: impl Bundle + Clone,
    ) {
        info!(
            "meshes: {}, materials: {}",
            gltf.meshes.len(),
            gltf.materials.len()
        );
        let mesh = gltf.meshes[0].clone();
        let material = gltf.materials[0].clone();
        if let Some(mesh) = gltf_meshes.get(&mesh) {
            for primitive in &mesh.primitives {
                let mesh = primitive.mesh.clone();
                let mut e = self.spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(material.clone()),
                    RigidBody::Static,
                    bundle.clone(),
                ));

                if let Some(mesh) = meshes.get(&mesh) {
                    e.insert(
                        Collider::trimesh_from_mesh(mesh)
                            .expect("failed to create collider from rock mesh"),
                    );
                }
            }
        }
    }
}
