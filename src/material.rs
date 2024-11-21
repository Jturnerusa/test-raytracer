use nalgebra::Vector3;

use crate::color::Color;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Diffuse { color: Color, adsorption: f64 },
    Metal { color: Color, roughness: f64 },
}

impl Material {
    pub fn from_deserialized(material: crate::serialize::Material) -> Self {
        match material {
            crate::serialize::Material::Diffuse { color, adsorption } => {
                Self::Diffuse { color, adsorption }
            }
            crate::serialize::Material::Metal { color, roughness } => {
                Self::Metal { color, roughness }
            }
            _ => todo!(),
        }
    }
}
