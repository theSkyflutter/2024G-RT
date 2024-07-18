mod base_material;
mod dielectric;
mod diffuse_light;
mod isotropic;
mod lambertian;
mod metal;

pub use base_material::BaseMaterial;
pub use dielectric::Dielectric;
pub use diffuse_light::DiffuseLight;
pub use isotropic::Isotropic;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Point3};

#[allow(unused_variables)]
pub trait Material: Send + Sync {
    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
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

    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }
}
