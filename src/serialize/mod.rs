use std::collections::HashMap;

use nalgebra::{Matrix4, Point3, Transform3};
use serde::Deserialize;

use crate::color::Color;

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Light {
    Area {
        size: f64,
        power: f64,
        transform: Transform3<f64>,
        color: Color,
    },
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Material {
    Diffuse { color: Color, adsorption: f64 },
    Metal { color: Color, roughness: f64 },
    Glass { color: Color, ior: f64 },
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Camera {
    pub fov: f64,
    pub zfar: f64,
    pub znear: f64,
    pub aspect_ratio: f64,
    pub transform: Transform3<f64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Mesh {
    pub verts: Vec<Point3<f64>>,
    pub faces: Vec<[usize; 3]>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Object {
    pub mesh: String,
    pub material: String,
    pub transform: Transform3<f64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Scene {
    pub background: Color,
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub meshes: HashMap<String, Mesh>,
    pub materials: HashMap<String, Material>,
    pub objects: Vec<Object>,
}

pub fn flip_y(m: Matrix4<f64>) -> Matrix4<f64> {
    m * Matrix4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    )
}
