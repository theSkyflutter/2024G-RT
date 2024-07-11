use super::Material;
use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Color, fuzz: f64) -> Self {
        Self {
            albedo: *albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected =
            r_in.direction().reflect(&rec.normal).unit() + Vec3::random_unit_vector() * self.fuzz;
        *scattered = Ray::new_with_time(&rec.p, &reflected, r_in.time());
        *attenuation = self.albedo;
        *scattered.direction() * rec.normal > 0.0
    }
}
