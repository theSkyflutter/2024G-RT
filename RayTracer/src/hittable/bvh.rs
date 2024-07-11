use super::{hittable_list::HittableList, HitRecord, Hittable};
use crate::{aabb::Aabb, interval::Interval, ray::Ray};
use std::{cmp::Ordering, sync::Arc};

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    fn from_objects(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bbox = Aabb::EMPTY;
        for object_index in start..end {
            bbox = Aabb::from_aabbs(&bbox, &objects[object_index].bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };

        let object_span = end - start;
        let left;
        let right;

        if object_span == 1 {
            left = objects[start].clone();
            right = left.clone();
        } else if object_span == 2 {
            left = objects[start].clone();
            right = objects[start + 1].clone();
        } else {
            objects[start..end].sort_unstable_by(comparator);

            let mid = start + object_span / 2;
            left = Arc::new(BvhNode::from_objects(objects, start, mid));
            right = Arc::new(BvhNode::from_objects(objects, mid, end));
        }

        Self { left, right, bbox }
    }

    pub fn from_hittable_list(list: &mut HittableList) -> Self {
        let len = list.objects.len();
        Self::from_objects(&mut list.objects, 0, len)
    }
}

impl Hittable for BvhNode {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        if self.bbox.hit(r, *ray_t) {
            let hit_left = self.left.hit(r, ray_t, rec);
            let hit_right = self.right.hit(
                r,
                &Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max }),
                rec,
            );

            hit_left || hit_right
        } else {
            false
        }
    }
}

fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: u8) -> Ordering {
    let a_bounding_box = a.bounding_box();
    let b_bounding_box = b.bounding_box();
    let a_axis_interval = a_bounding_box.axis_interval(axis_index);
    let b_axis_interval = b_bounding_box.axis_interval(axis_index);
    if a_axis_interval.min < b_axis_interval.min {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> Ordering {
    box_compare(a, b, 2)
}
