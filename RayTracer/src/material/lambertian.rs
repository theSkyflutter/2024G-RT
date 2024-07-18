use super::Material;
use crate::{
    color::Color,
    hittable::HitRecord,
    onb::Onb,
    ray::Ray,
    texture::{SolidColor, Texture},
    vec3::Vec3,
};
use std::{f64::consts::PI, sync::Arc};

pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(tex: &Arc<dyn Texture>) -> Self {
        Self { tex: tex.clone() }
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
        let mut uvw = Onb::new();
        uvw.build_from_w(&rec.normal);
        let scatter_direction = uvw.local_with_vec3(&Vec3::random_cosine_direction());

        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        *scattered = Ray::new_with_time(&rec.p, &scatter_direction.unit(), r_in.time());
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let mut uvw = Onb::new();
        uvw.build_from_w(&rec.normal);
        uvw.w() * *scattered.direction() / PI
    }
}
