use super::{HitRecord, Hittable};
use crate::{aabb::Aabb, interval::Interval, ray::Ray, vec3::Vec3};
use std::sync::Arc;

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: &Arc<dyn Hittable>, offset: &Vec3) -> Self {
        let bbox = object.bounding_box() + *offset;
        Self {
            object: object.clone(),
            offset: *offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let offset_r = Ray::new_with_time(&(*r.origin() - self.offset), r.direction(), r.time());

        if self.object.hit(&offset_r, ray_t, rec) {
            rec.p += self.offset;
            true
        } else {
            false
        }
    }
}
