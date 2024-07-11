use super::Material;
use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    texture::{SolidColor, Texture},
    vec3::Vec3,
};
use std::sync::Arc;

pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    // pub fn new(tex: &Arc<dyn Texture>) -> Self {
    //     Self { tex: tex.clone() }
    // }

    pub fn from_color(albedo: &Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray::new_with_time(&rec.p, &Vec3::random_unit_vector(), r_in.time());
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        true
    }
}
