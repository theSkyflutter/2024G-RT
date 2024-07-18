use std::ops::{Index, IndexMut};

use crate::vec3::Vec3;

pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn new() -> Self {
        Self {
            axis: [Vec3::default(); 3],
        }
    }

    // pub fn u(&self) -> Vec3 {
    //     self.axis[0]
    // }

    // pub fn v(&self) -> Vec3 {
    //     self.axis[1]
    // }

    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }

    // pub fn local(&self, a: f64, b: f64, c: f64) -> Vec3 {
    //     self.axis[0] * a + self.axis[1] * b + self.axis[2] * c
    // }

    pub fn local_with_vec3(&self, a: &Vec3) -> Vec3 {
        self.axis[0] * a.x + self.axis[1] * a.y + self.axis[2] * a.z
    }

    pub fn build_from_w(&mut self, w: &Vec3) {
        let unit_w = w.unit();
        let a = if unit_w.x.abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };
        let v = unit_w.cross(&a).unit();
        let u = unit_w.cross(&v);
        self.axis[0] = u;
        self.axis[1] = v;
        self.axis[2] = unit_w;
    }
}

impl Index<u8> for Onb {
    type Output = Vec3;

    fn index(&self, index: u8) -> &Self::Output {
        &self.axis[index as usize]
    }
}

impl IndexMut<u8> for Onb {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        &mut self.axis[index as usize]
    }
}
