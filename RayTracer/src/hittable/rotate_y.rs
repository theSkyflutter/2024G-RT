use super::{HitRecord, Hittable};
use crate::{
    aabb::Aabb,
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::{
    f64::{INFINITY, NEG_INFINITY},
    sync::Arc,
};

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: &Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(NEG_INFINITY, NEG_INFINITY, NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Self {
            object: object.clone(),
            sin_theta,
            cos_theta,
            bbox: Aabb::from_endpoints(&min, &max),
        }
    }
}

impl Hittable for RotateY {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut origin = *r.origin();
        let mut direction = *r.direction();

        origin.x = self.cos_theta * r.origin().x - self.sin_theta * r.origin().z;
        origin.z = self.sin_theta * r.origin().x + self.cos_theta * r.origin().z;

        direction.x = self.cos_theta * r.direction().x - self.sin_theta * r.direction().z;
        direction.z = self.sin_theta * r.direction().x + self.cos_theta * r.direction().z;

        let rotated_r = Ray::new_with_time(&origin, &direction, r.time());

        if self.object.hit(&rotated_r, ray_t, rec) {
            let mut p = rec.p;
            p.x = self.cos_theta * rec.p.x + self.sin_theta * rec.p.z;
            p.z = -self.sin_theta * rec.p.x + self.cos_theta * rec.p.z;

            let mut normal = rec.normal;
            normal.x = self.cos_theta * rec.normal.x + self.sin_theta * rec.normal.z;
            normal.z = -self.sin_theta * rec.normal.x + self.cos_theta * rec.normal.z;

            rec.p = p;
            rec.normal = normal;

            true
        } else {
            false
        }
    }
}
