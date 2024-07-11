use crate::{interval::Interval, vec3::Vec3};
use image::{DynamicImage, GenericImage, Rgba};

pub type Color = Vec3;

impl Color {
    fn to_array(&self) -> [f64; 3] {
        [self.x, self.y, self.z]
    }
}

/// the multi-sample write_color() function
pub fn write_color(pixel_color: &Color, img: &mut DynamicImage, i: u32, j: u32) {
    let pixel_color = pixel_color.to_array();

    static INTENSITY: Interval = Interval::new(0.0, 0.999);
    // Write the translated [0,255] value of each color component.
    img.put_pixel(
        i,
        j,
        Rgba([
            (256 as f64 * INTENSITY.clamp(linear_to_gamma(pixel_color[0]))) as u8,
            (256 as f64 * INTENSITY.clamp(linear_to_gamma(pixel_color[1]))) as u8,
            (256 as f64 * INTENSITY.clamp(linear_to_gamma(pixel_color[2]))) as u8,
            1,
        ]),
    );
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}
