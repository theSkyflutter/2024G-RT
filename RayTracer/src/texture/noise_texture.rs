mod perlin {
    use crate::{
        rtweekend,
        vec3::{Point3, Vec3},
    };

    pub(super) struct Perlin {
        randvec: [Vec3; Self::POINT_COUNT],
        perm_x: [usize; Self::POINT_COUNT],
        perm_y: [usize; Self::POINT_COUNT],
        perm_z: [usize; Self::POINT_COUNT],
    }

    impl Perlin {
        const POINT_COUNT: usize = 256;

        pub(super) fn new() -> Self {
            let mut randvec = [Vec3::default(); Self::POINT_COUNT];
            for f in &mut randvec {
                *f = Vec3::random_in_range(-1.0, 1.0).unit();
            }

            let perm_x = Self::perlin_generate_perm();
            let perm_y = Self::perlin_generate_perm();
            let perm_z = Self::perlin_generate_perm();

            Self {
                randvec,
                perm_x,
                perm_y,
                perm_z,
            }
        }

        fn noise(&self, p: &Point3) -> f64 {
            let u = p.x - p.x.floor();
            let v = p.y - p.y.floor();
            let w = p.z - p.z.floor();

            let i = p.x.floor() as i32;
            let j = p.y.floor() as i32;
            let k = p.z.floor() as i32;
            let mut c = [[[Vec3::default(); 2]; 2]; 2];

            for di in 0..2 {
                for dj in 0..2 {
                    for dk in 0..2 {
                        c[di][dj][dk] = self.randvec[self.perm_x[(i + di as i32 & 255) as usize]
                            ^ self.perm_y[(j + dj as i32 & 255) as usize]
                            ^ self.perm_z[(k + dk as i32 & 255) as usize]];
                    }
                }
            }

            perlin_interp(&c, u, v, w)
        }

        pub(super) fn turb(&self, p: &Point3, depth: u32) -> f64 {
            let mut accum = 0.0;
            let mut temp_p = *p;
            let mut weight = 1.0;

            for _ in 0..depth {
                accum += weight * self.noise(&temp_p);
                weight *= 0.5;
                temp_p *= 2.0;
            }

            accum.abs()
        }

        fn perlin_generate_perm() -> [usize; Self::POINT_COUNT] {
            let mut p = [0; Self::POINT_COUNT];
            for i in 0..Self::POINT_COUNT {
                p[i] = i;
            }
            Self::permute(&mut p);
            p
        }

        fn permute(p: &mut [usize; Self::POINT_COUNT]) {
            for i in (1..Self::POINT_COUNT).rev() {
                let target = rtweekend::random_int_in_range(0, i as i32) as usize;
                let tmp = p[i];
                p[i] = p[target];
                p[target] = tmp;
            }
        }
    }

    fn perlin_interp(c: &[[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += c[i][j][k]
                        * weight_v
                        * (i as f64 * uu + (1 - i) as f64 * (1.0 - uu))
                        * (j as f64 * vv + (1 - j) as f64 * (1.0 - vv))
                        * (k as f64 * ww + (1 - k) as f64 * (1.0 - ww));
                }
            }
        }

        accum
    }
}

use super::Texture;
use crate::{color::Color, vec3::Point3};
use perlin::Perlin;

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        Color::new(0.5, 0.5, 0.5) * (1.0 + (self.scale * p.z + 10.0 * self.noise.turb(p, 7)).sin())
    }
}
