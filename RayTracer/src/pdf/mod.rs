mod cosine_pdf;
mod hittable_pdf;
mod sphere_pdf;

pub use cosine_pdf::CosinePdf;
pub use hittable_pdf::HittablePdf;

use crate::vec3::Vec3;

pub trait Pdf {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}
