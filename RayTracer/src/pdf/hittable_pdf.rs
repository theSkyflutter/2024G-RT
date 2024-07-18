use super::Pdf;
use crate::{
    hittable::Hittable,
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

pub struct HittablePdf {
    objects: Arc<dyn Hittable>,
    origin: Point3,
}

impl HittablePdf {
    pub fn new(objects: &Arc<dyn Hittable>, origin: &Point3) -> Self {
        Self {
            objects: objects.clone(),
            origin: *origin,
        }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}
