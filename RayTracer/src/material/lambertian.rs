use super::Material;
use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    texture::{SolidColor, Texture},
    vec3::Vec3,
};
use std::sync::Arc;

pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(tex: &Arc<dyn Texture>) -> Self {
        Self {
            tex: tex.clone(),
        }
    }

    pub fn from_color(albedo: &Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new_with_time(&rec.p, &scatter_direction, r_in.time());
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        true
    }
}
