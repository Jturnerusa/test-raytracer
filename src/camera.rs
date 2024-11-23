use nalgebra::{Perspective3, Point2, Point3, Rotation3, Transform3, Vector3};
use rerun::{Rotation3D, Vector3D};

use crate::{ray::Ray, serialize};

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub perspective: Perspective3<f64>,
    pub transform: Transform3<f64>,
}

impl Camera {
    pub fn cast_ray(&self, clip: Point2<f64>, rec: &mut rerun::RecordingStream) -> Ray {
        let camera = self
            .perspective
            .unproject_point(&Point3::new(clip.x, -clip.y, 0.0));
        let origin = self.transform.transform_point(&Point3::origin());
        let direction = (self.transform.transform_point(&camera) - origin).normalize();

        rec.log(
            "ray",
            &rerun::Arrows3D::from_vectors([[
                direction.x as f32,
                direction.y as f32,
                direction.z as f32,
            ]])
            .with_origins([[origin.x as f32, origin.y as f32, origin.z as f32]]),
        )
        .unwrap();

        Ray { origin, direction }
    }

    pub fn from_deserialized(camera: &crate::serialize::Camera) -> Self {
        Self {
            perspective: Perspective3::new(
                camera.aspect_ratio,
                camera.fov,
                camera.znear,
                camera.zfar,
            ),
            transform: camera.transform,
        }
    }
}
