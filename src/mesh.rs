use std::ops::Range;

use nalgebra::{Point3, Transform3};
use rand::{rngs::OsRng, Rng};
use rerun::transform;

use crate::{
    hit::{self, Hit},
    material::Material,
    ray::Ray,
};

#[derive(Clone, Debug)]
pub struct Mesh {
    pub verts: Vec<Point3<f64>>,
    pub faces: Vec<[usize; 3]>,
    pub material: Material,
    pub transform: Transform3<f64>,
}

impl Mesh {
    pub fn rec(&self, rec: &rerun::RecordingStream) {
        rec.log(
            format!("mesh-{}", OsRng.gen::<u32>()),
            &rerun::Mesh3D::new(self.verts.iter().map(|vert| {
                let transformed = self.transform.transform_point(vert);
                [
                    transformed.x as f32,
                    transformed.y as f32,
                    transformed.z as f32,
                ]
            }))
            .with_triangle_indices(
                self.faces
                    .iter()
                    .map(|[a, b, c]| [*a as u32, *b as u32, *c as u32]),
            ),
        )
        .unwrap()
    }
}

impl Hit for Mesh {
    fn hit(&self, ray: Ray, interval: Range<f64>) -> Option<crate::hit::Record> {
        let mut interval = interval;
        let mut closest = None;

        let mesh_ray = Ray {
            origin: self
                .transform
                .try_inverse()
                .unwrap()
                .transform_point(&ray.origin),
            direction: self
                .transform
                .try_inverse()
                .unwrap()
                .transform_vector(&ray.direction),
        };

        for [a, b, c] in &self.faces {
            let v0 = self.verts[*a];
            let v1 = self.verts[*b];
            let v2 = self.verts[*c];

            let v0v1 = v1 - v0;
            let v0v2 = v2 - v0;
            let n = v0v1.cross(&v0v2);

            if n.dot(&ray.direction).abs() < f64::EPSILON {
                continue;
            }

            let d = -n.dot(&v0.coords);
            let t = -(n.dot(&mesh_ray.origin.coords) + d) / n.dot(&mesh_ray.direction);

            if !interval.contains(&t) {
                continue;
            }

            let p = mesh_ray.at(t);

            let v0p = p - v0;
            let ne = v0v1.cross(&v0p);
            if n.dot(&ne) < 0.0 {
                continue;
            }

            let v2v1 = v2 - v1;
            let v1p = p - v1;
            let ne = v2v1.cross(&v1p);
            if n.dot(&ne) < 0.0 {
                continue;
            }

            let v2v0 = v0 - v2;
            let v2p = p - v2;
            let ne = v2v0.cross(&v2p);
            if n.dot(&ne) < 0.0 {
                continue;
            }

            closest = Some((t, n));
            interval.end = t;
        }

        closest.map(|(t, normal)| {
            let mesh_p = mesh_ray.at(t);
            let world_p = self.transform.transform_point(&mesh_p);
            let world_t = (world_p - ray.origin).magnitude();
            let world_normal = self.transform.transform_vector(&normal);
            hit::Record::Object {
                t: world_t,
                normal: world_normal,
                material: self.material,
            }
        })
    }
}
