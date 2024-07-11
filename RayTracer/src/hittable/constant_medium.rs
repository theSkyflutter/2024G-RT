use super::{HitRecord, Hittable};
use crate::{
    color::Color,
    interval::Interval,
    material::{Isotropic, Material},
    ray::Ray,
    rtweekend,
    vec3::Vec3,
};
use std::{f64::INFINITY, sync::Arc};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    // fn new_with_texture(
    //     boundary: &Arc<dyn Hittable>,
    //     density: f64,
    //     tex: &Arc<dyn Texture>,
    // ) -> Self {
    //     Self {
    //         boundary: boundary.clone(),
    //         neg_inv_density: -1.0 / density,
    //         phase_function: Arc::new(Isotropic::new(tex)),
    //     }
    // }

    pub fn new_with_color(boundary: &Arc<dyn Hittable>, density: f64, albedo: &Color) -> Self {
        Self {
            boundary: boundary.clone(),
            neg_inv_density: -1.0 / density,
            phase_function: Arc::new(Isotropic::from_color(albedo)),
        }
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self) -> crate::aabb::Aabb {
        self.boundary.bounding_box()
    }

    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self.boundary.hit(r, &Interval::UNIVERSE, &mut rec1)
            || !self
                .boundary
                .hit(r, &Interval::new(rec1.t + 0.0001, INFINITY), &mut rec2)
        {
            return false;
        }

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }
        if rec2.t > ray_t.max {
            rec2.t = ray_t.max
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < 0.0 {
            rec1.t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rtweekend::random_double().log2();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);
        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat = Some(self.phase_function.clone());

        true
    }
}
