use std::{
    error::Error,
    io::Write,
    ops::{Div, Mul},
};

use nalgebra::Vector4;

pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 0.0);
pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 0.0);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, serde::Deserialize)]
pub struct Color(Vector4<f64>);

impl Color {
    pub const fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self(Vector4::new(r, g, b, a))
    }

    pub fn into_rgba8888(self) -> [u8; 4] {
        [
            (self.0.x * 255.0) as u8,
            (self.0.y * 255.0) as u8,
            (self.0.z * 255.0) as u8,
            (self.0.w * 255.0) as u8,
        ]
    }
}

impl From<Vector4<f64>> for Color {
    fn from(value: Vector4<f64>) -> Self {
        Self(value)
    }
}

impl From<Color> for Vector4<f64> {
    fn from(value: Color) -> Self {
        value.0
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0.component_mul(&rhs.0))
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div for Color {
    type Output = Color;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0.component_div(&rhs.0))
    }
}

pub fn write_ppm<W: Write>(
    width: usize,
    height: usize,
    pixels: &[Color],
    mut writer: W,
) -> Result<(), Box<dyn Error>> {
    writeln!(writer, "P3")?;
    writeln!(writer, "{width} {height}")?;
    writeln!(writer, "255")?;

    for pixel in pixels {
        let [r, g, b, _] = pixel.into_rgba8888();

        writeln!(writer, "{r} {g} {b}")?;
    }

    Ok(())
}
