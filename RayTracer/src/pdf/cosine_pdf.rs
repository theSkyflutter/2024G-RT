use super::Pdf;
use crate::{onb::Onb, vec3::Vec3};
use std::f64::consts::PI;

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        let mut uvw = Onb::new();
        uvw.build_from_w(w);
        Self { uvw }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine_theta = direction.unit() * self.uvw.w();
        (cosine_theta / PI).max(0.0)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.local_with_vec3(&Vec3::random_cosine_direction())
    }
}
