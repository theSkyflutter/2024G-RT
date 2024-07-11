use super::{solid_color::SolidColor, Texture};
use crate::{color::Color, vec3::Point3};
use std::sync::Arc;

pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    // fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
    //     Self {
    //         inv_scale: 1.0 / scale,
    //         even: even.clone(),
    //         odd: odd.clone(),
    //     }
    // }

    pub fn from_colors(scale: f64, c1: &Color, c2: &Color) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even: Arc::new(SolidColor::new(c1)),
            odd: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x_integer = (self.inv_scale * p.x).floor() as i64;
        let y_integer = (self.inv_scale * p.y).floor() as i64;
        let z_integer = (self.inv_scale * p.z).floor() as i64;

        if (x_integer + y_integer + z_integer) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
