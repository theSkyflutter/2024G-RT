use super::Texture;
use crate::{color::Color, interval::Interval, rtw_image::RtwImage, vec3::Point3};

pub struct ImageTexture {
    image: RtwImage,
}

impl ImageTexture {
    pub fn new(filename: &str) -> Self {
        Self {
            image: RtwImage::open(filename),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, mut u: f64, mut v: f64, _p: &Point3) -> Color {
        if self.image.height() == 0 {
            Color::new(0.0, 1.0, 1.0)
        } else {
            u = Interval::new(0.0, 1.0).clamp(u);
            v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

            let i = (u * self.image.width() as f64) as u32;
            let j = (v * self.image.height() as f64) as u32;
            let pixel = self.image.pixel_data(i, j);

            let color_scale = 1.0 / 255.0;
            Color::new(
                color_scale * pixel[0] as f64,
                color_scale * pixel[1] as f64,
                color_scale * pixel[2] as f64,
            )
        }
    }
}
