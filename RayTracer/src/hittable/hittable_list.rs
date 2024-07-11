use super::{HitRecord, Hittable};
use crate::{aabb::Aabb, interval::Interval, ray::Ray};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl HittableList {
    pub fn new(object: &Arc<dyn Hittable>) -> Self {
        let mut hittable_list = Self::default();
        hittable_list.add(object);
        hittable_list
    }

    // pub fn clear(&mut self) {
    //     self.objects.clear();
    // }

    pub fn add(&mut self, object: &Arc<dyn Hittable>) {
        self.objects.push(object.clone());
        self.bbox = Aabb::from_aabbs(&self.bbox, &object.bounding_box());
    }
}

impl Hittable for HittableList {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, &Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
}
