use std::ops::Range;

use nalgebra::{Point3, Rotation3, Transform3, Vector3};
use rand::{rngs::OsRng, Rng};

use crate::{
    color::Color,
    hit::{Hit, Record},
    ray::Ray,
};

#[derive(Clone, Copy, Debug)]
pub enum Light {
    Area {
        size: f64,
        power: f64,
        color: Color,
        transform: Transform3<f64>,
    },
}

impl Light {
    pub fn transform(self) -> Transform3<f64> {
        match self {
            Self::Area { transform, .. } => transform,
        }
    }

    pub fn from_deserialized(light: crate::serialize::Light) -> Self {
        match light {
            crate::serialize::Light::Area {
                size,
                power,
                transform,
                color,
            } => Light::Area {
                size,
                power,
                transform: Transform3::from_matrix_unchecked(transform.into_inner().transpose()),
                color,
            },
        }
    }

    pub fn rec(&self, rec: &mut rerun::RecordingStream) {
        match self {
            Self::Area {
                size, transform, ..
            } => {
                let q = transform.transform_point(&Point3::new(-(size / 2.0), -(size / 2.0), 0.0));
                let u =
                    transform.transform_point(&(Point3::new(size / 2.0, -(size / 2.0), 0.0))) - q;
                let v =
                    transform.transform_point(&(Point3::new(-(size / 2.0), size / 2.0, 0.0))) - q;

                rec.log(
                    format!("light {}", OsRng.gen::<u64>()),
                    &rerun::Arrows3D::from_vectors([
                        [u.x as f32, u.y as f32, u.z as f32],
                        [v.x as f32, v.y as f32, v.z as f32],
                    ])
                    .with_origins([
                        [q.x as f32, q.y as f32, q.z as f32],
                        [q.x as f32, q.y as f32, q.z as f32],
                    ]),
                )
                .unwrap();
            }
        }
    }
}

impl Hit for Light {
    fn hit(&self, ray: Ray, interval: Range<f64>) -> Option<crate::hit::Record> {
        let light_ray = Ray {
            origin: self.transform().transform_point(&ray.origin),
            direction: self.transform().transform_vector(&ray.direction),
        };

        match self {
            Self::Area {
                size, power, color, ..
            } => {
                let q = Point3::new(-(size / 2.0), 0.0, -(size / 2.0));
                let u = Point3::new(size / 2.0, 0.0, -(size / 2.0)) - q;
                let v = Point3::new(-(size / 2.0), 0.0, size / 2.0) - q;

                quad_ray_intersection(q, u, v, light_ray, interval).map(|light_t| {
                    let light_p = light_ray.at(light_t);
                    let world_p = self
                        .transform()
                        .try_inverse()
                        .unwrap()
                        .transform_point(&light_p);
                    let world_t = (ray.origin - world_p).magnitude_squared();

                    Record::Light {
                        t: world_t,
                        color: *color,
                        power: *power,
                    }
                })
            }
        }
    }
}

fn quad_ray_intersection(
    q: Point3<f64>,
    u: Vector3<f64>,
    v: Vector3<f64>,
    ray: Ray,
    interval: Range<f64>,
) -> Option<f64> {
    let n = u.cross(&v);
    let w = n / n.dot(&n);
    let normal = n.normalize();
    let d = normal.dot(&q.coords);
    let denom = normal.dot(&ray.direction);

    if denom.abs() < f64::EPSILON {
        return None;
    }

    let t = (d - normal.dot(&ray.origin.coords)) / denom;

    if !interval.contains(&t) {
        return None;
    }

    let intersection = ray.at(t);
    let p = intersection - q;
    let a = w.dot(&p.cross(&v));
    let b = w.dot(&u.cross(&p));

    if !(0.0..1.0).contains(&a) || !(0.0..1.0).contains(&b) {
        None
    } else {
        Some(t)
    }
}
