mod checker_texture;
mod image_texture;
mod noise_texture;
mod solid_color;

pub use checker_texture::CheckerTexture;
pub use image_texture::ImageTexture;
pub use noise_texture::NoiseTexture;
pub use solid_color::SolidColor;

use crate::{color::Color, vec3::Point3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
