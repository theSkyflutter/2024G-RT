use super::Material;
use crate::{
    color::Color,
    texture::{SolidColor, Texture},
    vec3::Point3,
};
use std::sync::Arc;

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    // pub fn new(tex: &Arc<dyn Texture>) -> Self {
    //     Self {
    //         tex: tex.clone(),
    //     }
    // }

    pub fn from_color(emit: &Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(emit)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.tex.value(u, v, p)
    }
}
