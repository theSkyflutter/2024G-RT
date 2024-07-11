mod bvh;
mod constant_medium;
mod hittable_list;
mod quad;
mod rotate_y;
mod sphere;
mod translate;

pub use bvh::BvhNode;
pub use constant_medium::ConstantMedium;
pub use hittable_list::HittableList;
pub use quad::{get_box, Quad};
pub use rotate_y::RotateY;
pub use sphere::Sphere;
pub use translate::Translate;

use crate::{
    aabb::Aabb,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> Aabb;
}

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Arc<dyn Material>>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = *r.direction() * *outward_normal < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}
