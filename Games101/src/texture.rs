#![allow(warnings)]
use nalgebra::Vector3;

use opencv::core::{MatTraitConst, VecN};
use opencv::imgcodecs::{imread, IMREAD_COLOR};

pub struct Texture
{
    pub img_data: opencv::core::Mat,
    pub width: usize,
    pub height: usize,
}

impl Texture
{
    pub fn new(name: &str) -> Self
    {
        let img_data = imread(name, IMREAD_COLOR).expect("Image reading error!");
        let width = img_data.cols() as usize;
        let height = img_data.rows() as usize;
        Texture {
            img_data,
            width,
            height,
        }
    }

    // pub fn get_color(&self, mut u: f64, mut v: f64) -> Vector3<f64>
    // {
    //     if u < 0.0 {
    //         u = 0.0;
    //     }
    //     if u > 1.0 {
    //         u = 1.0;
    //     }
    //     if v < 0.0 {
    //         v = 0.0;
    //     }
    //     if v > 1.0 {
    //         v = 1.0;
    //     }

    //     let u_img = u * self.width as f64;
    //     let v_img = (1.0 - v) * self.height as f64;
    //     let color: &VecN<u8, 3> = self.img_data.at_2d(v_img as i32, u_img as i32).unwrap();

    //     Vector3::new(color[2] as f64, color[1] as f64, color[0] as f64)
    // }

    pub fn get_color(&self, mut u: f64, mut v: f64) -> Vector3<f64>
    {
        // 在此实现双线性插值函数, 并替换掉get_color

        if u < 0.0 {
            u = 0.0;
        }
        if u > 1.0 {
            u = 1.0;
        }
        if v < 0.0 {
            v = 0.0;
        }
        if v > 1.0 {
            v = 1.0;
        }

        let u_img = u * self.width as f64;
        let v_img = (1.0 - v) * self.height as f64;
        let u_floor = u_img.floor();
        let u_ceil = u_img.ceil();
        let v_floor = v_img.floor();
        let v_ceil = v_img.ceil();
        let color00: &VecN<u8, 3> = self.img_data.at_2d(v_floor as i32, u_floor as i32).unwrap();
        let color10: &VecN<u8, 3> = self.img_data.at_2d(v_floor as i32, u_ceil as i32).unwrap();
        let color01: &VecN<u8, 3> = self.img_data.at_2d(v_ceil as i32, u_floor as i32).unwrap();
        let color11: &VecN<u8, 3> = self.img_data.at_2d(v_ceil as i32, u_ceil as i32).unwrap();

        let color0 = [
            (u_img - u_floor) * color00[2] as f64 + (u_ceil - u_img) * color10[2] as f64,
            (u_img - u_floor) * color00[1] as f64 + (u_ceil - u_img) * color10[1] as f64,
            (u_img - u_floor) * color00[0] as f64 + (u_ceil - u_img) * color10[0] as f64,
        ];
        let color1 = [
            (u_img - u_floor) * color01[2] as f64 + (u_ceil - u_img) * color11[2] as f64,
            (u_img - u_floor) * color01[1] as f64 + (u_ceil - u_img) * color11[1] as f64,
            (u_img - u_floor) * color01[0] as f64 + (u_ceil - u_img) * color11[0] as f64,
        ];

        let color = [
            (v_img - v_floor) * color0[0] + (v_ceil - v_img) * color1[0],
            (v_img - v_floor) * color0[1] + (v_ceil - v_img) * color1[1],
            (v_img - v_floor) * color0[2] + (v_ceil - v_img) * color1[2],
        ];

        Vector3::from(color)
    }
}
