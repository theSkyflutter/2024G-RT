use super::{HitRecord, Hittable};
use crate::{
    aabb::Aabb,
    interval::Interval,
    material::Material,
    vec3::{Point3, Vec3},
};
use std::{f64::consts::PI, sync::Arc};

pub struct Sphere {
    center1: Point3,
    raduis: f64,
    mat: Arc<dyn Material>,
    is_moving: bool,
    center_vec: Vec3,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(center: &Point3, raduis: f64, mat: &Arc<dyn Material>) -> Self {
        let rvec = Vec3::new(raduis, raduis, raduis);
        Self {
            center1: *center,
            raduis: if raduis > 0.0 { raduis } else { 0.0 },
            mat: mat.clone(),
            is_moving: false,
            center_vec: Vec3::default(),
            bbox: Aabb::from_endpoints(&(*center - rvec), &(*center + rvec)),
        }
    }

    pub fn new_moving(
        center1: &Point3,
        center2: &Point3,
        raduis: f64,
        mat: &Arc<dyn Material>,
    ) -> Self {
        let rvec = Vec3::new(raduis, raduis, raduis);
        let box1 = Aabb::from_endpoints(&(*center1 - rvec), &(*center1 + rvec));
        let box2 = Aabb::from_endpoints(&(*center2 - rvec), &(*center2 + rvec));
        Self {
            center1: *center1,
            raduis: if raduis > 0.0 { raduis } else { 0.0 },
            mat: mat.clone(),
            is_moving: true,
            center_vec: *center2 - *center1,
            bbox: Aabb::from_aabbs(&box1, &box2),
        }
    }

    fn sphere_center(&self, time: f64) -> Point3 {
        self.center1 + self.center_vec * time
    }
}

impl Hittable for Sphere {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn hit(&self, r: &crate::ray::Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let center = if self.is_moving {
            self.sphere_center(r.time())
        } else {
            self.center1
        };
        let oc = center - *r.origin();
        let a = r.direction().squared_length();
        let h = *r.direction() * oc;
        let c = oc.squared_length() - self.raduis * self.raduis;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(root);
        let outward_normal = (rec.p - center) / self.raduis;
        rec.set_face_normal(r, &outward_normal);
        get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat = Some(self.mat.clone());

        true
    }
}

fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
    let theta = (-p.y).acos();
    let phi = (-p.z).atan2(p.x) + PI;

    *u = phi / (2.0 * PI);
    *v = theta / PI;
}
