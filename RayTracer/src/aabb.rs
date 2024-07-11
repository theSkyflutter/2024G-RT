use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::ops::Add;

#[derive(Clone, Copy, Default)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    fn new(x: &Interval, y: &Interval, z: &Interval) -> Self {
        Self::pad_to_minimums(x, y, z)
    }

    pub fn from_endpoints(a: &Point3, b: &Point3) -> Self {
        let x = if a.x <= b.x {
            Interval::new(a.x, b.x)
        } else {
            Interval::new(b.x, a.x)
        };
        let y = if a.y <= b.y {
            Interval::new(a.y, b.y)
        } else {
            Interval::new(b.y, a.y)
        };
        let z = if a.z <= b.z {
            Interval::new(a.z, b.z)
        } else {
            Interval::new(b.z, a.z)
        };

        Self::pad_to_minimums(&x, &y, &z)
    }

    fn pad_to_minimums(x: &Interval, y: &Interval, z: &Interval) -> Self {
        let delta = 0.0001;

        Self {
            x: if x.size() < delta {
                x.expand(delta)
            } else {
                *x
            },
            y: if y.size() < delta {
                y.expand(delta)
            } else {
                *y
            },
            z: if z.size() < delta {
                z.expand(delta)
            } else {
                *z
            },
        }
    }

    pub fn from_aabbs(box0: &Self, box1: &Self) -> Self {
        Self {
            x: Interval::from_intervals(&box0.x, &box1.x),
            y: Interval::from_intervals(&box0.y, &box1.y),
            z: Interval::from_intervals(&box0.z, &box1.z),
        }
    }

    pub fn axis_interval(&self, n: u8) -> &Interval {
        if n == 1 {
            &self.y
        } else if n == 2 {
            &self.z
        } else {
            &self.x
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> u8 {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                0
            } else {
                2
            }
        } else {
            if self.y.size() > self.z.size() {
                1
            } else {
                2
            }
        }
    }

    pub const EMPTY: Self = Self {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };
    // const UNIVERSE: Self = Self::new(
    //     &Interval::UNIVERSE,
    //     &Interval::UNIVERSE,
    //     &Interval::UNIVERSE,
    // );
}

impl Add<Vec3> for Aabb {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Aabb::new(&(self.x + rhs.x), &(self.x + rhs.y), &(self.z + rhs.z))
    }
}
