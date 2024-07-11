mod dielectric;
mod diffuse_light;
mod isotropic;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use isotropic::Isotropic;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Point3};

#[allow(unused_variables)]
pub trait Material: Send + Sync {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        Color::zeros()
    }

    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false
    }
}
