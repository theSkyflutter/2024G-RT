use super::Material;
use crate::{color::Color, hittable::HitRecord, ray::Ray, rtweekend};

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::ones();
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction().unit();
        let cos_theta = (-unit_direction * rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let direction;
        if ri * sin_theta > 1.0 || reflectance(cos_theta, ri) > rtweekend::random_double() {
            direction = unit_direction.reflect(&rec.normal);
        } else {
            direction = unit_direction.refract(&rec.normal, ri);
        }

        *scattered = Ray::new_with_time(&rec.p, &direction, r_in.time());
        true
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}
