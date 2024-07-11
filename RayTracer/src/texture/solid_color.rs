use super::Texture;
use crate::{color::Color, vec3::Point3};

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: &Color) -> Self {
        Self { albedo: *albedo }
    }

    // fn from_rgb(red: f64, green: f64, blue: f64) -> Self {
    //     Self::new(&Color::new(red, green, blue))
    // }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}
