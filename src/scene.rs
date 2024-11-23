use nalgebra::{Rotation3, Transform3, Vector3};

use crate::{camera::Camera, color::Color, light::Light, material::Material, mesh::Mesh};

#[derive(Clone, Debug)]
pub struct Scene {
    pub meshes: Vec<Mesh>,
    pub lights: Vec<Light>,
    pub camera: Camera,
    pub background: Color,
}

impl Scene {
    pub fn from_deserialized(scene: &crate::serialize::Scene) -> Self {
        Self {
            meshes: scene
                .objects
                .iter()
                .map(|object| Mesh {
                    verts: scene.meshes[object.mesh.as_str()].verts.clone(),
                    faces: scene.meshes[object.mesh.as_str()].faces.clone(),
                    material: Material::from_deserialized(
                        scene.materials[object.material.as_str()],
                    ),
                    transform: object.transform,
                })
                .collect::<Vec<_>>(),
            lights: scene
                .lights
                .iter()
                .map(|light| Light::from_deserialized(*light))
                .collect(),
            camera: Camera::from_deserialized(&scene.camera),
            background: scene.background,
        }
    }

    pub fn rec(&self, rec: &mut rerun::RecordingStream) {
        for light in &self.lights {
            light.rec(rec);
        }

        for mesh in &self.meshes {
            mesh.rec(rec);
        }
    }
}
