use nalgebra::{Point3, Vector3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn at(self, t: f64) -> Point3<f64> {
        self.origin + t * self.direction
    }
}
