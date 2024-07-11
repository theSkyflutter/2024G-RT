use super::{HitRecord, Hittable, HittableList};
use crate::{
    aabb::Aabb,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::sync::Arc;

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: &Point3, u: &Vec3, v: &Vec3, mat: &Arc<dyn Material>) -> Self {
        let n = u.cross(&v);
        let normal = n.unit();
        let d = normal * *q;
        let w = n / (n * n);

        let bbox_diagonal1 = Aabb::from_endpoints(q, &(*q + *u + *v));
        let bbox_diagonal2 = Aabb::from_endpoints(&(*q + *u), &(*q + *v));
        let bbox = Aabb::from_aabbs(&bbox_diagonal1, &bbox_diagonal2);

        Self {
            q: *q,
            u: *u,
            v: *v,
            w,
            mat: mat.clone(),
            bbox,
            normal,
            d,
        }
    }
}

impl Hittable for Quad {
    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: &Interval, rec: &mut HitRecord) -> bool {
        let denom = self.normal * *r.direction();

        if denom.abs() < 1e-8 {
            false
        } else {
            let t = (self.d - self.normal * *r.origin()) / denom;
            if !ray_t.contains(t) {
                false
            } else {
                let intersection = r.at(t);
                let planar_hitpt_vector = intersection - self.q;
                let alpha = self.w * planar_hitpt_vector.cross(&self.v);
                let beta = self.w * self.u.cross(&planar_hitpt_vector);

                if !is_interiior(alpha, beta, rec) {
                    false
                } else {
                    rec.t = t;
                    rec.p = intersection;
                    rec.mat = Some(self.mat.clone());
                    rec.set_face_normal(r, &self.normal);

                    true
                }
            }
        }
    }
}

fn is_interiior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
    let unit_interval = Interval::new(0.0, 1.0);

    if !unit_interval.contains(a) || !unit_interval.contains(b) {
        false
    } else {
        rec.u = a;
        rec.v = b;
        true
    }
}

pub fn get_box(a: &Point3, b: &Point3, mat: &Arc<dyn Material>) -> Arc<HittableList> {
    let mut sides = HittableList::default();

    let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    sides.add(
        &(Arc::new(Quad::new(&Point3::new(min.x, min.y, max.z), &dx, &dy, mat))
            as Arc<dyn Hittable>),
    );
    sides.add(
        &(Arc::new(Quad::new(&Point3::new(max.x, min.y, max.z), &-dz, &dy, mat))
            as Arc<dyn Hittable>),
    );
    sides.add(
        &(Arc::new(Quad::new(&Point3::new(max.x, min.y, min.z), &-dx, &dy, mat))
            as Arc<dyn Hittable>),
    );
    sides.add(
        &(Arc::new(Quad::new(&Point3::new(min.x, min.y, min.z), &dz, &dy, mat))
            as Arc<dyn Hittable>),
    );
    sides.add(
        &(Arc::new(Quad::new(&Point3::new(min.x, max.y, max.z), &dx, &-dz, mat))
            as Arc<dyn Hittable>),
    );
    sides.add(
        &(Arc::new(Quad::new(&Point3::new(min.x, min.y, min.z), &dx, &dz, mat))
            as Arc<dyn Hittable>),
    );

    Arc::new(sides)
}
