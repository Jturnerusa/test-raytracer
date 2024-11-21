use std::ops::Range;

use nalgebra::Vector3;

use crate::{color::Color, material::Material, ray::Ray};

pub trait Hit {
    fn hit(&self, ray: Ray, interval: Range<f64>) -> Option<Record>;
}

#[derive(Clone, Copy, Debug)]
pub enum Record {
    Light {
        color: Color,
        power: f64,
        t: f64,
    },
    Object {
        normal: Vector3<f64>,
        material: Material,
        t: f64,
    },
}

impl Record {
    pub fn t(&self) -> f64 {
        match self {
            Self::Light { t, .. } => *t,
            Self::Object { t, .. } => *t,
        }
    }
}
